// 개인용 개조: Claude 구독(Pro·Max)으로 채팅하기.
//
// API 키 대신, 이 PC에 설치되어 로그인된 Claude Code CLI 를 `claude -p` 로 부른다.
// 구독을 쓰는 건 Claude Code 자신이고, Coucou 는 로그인 정보를 읽지 않는다.
// 질문은 stdin 으로 넘기고, stream-json 출력의 text_delta 를 `chat-delta` 로 흘린다.
//
// 채팅 전용이라 Claude Code 가 아무것도 실행하지 못하게 막는다:
//   --tools Read          읽기 도구 하나만 켬 (파일 수정·명령 실행 등은 전부 끔)
//   작업 폴더 = 보관함    Read 는 작업 폴더 안만 허락 없이 읽는다. 밖은 -p 모드에서 거부된다 (2026-10-12 시험).
//   --strict-mcp-config   MCP 서버를 하나도 불러오지 않음
//   ENABLE_CLAUDEAI_MCP_SERVERS=false   claude.ai 연결 도구(Canva, Gmail…)도 끔
//   --setting-sources ""  사용자 설정을 읽지 않음 → 훅이 돌지 않아 가짜 세션이 안 생김
// 이 옵션이 없으면 첫 질문에 약 19만 토큰이 실려 구독 한도를 빨리 먹는다 (2026-10-12 실측: 608 토큰으로 줄어듦).
//
// 끌어다 놓은 파일: PDF·이미지·텍스트는 Claude Code 가 Read 로 직접 읽고,
// HWPX·DOCX·PPTX 는 Read 가 못 읽는 zip 이라 윈도우 기본 tar 로 본문을 꺼내 질문에 붙인다.
//
// 대화 기록은 Claude Code 세션으로 남기지 않고(--no-session-persistence), 지난 대화를
// 매번 질문 앞에 붙여 보낸다.
// ponytail: 대화가 길어지면 매번 전체를 다시 보낸다. 느려지면 --session-id/--resume 으로 바꿀 것.

use std::process::Stdio;
use std::time::Duration;

use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

use crate::chat::{self, Chat, ChatContext, ChatReply, ModelInfo};
use crate::island::WINDOW_LABEL;
use crate::platform;

pub const ID: &str = "claude-cli";
pub const DEFAULT_MODEL: &str = "sonnet";

/// 한 번의 답을 기다리는 최대 시간.
const TIMEOUT: Duration = Duration::from_secs(300);
/// 화면에 새 글자를 알리는 최소 간격 (local_chat 과 같음).
const DELTA_INTERVAL: Duration = Duration::from_millis(1000 / 15);

/// 모델 고르기 목록: Claude Code 가 받는 별칭 그대로.
pub fn models() -> Vec<ModelInfo> {
    [("sonnet", "Sonnet"), ("opus", "Opus"), ("haiku", "Haiku")]
        .into_iter()
        .map(|(id, label)| ModelInfo { id: id.into(), label: label.into() })
        .collect()
}

fn find_claude() -> Option<std::path::PathBuf> {
    platform::find_on_path("claude").or_else(|| {
        let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
        let exe = std::path::PathBuf::from(home).join(".local").join("bin").join("claude.exe");
        exe.is_file().then_some(exe)
    })
}

/// 지난 대화와 새 질문을 한 덩어리 글로.
fn prompt_text(history: &[Value], question: &str) -> String {
    if history.is_empty() {
        return question.to_string();
    }
    let mut out = String::from("The conversation so far:\n\n");
    for turn in history {
        let who = if turn["role"] == "assistant" { "Assistant" } else { "User" };
        out.push_str(&format!("{who}: {}\n\n", turn["content"].as_str().unwrap_or("")));
    }
    out.push_str(&format!("Answer the user's new message:\n\n{question}"));
    out
}

/// 한 번에 붙이는 문서 본문의 최대 글자 수 (local_chat 과 같음).
const MAX_INLINE_CHARS: usize = 24_000;

/// zip 으로 된 문서 안에서 본문이 든 XML.
fn office_parts(path: &std::path::Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_lowercase().as_str() {
        "hwpx" => Some("Contents/section*.xml"),
        "docx" => Some("word/document.xml"),
        "pptx" => Some("ppt/slides/slide*.xml"),
        _ => None,
    }
}

/// 문단 끝은 줄바꿈, 나머지 태그는 지우고, 기본 엔티티만 되돌린다.
/// ponytail: 표·각주 모양은 버린다. 서식이 필요해지면 제대로 된 XML 파서로.
fn xml_text(xml: &str) -> String {
    let mut out = String::with_capacity(xml.len() / 4);
    let mut rest = xml;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        let Some(len) = rest[start..].find('>') else { break };
        let tag = &rest[start..start + len + 1];
        if tag.starts_with("</") && tag.ends_with(":p>") {
            out.push('\n');
        }
        rest = &rest[start + len + 1..];
    }
    let text = out.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&");
    text.lines().map(str::trim_end).filter(|l| !l.trim().is_empty()).collect::<Vec<_>>().join("\n")
}

