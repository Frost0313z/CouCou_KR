![CouCou_KR 아이콘](windows/src-tauri/icons/128x128.png)

# CouCou_KR

**화면 맨 위에 사는 작은 친구 Mochi 가 AI 코딩 에이전트를 지켜봐 주는 [Coucou](https://github.com/Louis-CFM/coucou) 윈도우판 — 한국어 개인용 포크.**

Claude Code 권한 승인, 작업 지켜보기, 파일 끌어다 놓기, Claude 와 채팅 — 하던 일을 멈추지 않고 화면 위에서 바로 합니다.
여기에 **한국어 화면**, **API 키 없는 Claude 구독 채팅**, **PDF·한글(HWPX) 문서 읽기**, **이미지 붙여넣기**, 그리고 노치 없는 노트북을 위한 **조용한 모드**와 **트레이로 숨기기**를 더했습니다.

![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![Rust](https://img.shields.io/badge/Rust-backend-000000?logo=rust&logoColor=white)
![TypeScript 5](https://img.shields.io/badge/TypeScript-5-3178C6?logo=typescript&logoColor=white)
![Vite 6](https://img.shields.io/badge/Vite-6-646CFF?logo=vite&logoColor=white)
![Claude Code 구독 채팅](https://img.shields.io/badge/Claude%20Code-%EA%B5%AC%EB%8F%85%20%EC%B1%84%ED%8C%85-D97757?logo=anthropic&logoColor=white)
![UI 한국어](https://img.shields.io/badge/UI-%ED%95%9C%EA%B5%AD%EC%96%B4-E4002B)
![version 0.4.0-kr](https://img.shields.io/badge/version-0.4.0--kr-0A84FF)
![Code: MIT](https://img.shields.io/badge/code-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Frost0313z/CouCou_KR?style=social)

![앱을 켜면 손을 흔드는 Mochi](windows/screenshots/greeting.png)

| 권한 승인 | Claude 와 채팅 |
|---|---|
| ![Claude Code 권한 요청과 거부·허용 버튼](windows/screenshots/approval.png) | ![아일랜드에서 Claude 와 채팅](windows/screenshots/chat.png) |
| **한눈에 보기** | **파일 끌어다 놓기** |
| ![왼쪽은 선택한 연동, 오른쪽은 나머지 필](windows/screenshots/overview.png) | ![파일을 기다리며 상자로 변한 Mochi](windows/screenshots/drop.png) |

스크린샷은 원작 윈도우판 화면입니다 (영어 화면).

## 기술 스택

| 영역 | 사용한 것 |
|---|---|
| 데스크톱 앱 | [Tauri 2](https://tauri.app) — 윈도우 기본 WebView2 위에서 동작, 설치 파일 없이 exe 하나로 실행 |
| 본체 | Rust — tokio(비동기), reqwest(네트워크), serde(JSON), windows 크레이트(윈도우 API) |
| 화면 | TypeScript (프레임워크 없음) + Vite, Mochi 는 이미지 없이 Canvas 2D 코드로 그림 |
| 에이전트 연결 | `coucou-hook.exe` (Rust) — Claude Code 훅을 이름 있는 파이프(Named Pipe)로 앱에 전달 |
| AI 채팅 | **Claude Code CLI (구독)**, Anthropic · OpenAI · Google AI · OpenRouter API, Ollama · LM Studio |
| 비밀 보관 | 윈도우 자격 증명 관리자 — 키·토큰은 파일로 남기지 않음 |
| 테스트 | `node:test` (화면 399개), `cargo test` (본체 417개) — 2026-10-18 기준 |

> **비공식 포크입니다.** 원작자 Louis Raillé 와 관계가 없고, 원작자가 만들거나 보증한 것이 아닙니다.
> 직접 빌드해서 쓰는 개인용이며, 설치 파일을 배포하지 않습니다.
> 원작 설명(영어)은 [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou#readme) 에 있습니다.

---

## 원작과 다른 점

| 기능 | 내용 |
|---|---|
| 한국어 화면 | 앱의 모든 문구(747개)를 한국어로 옮겼습니다. 윈도우가 한국어면 자동으로 한국어로 나오고, 설정 → 일반 → 언어에서 고를 수도 있습니다. |
| Claude 구독으로 채팅 | API 키 없이, 이 PC에 로그인된 **Claude Code**(Pro·Max 구독)로 아일랜드 채팅에 답합니다. 채팅창 위 모델 버튼 → **Claude Code** 칩을 고르면 됩니다. |
| 파일 읽기 | 아일랜드에 끌어다 놓은 **PDF·이미지·텍스트**는 Claude Code 가 직접 읽고, **HWPX·DOCX·PPTX** 는 본문 글자를 꺼내 함께 보냅니다. |
| 이미지 붙여넣기 | 채팅 입력창에서 `Ctrl+V` 로 캡처나 복사한 이미지를 붙여 넣으면 첨부됩니다. 대화 중간에도 됩니다. |
| 조용한 모드 | 노치 없는 화면에서 아일랜드가 위쪽을 가리지 않게 합니다. 에이전트가 일하는 동안에는 아일랜드를 꺼내지 않고, **권한 요청·질문이 올 때만** 엽니다. 포인터가 떠나면 3초 뒤에 숨습니다(원래 60초). 설정 → 일반 → **조용한 모드** 에서 켭니다. |
| 트레이로 숨기기 | **`Ctrl+Alt+H`** 를 누르면 아일랜드와 바탕화면 Mochi 가 모두 사라지고, 다시 누르면 제자리로 돌아옵니다. 트레이 → 일시정지와 같은 동작이라, 숨긴 동안 권한 요청은 Claude Code 가 터미널에서 직접 묻습니다. 키는 설정 → 단축키에서 바꿀 수 있습니다. |
| 한국어 문서 | 윈도우판 설명서를 한국어로 옮겼습니다: [windows/README.ko.md](windows/README.ko.md) |

맥·아이폰·안드로이드 쪽은 원작 그대로이며, 고친 것은 **윈도우판(`windows/`)** 뿐입니다.

## 설치 (직접 빌드)

모든 변경은 **`main`** 에 있습니다. 원작 그대로의 코드는 [원작 저장소](https://github.com/Louis-CFM/coucou)에서 볼 수 있습니다.

### 필요한 것

- [Rust](https://rustup.rs)
- [Node.js 20 이상](https://nodejs.org)
- Visual Studio Build Tools 의 **"C++를 사용한 데스크톱 개발"** 과 **Windows SDK**
- Claude 구독으로 채팅하려면: [Claude Code](https://claude.com/claude-code) 설치 후 `claude` 로 한 번 로그인

### 빌드

```powershell
git clone https://github.com/Frost0313z/CouCou_KR.git
cd CouCou_KR/windows
npm install
npm run tauri dev               # 고치면 바로 반영되는 개발용 실행
npx tauri build --no-bundle     # 실행 파일 만들기 → windows\target\release\coucou.exe
```

처음 빌드는 오래 걸립니다(이 PC 에서 약 6~20분). 다음부터는 몇 분이면 됩니다.
만든 `coucou.exe` 는 설치 없이 바로 실행되니, 바탕화면에 바로가기를 만들어 두면 편합니다.
다시 빌드할 때는 Coucou 를 먼저 종료하세요. 실행 중이면 exe 파일을 덮어쓰지 못해 빌드가 실패합니다.

## 처음 설정

알림 영역(작업 표시줄 오른쪽 아래)의 Mochi 아이콘 → **설정…** 에서:

1. **Claude Code → 훅 설치…** — 이게 있어야 Claude Code 작업이 아일랜드에 나타납니다. 바뀔 내용과 백업 경로를 먼저 보여 주고, 확인을 눌러야 `~/.claude/settings.json` 에 씁니다. 기존 훅은 건드리지 않습니다.
2. **사용 중인 필** — 주 도구(VS Code 등)를 고르고, 쓰지 않는 필은 뺍니다.
3. **연동 → GitHub** — 내 PR·리뷰 요청·CI 상태를 보려면 토큰을 넣습니다. 읽기 전용(fine-grained) 토큰을 권합니다. 토큰은 윈도우 자격 증명 관리자에만 저장됩니다.
4. **단축키** — 다른 앱과 겹치는 단축키는 *사용 중* 으로 표시되니 다른 키로 바꿉니다.
5. **노치 없는 노트북이라면** — **일반 → 조용한 모드** 를 켜고, `Ctrl+Alt+D` 로 Mochi 를 바탕화면(기본 위치는 오른쪽 아래, 작업 표시줄 위)에 내보내세요. 화면 위쪽은 평소에 비어 있고, 작업 상태는 Mochi 로 보고, 승인·질문이 오면 Mochi 가 아일랜드로 날아가 열어 줍니다. 잠깐 다 치우고 싶으면 `Ctrl+Alt+H`.

## Claude 구독 채팅은 이렇게 동작합니다

채팅창에 쓴 질문을 Coucou 가 `claude -p` 로 Claude Code 에 넘기고, 답을 받아 아일랜드에 보여 줍니다. 구독을 쓰는 건 Claude Code 자신이고, Coucou 는 로그인 정보를 읽지 않습니다.

채팅 전용으로 쓰기 위해 Claude Code 를 이렇게 제한해서 부릅니다:

- **파일 읽기(Read) 하나만** 켭니다. 파일 수정·명령 실행은 막혀 있습니다.
- 읽을 수 있는 곳은 **끌어다 놓은 파일이 모이는 보관함 폴더** 뿐입니다. 그 밖의 파일 읽기는 거부됩니다.
- 연결된 MCP 도구(Canva, Gmail 등)와 사용자 설정을 불러오지 않습니다. 그래서 채팅이 가짜 작업 세션으로 뜨지 않고, 질문 하나에 드는 토큰이 약 19만에서 약 600으로 줄어듭니다 (2026-10-12 측정).

알아 둘 점:

- 질문할 때마다 Claude Code 를 새로 띄워서 첫 글자가 나오기까지 5~7초쯤 걸립니다.
- 채팅도 Claude Code 작업과 **같은 구독 사용 한도**를 씁니다. PDF 는 파일당 수만 토큰이 들 수 있습니다.
- 구독을 다른 프로그램에서 쓰는 것에 대한 Anthropic 의 약관은 분명하지 않은 부분이 있습니다. 이 기능은 **내 PC 에서 나 혼자 쓰는 용도로만** 만들었습니다.

## 알려진 한계

- 옛날 한글 파일(`.hwp`)은 읽지 못합니다. HWPX 나 PDF 로 저장해서 넘겨 주세요.
- 자동 닫힘 시간 버튼(`10s · 15s · 30s`)처럼 코드에 직접 적힌 일부 단위 표시는 영어로 남아 있습니다.
- 원작 저장소의 번역 파일(`strings.json`)이 맥 쪽 최신 문구를 아직 반영하지 않아, 화면 쪽 테스트 1개(`strings.json is what the generator makes…`)가 원래부터 실패합니다.

## 브랜치

| 브랜치 | 내용 |
|---|---|
| `main` | 위의 모든 변경 (이 브랜치로 빌드하세요) |
| `personal/claude-cli-chat` | 변경을 만들던 작업 브랜치 (`main` 에 합쳐짐) |
| `docs/windows-readme-ko` | 윈도우판 설명서 한국어 번역만 (`main` 에 합쳐짐) |

## 라이선스

- **코드:** 원작과 같은 [MIT](LICENSE) 입니다. 원작의 저작권 표시를 그대로 둡니다.
- **이름 "Coucou"·"Mochi", Mochi 캐릭터, 아이콘, 소리, 이미지:** © Louis Raillé, 모든 권리 보유 — [LICENSE-ASSETS.md](LICENSE-ASSETS.md).
  원작자의 서면 허락 없이는 이 이름과 에셋으로 앱이나 포크를 **배포할 수 없습니다.** 이 저장소는 직접 빌드해서 쓰는 개인용이고, 설치 파일이나 릴리스를 배포하지 않습니다.

원작: [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) · 만든 사람 Louis Raillé
