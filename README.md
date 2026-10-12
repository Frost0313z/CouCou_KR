# CouCou_KR

**[Coucou](https://github.com/Louis-CFM/coucou) 윈도우판을 한국어 사용자에 맞게 고친 개인용 포크입니다.**

화면 맨 위에 사는 작은 캐릭터 Mochi가 Claude Code 같은 AI 코딩 에이전트의 작업을 실시간으로 보여 주고, 권한 승인·질문 답변·채팅·파일 전달을 그 자리에서 할 수 있게 해 줍니다.

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
| 한국어 문서 | 윈도우판 설명서를 한국어로 옮겼습니다: [windows/README.ko.md](windows/README.ko.md) |

맥·아이폰·안드로이드 쪽은 원작 그대로이며, 고친 것은 **윈도우판(`windows/`)** 뿐입니다.

## 설치 (직접 빌드)

모든 변경은 **`personal/claude-cli-chat`** 브랜치에 있습니다. `main` 은 원작 그대로입니다.

### 필요한 것

- [Rust](https://rustup.rs)
- [Node.js 20 이상](https://nodejs.org)
- Visual Studio Build Tools 의 **"C++를 사용한 데스크톱 개발"** 과 **Windows SDK**
- Claude 구독으로 채팅하려면: [Claude Code](https://claude.com/claude-code) 설치 후 `claude` 로 한 번 로그인

### 빌드

```powershell
git clone https://github.com/Frost0313z/CouCou_KR.git
cd CouCou_KR
git switch personal/claude-cli-chat
cd windows
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
| `personal/claude-cli-chat` | 위의 모든 변경 (이 브랜치로 빌드하세요) |
| `docs/windows-readme-ko` | 윈도우판 설명서 한국어 번역만 (위 브랜치에 이미 포함) |
| `main` | 원작 그대로 |

## 라이선스

- **코드:** 원작과 같은 [MIT](LICENSE) 입니다. 원작의 저작권 표시를 그대로 둡니다.
- **이름 "Coucou"·"Mochi", Mochi 캐릭터, 아이콘, 소리, 이미지:** © Louis Raillé, 모든 권리 보유 — [LICENSE-ASSETS.md](LICENSE-ASSETS.md).
  원작자의 서면 허락 없이는 이 이름과 에셋으로 앱이나 포크를 **배포할 수 없습니다.** 이 저장소는 직접 빌드해서 쓰는 개인용이고, 설치 파일이나 릴리스를 배포하지 않습니다.

원작: [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) · 만든 사람 Louis Raillé