/// HWPX·DOCX·PPTX 의 본문 글자. 윈도우 10/11 에 기본으로 있는 tar(bsdtar)가 zip 을 연다.
fn office_text(path: &std::path::Path) -> Option<String> {
    let parts = office_parts(path)?;
    let mut cmd = std::process::Command::new("tar");
    cmd.arg("-xOf").arg(path).arg(parts).stdin(Stdio::null()).stderr(Stdio::null());
    platform::no_console(&mut cmd);
    let out = cmd.output().ok()?;
    let text = xml_text(&String::from_utf8_lossy(&out.stdout));
    (!text.is_empty()).then_some(text)
}

/// 이번 질문: 파일은 첫 질문에만 붙는다 (다른 제공자와 같음).
fn question_text(first: bool, context: Option<&ChatContext>, query: &str) -> String {
    let Some(ChatContext::File { name, path }) = context.filter(|_| first) else {
        return crate::local_chat::user_text(first, context, query);
    };
    let path = std::path::Path::new(path);
    if let Some(text) = office_text(path) {
        let mut body: String = text.chars().take(MAX_INLINE_CHARS).collect();
        if body.len() < text.len() {
            body.push_str("\n[truncated]");
        }
        return format!("File: {name}\nFile contents:\n{body}\n\n{query}");
    }
    let file = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_else(|| name.clone());
    format!("The user attached the file \"{file}\" (it is in your working directory). Read it with the Read tool before answering.\n\n{query}")
}

/// stream-json 한 줄 → 글자 조각, 최종 답, 또는 오류.
#[derive(Debug, PartialEq)]
enum Line {
    /// 새 응답 메시지 시작 (도구를 쓴 뒤 다시 말할 때): 보이던 글은 지운다.
    NewMessage,
    Delta(String),
    Done(String),
    Failed(String),
    Other,
}

fn parse_line(line: &str) -> Line {
    let Ok(v) = serde_json::from_str::<Value>(line) else { return Line::Other };
    match v["type"].as_str() {
        Some("stream_event") if v.pointer("/event/type") == Some(&json!("message_start")) => Line::NewMessage,
        Some("stream_event") if v.pointer("/event/delta/type") == Some(&json!("text_delta")) => {
            Line::Delta(v.pointer("/event/delta/text").and_then(Value::as_str).unwrap_or("").to_string())
        }
        Some("result") => {
            let text = v["result"].as_str().unwrap_or("").to_string();
            if v["is_error"].as_bool() == Some(true) || v["subtype"].as_str().is_some_and(|s| s != "success") {
                Line::Failed(if text.is_empty() { "Claude Code returned an error.".into() } else { text })
            } else {
                Line::Done(text)
            }
        }
        _ => Line::Other,
    }
}

pub async fn send(
    app: &AppHandle,
    chat: &Chat,
    model: &str,
    query: String,
    context: Option<ChatContext>,
) -> Result<ChatReply, String> {
    let exe = find_claude().ok_or("Claude Code not found. Install it and sign in with `claude` first.")?;
    let turn = chat.begin(ID);
    let question = question_text(turn.first, context.as_ref(), &query);
    let inbox = crate::files::inbox_dir();
    let workdir = if inbox.is_dir() { inbox } else { std::env::temp_dir() };
    let model = if model.is_empty() { DEFAULT_MODEL } else { model };

    let mut std_cmd = std::process::Command::new(&exe);
    std_cmd
        .args(["-p", "--output-format", "stream-json", "--verbose", "--include-partial-messages"])
        .args(["--tools", "Read", "--strict-mcp-config", "--setting-sources", "", "--no-session-persistence"])
        .args(["--model", model, "--system-prompt", &chat::system_prompt(false)])
        .env("ENABLE_CLAUDEAI_MCP_SERVERS", "false")
        // 보관함에서 돌린다: Read 가 닿는 곳이 끌어다 놓은 파일뿐이고, 프로젝트 CLAUDE.md 도 끼어들지 않는다.
        .current_dir(workdir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    platform::no_console(&mut std_cmd);
    let mut child = tokio::process::Command::from(std_cmd)
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| format!("Could not start Claude Code: {e}"))?;

    let mut stdin = child.stdin.take().ok_or("Claude Code has no input.")?;
    stdin.write_all(prompt_text(&turn.history, &question).as_bytes()).await.map_err(|e| e.to_string())?;
    drop(stdin);

    let stdout = child.stdout.take().ok_or("Claude Code has no output.")?;
    let mut stderr = child.stderr.take().ok_or("Claude Code has no output.")?;
    let read = async {
        let mut lines = BufReader::new(stdout).lines();
        let mut shown = String::new();
        let mut last = tokio::time::Instant::now() - DELTA_INTERVAL;
        while let Some(line) = lines.next_line().await.map_err(|e| e.to_string())? {
            match parse_line(&line) {
                Line::NewMessage => shown.clear(),
                Line::Delta(d) => {
                    shown.push_str(&d);
                    if last.elapsed() >= DELTA_INTERVAL {
                        last = tokio::time::Instant::now();
                        let _ = app.emit_to(WINDOW_LABEL, "chat-delta", shown.clone());
                    }
                }
                Line::Done(text) => return Ok(if text.is_empty() { shown } else { text }),
                Line::Failed(msg) => return Err(msg),
                Line::Other => {}
            }
        }
        // 결과 줄 없이 끝남: 로그인 안 됨 같은 경우라 stderr 를 보여 준다.
        let mut err = String::new();
        let _ = (&mut stderr).take(4096).read_to_string(&mut err).await;
        Err(if err.trim().is_empty() { "Claude Code stopped without an answer.".into() } else { err.trim().to_string() })
    };
    let answer = tokio::time::timeout(TIMEOUT, read).await.map_err(|_| "Claude Code took too long.".to_string())??;
    let _ = child.wait().await;

    let answer = answer.trim().to_string();
    if answer.is_empty() {
        return Err("No response text.".into());
    }
    let plain = chat::plain_question(turn.first, context.as_ref(), &query);
    chat.commit(&turn, json!({ "role": "user", "content": question }), json!({ "role": "assistant", "content": answer }), &plain, &answer);
    Ok(ChatReply { text: answer })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_json_lines() {
        let d = r#"{"type":"stream_event","event":{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hel"}}}"#;
        assert_eq!(parse_line(d), Line::Delta("Hel".into()));
        let ok = r#"{"type":"result","subtype":"success","is_error":false,"result":"Hello"}"#;
        assert_eq!(parse_line(ok), Line::Done("Hello".into()));
        let bad = r#"{"type":"result","subtype":"success","is_error":true,"result":"Not logged in"}"#;
        assert_eq!(parse_line(bad), Line::Failed("Not logged in".into()));
        let bad = r#"{"type":"result","subtype":"error_max_turns","is_error":false}"#;
        assert!(matches!(parse_line(bad), Line::Failed(_)));
        assert_eq!(parse_line(r#"{"type":"stream_event","event":{"type":"message_start","message":{}}}"#), Line::NewMessage);
        assert_eq!(parse_line(r#"{"type":"system","subtype":"init"}"#), Line::Other);
        assert_eq!(parse_line("not json"), Line::Other);
    }

    #[test]
    fn office_xml_becomes_paragraphs() {
        let xml = r#"<?xml version="1.0"?><hs:sec><hp:p><hp:run><hp:t>첫 문단</hp:t></hp:run></hp:p><hp:p><hp:t>A &amp; B &lt;3</hp:t></hp:p><hp:p></hp:p></hs:sec>"#;
        assert_eq!(xml_text(xml), "첫 문단\nA & B <3");
        let docx = r#"<w:document><w:body><w:p><w:r><w:t>Hello</w:t></w:r><w:r><w:t xml:space="preserve"> world</w:t></w:r></w:p></w:body></w:document>"#;
        assert_eq!(xml_text(docx), "Hello world");
        assert_eq!(office_parts(std::path::Path::new("a/B.HWPX")), Some("Contents/section*.xml"));
        assert_eq!(office_parts(std::path::Path::new("a.pdf")), None);
    }

    #[test]
    fn a_pdf_is_left_to_the_read_tool() {
        let file = ChatContext::File { name: "cv.pdf".into(), path: "C:/inbox/cv (2).pdf".into() };
        let q = question_text(true, Some(&file), "summarise");
        assert!(q.contains("\"cv (2).pdf\""), "{q}");
        assert!(q.contains("Read tool"));
        assert_eq!(question_text(false, Some(&file), "again"), "again");
    }

    #[test]
    fn history_goes_before_the_new_question() {
        assert_eq!(prompt_text(&[], "hi"), "hi");
        let h = [json!({"role":"user","content":"a"}), json!({"role":"assistant","content":"b"})];
        let p = prompt_text(&h, "c");
        assert!(p.contains("User: a\n\nAssistant: b\n\n"));
        assert!(p.ends_with("c"));
    }
}

/// 실제 문서로 본문 추출 확인: COUCOU_DOC=<파일> cargo test real_document -- --ignored --nocapture
#[cfg(test)]
#[test]
#[ignore]
fn real_document() {
    let path = std::env::var("COUCOU_DOC").expect("COUCOU_DOC");
    let text = office_text(std::path::Path::new(&path)).expect("no text");
    println!("{} chars\n{}", text.chars().count(), text.chars().take(400).collect::<String>());
}
