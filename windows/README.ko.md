![Coucou 아이콘](src-tauri/icons/128x128.png)

# 윈도우용 Coucou

**PC에는 노치가 없어서, Mochi는 대신 화면 맨 위에 삽니다.**

Claude Code 권한을 승인하고, 세션이 일하는 모습을 지켜보고, 파일을 떨어뜨리고, Claude와 대화하고, 쓰는 서비스를 살펴보세요. 하던 일에서 손을 떼지 않고요.

![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![Rust](https://img.shields.io/badge/Rust-backend-000?logo=rust)
![Code: MIT](https://img.shields.io/badge/code-MIT-green)

> 이 문서는 [README.md](README.md)(영어 원문)를 옮긴 것입니다. 내용이 다르면 원문이 맞습니다.

![앱을 켜면 손을 흔드는 Mochi](screenshots/greeting.png)

---

## 설치

항상 최신 버전인 **[Coucou-Windows.msi](https://github.com/Louis-CFM/coucou/releases/download/windows-latest/Coucou-Windows.msi)**
(Windows Installer) 또는 **[Coucou-Windows-setup.exe](https://github.com/Louis-CFM/coucou/releases/download/windows-latest/Coucou-Windows-setup.exe)**
를 내려받아 실행하세요. .exe 는 현재 사용자에게만 설치되며 관리자 권한을 묻지 않습니다. .msi 는 관리자 권한을 요구할 수 있습니다.

**처음 실행하면 Windows 가 경고를 띄웁니다. 정상입니다.** 설치 파일에 아직 코드 서명이 없어서 SmartScreen(윈도우의 앱 평판 검사)이 게시자를 모르기 때문입니다.

1. *게시자: 알 수 없는 게시자* 라고 적힌 **"Windows의 PC 보호"** 화면이 뜹니다.
2. **추가 정보**를 누르면 **실행** 버튼이 나타납니다.
3. **실행**을 누르면 설치가 평소대로 시작됩니다.

유료 인증서로 서명하지 않았기 때문일 뿐입니다. Coucou 는 오픈소스이고, Microsoft Defender 검사에서도 깨끗하게 나옵니다.

Microsoft Defender 가 한때 설치 파일을 잘못 잡아낸 적이 있습니다(`Trojan:Win32/Wacatac.H!ml`,
머신러닝 오탐). Microsoft 가 검토한 뒤 탐지를 없앴습니다.
그래도 Defender 가 막는다면 정의를 업데이트(PowerShell 에서 `Update-MpSignature`)한 뒤 다시 시도하세요.

[직접 빌드](#직접-빌드하기)할 수도 있습니다.

## 쓰는 법

![작은 아일랜드. 연동 필이 작은 Mochi 모양으로 붙어 있다](screenshots/compact.png)
![전체 화면: 왼쪽은 선택한 연동, 오른쪽은 나머지 필](screenshots/overview.png)
![Claude Code 권한 요청과 거부·허용 버튼](screenshots/approval.png)
![아일랜드에서 Claude와 대화](screenshots/chat.png)
![파일을 기다리며 상자로 변한 Mochi](screenshots/drop.png)

| 이렇게 하면 | 이렇게 됩니다 |
|---|---|
| 마우스를 화면 맨 위 가운데로 가져간다 | Mochi 가 빼꼼 나옵니다 |
| 작은 아일랜드를 클릭한다 | 열립니다. **Settings → General → Open on hover** 를 켜면 포인터를 올리기만 해도 열리고, 포인터가 떠나면 곧 다시 접힙니다(안쪽을 클릭하면 열린 채로 있습니다) |
| Mochi 를 클릭한다 | 짜증을 냅니다. 세 번 연달아 누르면 어지러워합니다 |
| Mochi 위에 포인터를 2초 올려 둔다 | 하트 |
| Mochi 를 오른쪽 클릭한다 | 옷장: 옷 위에 포인터를 올리면 입어 보고, 클릭하면 그대로 입습니다. **Auto** 는 계절에 맞춰 입힙니다(10월엔 마녀 모자, 12월엔 산타 모자…) |
| Mochi 를 아일랜드 밖으로 끌어낸다 | 바탕화면으로 옮겨 가 옷을 입은 채 커서를 바라보며 지냅니다. 아일랜드 위에 다시 놓으면 집으로 돌아옵니다 |
| 바탕화면에서 Mochi 를 클릭 / 오른쪽 클릭 / 더블클릭 | 쿡 찌르기 / 옷장 / 집으로 날아감. 끌어서 옮길 수 있습니다 |
| 파일을 아일랜드로 끌어온다 | Mochi 가 상자로 변해 파일을 삼키고, 그 파일에 대한 질문에 답해 주겠다고 합니다 |
| 세션 진행 표시줄의 파일을 클릭한다 | 아일랜드 안에 변경 내용(diff)이 열립니다. ↗ 는 VS Code 에서 파일을 열고, ‹ 나 `Esc` 는 뒤로 갑니다 |
| `Esc` | 아일랜드를 닫습니다 |
| Mochi 소리와 같은 이름의 파일(`finish.wav`, `approval.mp3`, `greet.m4a`…)을 소리 폴더에 넣는다 | **Settings → General → Reload sounds** 뒤에 그 소리를 대신합니다. **Open sounds folder** 가 폴더를 보여 줍니다: 리눅스는 `~/.config/coucou/sounds`, 윈도우는 `%APPDATA%\Coucou\sounds` |
| 알림 영역(트레이) 아이콘 | Open, Weekly recap, Wardrobe…, Settings…, Pause, Quit |
| `Ctrl+Alt+Space` | 어느 앱에서든 채팅을 엽니다 |
| `Ctrl+Alt+A` | 기다리는 권한 요청이나 질문으로 바로 갑니다 |
| `Ctrl+Alt+T` | 세션이 돌고 있는 창을 앞으로 가져옵니다("Open terminal") |
| `Ctrl+Alt+→` / `Ctrl+Alt+←` | 다음 / 이전 필 |
| `Ctrl+Alt+S` | Mochi 소리 끄기 / 켜기 |
| `Ctrl+Alt+G` | 옷장을 엽니다 |
| `Ctrl+Alt+D` | Mochi 를 바탕화면으로 보내거나 집으로 데려옵니다 |
| `Ctrl+Alt+N` | 아일랜드를 열고 닫습니다(직접 켜기 전까지는 꺼져 있음) |
| 열린 아일랜드에서: `Ctrl+→` `Ctrl+←`, `Ctrl+1`–`Ctrl+9` | 필 전환 |
| 열린 아일랜드에서: `Ctrl+↓` `Ctrl+↑`, `Ctrl+O` | 열린 GitHub 목록을 오르내리고, 강조된 줄을 엽니다 |
| 열린 아일랜드에서: `Ctrl+E` | 가장 최근 수정의 diff 를 열거나 닫습니다 |
| 열린 아일랜드에서: `Ctrl+Enter`, `Ctrl+K` | 보내기, 새 대화 시작 |
| 열린 아일랜드에서: `Ctrl+,`, `Ctrl+P` | 설정, 아일랜드 열어 두기 |

### 키보드 단축키

모든 전역 단축키는 **Settings… → Shortcuts** 에서 바꾸거나 끌 수 있습니다.
단축키를 클릭하고 새 키를 누르면 됩니다. 다른 앱이 이미 쓰고 있는 조합에는
*In use*, Coucou 단축키 두 개가 같은 키를 쓰면 *Used twice* 가 붙습니다.
새 단축키를 녹화하는 동안에는 Coucou 가 자기 단축키를 잠시 놓아서 키가 녹화기에 그대로 전달됩니다.

기본값은 맥의 `⌃⌥` 조합과 다릅니다. 윈도우에서 `Ctrl+Alt` 는 `AltGr` 과 같아서,
전역 `Ctrl+Alt+E` 를 잡아 두면 프랑스어·독일어 자판에서 치는 `€` 가 전부 먹혀 버립니다.
기본값은 프랑스어, 독일어, 스페인어, 이탈리아어, 포르투갈어, 브라질(ABNT2) 자판의
AltGr 층과 겹치지 않는지 확인했습니다. 그래서 필 전환은 `[` `]` 대신 화살표를, 음소거는
`M` 대신 `S` 를 씁니다(독일어에서 `AltGr+M` 은 `µ`). 여기에 더해 Coucou 는 설치된 자판에서
각 `Ctrl+Alt` 조합이 어떤 글자를 치는지 윈도우에 물어보고, 글자를 치는 조합은 등록하지 않은 채
설정에 *Types “ą”* 처럼 표시합니다. 예를 들어 폴란드어는 `AltGr+A` 에 `ą`, `AltGr+S` 에 `ś` 가 있습니다.
녹화기도 그런 조합은 받지 않습니다.

오래된 Intel 그래픽 드라이버는 `Ctrl+Alt+←` / `→` 로 화면을 회전시킵니다. 아직 그렇다면 이 두 개가
*In use* 로 나옵니다. 리눅스의 Xfce 와 MATE 는 `Ctrl+Alt+D` 를 바탕화면 보기로 쓰므로, 거기서는
다른 키를 줄 때까지 **Send Mochi to the desktop** 이 *In use* 로 나옵니다.

나머지는 알아서 일어납니다. Claude Code 가 권한을 요청하면 아일랜드가 **Deny / Allow** 와 함께 열리고,
Claude Code 가 질문하면 고를 수 있는 선택지가 나오고, 세션이 끝나면 무엇을 했는지 보여 주고,
연동한 서비스들은 Mochi 옆의 색깔 필에 자리 잡습니다.

권한 카드나 질문은 답할 때까지 남아 있습니다. 마우스가 떠나도 접히지 않고, 아일랜드가 이미 열려 있거나
다른 필이 앞에 있어도 떠오르며, 답하고 나면 보고 있던 필로 돌아갑니다. 나중으로 미루려면 카드 모서리의
**⌃**(또는 아일랜드 안에서 `Esc`)로 접으세요. 아일랜드가 작은 크기로 줄어 화면에 남고, 아무것도 답하지 않으며,
다시 열면 카드가 보입니다. **Open terminal** 은 세션이 돌고 있는 창을 앞으로 가져옵니다.

Mochi 가 바탕화면에 살고 있을 때는, 권한 요청이나 질문이 오면 아일랜드로 날아갔다가 답하고 나면 제자리로
돌아옵니다. 작업이 끝나면 작게 점프하고, 2분 동안 아무 일도 없고 커서가 멀리 있으면 꾸벅꾸벅 좁니다.
자는 동안에는 자원을 거의 쓰지 않습니다(커서 위치를 묻지 않고, 초당 몇 프레임만 그립니다). 앱을 다시 켜도
자기 자리를 기억합니다. 그 자리가 지금은 연결되지 않은 모니터였다면 아일랜드에 머뭅니다.

**실시간 diff.** Claude 가 수정하는 파일(Edit, MultiEdit, Write)마다 세션 진행 표시줄에 **+N −M** 줄 수와
함께 나타나고, 클릭하면 diff 가 보입니다. 제한은 맥과 같습니다. 200 KB 나 4 000 줄을 넘으면 줄 수만 남기고,
세션당 diff 는 최대 50개이며, 마지막 수정 한 시간 뒤나 세션이 끝날 때 잊습니다. Claude 가 일을 마치면
카드에는 다음 프롬프트 전까지 마지막 답변의 첫 문단이 한 줄로 남아 있습니다.

## 내 필

**Settings… → Active pills** 에 맥 앱과 같은 목록에서 고른, 내가 쓰는 도구들이 나옵니다.
**주 도구**(VS Code, Cursor, Codex, Antigravity 중 하나)를 고르세요. 주 도구는 항상 있고 자리를 차지하지 않습니다.
그 밖에 최대 네 개를 더 고를 수 있습니다: 에이전트(Gemini CLI, Copilot CLI, Muse Code, OpenCode, Amp, Hermes,
Claude Desktop), 채팅 제공자(Anthropic, Google AI, OpenAI, Ollama, LM Studio), 그리고 **Integrations** 아래의 서비스들.
훅(hook, 다른 프로그램이 일이 생길 때마다 불러 주는 연결 고리)으로 동작하는 필은 훅이 설치됐는지만 알려 주고
키를 묻지 않습니다. 로컬 모델 서버 필은 채팅이 연결됐는지 알려 줍니다. 고르지 않은 필의 세션도
돌아가는 동안에는 나타납니다.

## Claude Code

![설정 창](screenshots/settings.png)

**Settings… → Claude Code → Install hooks…** 를 여세요. `%USERPROFILE%\.claude\settings.json` 에서
무엇이 바뀌는지 정확한 diff 와, 미리 떠 둘 날짜별 백업 경로가 보입니다. 클릭하기 전에는 아무것도 쓰지 않습니다.
내가 직접 넣은 훅은 건드리지 않고, 제거할 때도 Coucou 의 항목만 지웁니다.

중계 프로그램은 작은 실행 파일 `coucou-hook.exe` 로, 앱이 켜질 때 `%LOCALAPPDATA%\Coucou\bin\` 에 복사됩니다.
Coucou 에 닿는 데 300 ms 만 주어지고, 앱이 꺼져 있거나 느리거나 죽어 있으면 깔끔하게 끝납니다.
**Coucou 때문에 Claude Code 세션이 막히거나 느려지는 일은 없습니다.** 권한 요청에 제때 아무도 답하지 않으면
Coucou 는 조용히 있고, Claude Code 가 평소처럼 터미널에서 묻습니다.

어느 터미널에서든 동작합니다 — Windows Terminal, PowerShell, VS Code, Git Bash.

### 요금제 사용량

맥처럼 아일랜드 머리 부분에 요금제 한도를 보여 줄 수 있습니다. Claude 의 5시간·주간 한도를 나타내는 작은 필
("Claude 73%", 50 % 미만은 초록, 80 % 까지 주황, 그 위는 빨강)과 Codex 용 필이 하나 더 있습니다. 클릭하면 자세한
내용과 초기화 시각이 나옵니다. 둘 다 기본으로 꺼져 있으니 **Settings… → Plan usage** 에서 켜세요.

- **Claude**(Pro, Max 요금제): 숫자는 Claude Code 의 상태 줄(status line)에서 가져옵니다. **Show in notch** 는
  먼저 `%USERPROFILE%\.claude\settings.json` 의 `statusLine` 변경 diff 를 보여 주고, 날짜별 백업을 떠 둔 뒤,
  클릭해야만 씁니다(훅과 같은 방식). 상태 줄은 `coucou-hook --statusline` 으로 바뀌는데, 이것은 한도 숫자만 넘기고
  (최대 300 ms) 원래 쓰던 상태 줄을 — 중계 프로그램 옆 `statusline-previous.json` 에 보관 — 같은 입력으로 실행해
  그 출력을 그대로 찍습니다. 윈도우에서는 Claude Code 처럼 그것을 Git Bash 로 실행합니다(`CLAUDE_CODE_GIT_BASH_PATH`,
  다음은 `PATH` 의 `git.exe` 가 속한 Git for Windows, 다음은 흔한 설치 폴더 순). 10초와 출력 64 KB 가 주어집니다.
  **Uninstall relay** 는 원래 상태 줄을 되돌려 놓습니다. 숫자는 Claude Code 가 답할 때 같이 옵니다. Claude Code
  명령줄 도구에서만 오기 때문에, Claude 데스크톱 앱은 상태 줄을 실행하지 않습니다. 데스크톱 앱만 쓴다면 Claude
  게이지는 "—" 로 남습니다.
- **Codex**: 아무것도 설치하지 않습니다. 필이 보일 때(또는 클릭할 때, 최대 1분에 한 번) Coucou 가
  `codex app-server` 를 띄워 Codex 의 `/status` 처럼 `account/rateLimits/read` 를 물어본 뒤 끕니다(최대 15초,
  일시정지 중에는 하지 않음). Codex 는 ChatGPT 로 로그인돼 있어야 합니다.

## 주간 결산

월요일 오전 8시부터, Coucou 가 처음 켜지거나 에이전트가 일을 시작하거나 아일랜드를 깨울 때, 지난 한 주를 정리한
카드가 나옵니다: 쓴 시간, 세션 수, 바뀐 파일과 줄 수, 실행한 명령, 권한 요청과 질문, 가장 많이 쓴 에이전트,
가장 많이 작업한 프로젝트, 가장 바빴던 날, 가장 긴 세션. **트레이 → Weekly recap** 으로 언제든 열 수 있습니다.

**Share image** 는 Mochi 가 들어간 1080 × 1920 그림으로 만듭니다. **Save image** 는 사진 폴더(없으면 다운로드 폴더)에
`Coucou weekly recap YYYY-MM-DD.png` 로 저장하며, 있는 파일을 덮어쓰지 않습니다. **Copy** 는 클립보드에 넣습니다.
**Hide project names** 를 켜면 그림에서 프로젝트 이름을 뺍니다.

기록은 로그 옆의 `recap.json` 으로, 최근 12주만 남깁니다. 숫자, 에이전트, 프로젝트 폴더 이름만 담고 —
명령, 파일 경로, 파일 내용, 프롬프트는 절대 담지 않습니다. 내 컴퓨터 밖으로 나가지 않습니다.
**Settings → General → Weekly recap** 에서 끄거나 지울 수 있습니다.

## 언어

Coucou 는 맥 앱과 같은 열 개 언어를 씁니다: English, 简体中文, हिन्दी, Español, العربية, Français, বাংলা,
Português (Brasil), Русский, Bahasa Indonesia. (한국어는 아직 없습니다.) **Settings… → General → Language** 에서
하나를 고르세요. 기본값인 **System** 은 시스템 언어가 이 중 하나면 그 언어를, 아니면 영어를 씁니다. 아일랜드,
설정 창, 트레이 메뉴가 그 자리에서 바뀝니다. 다시 시작하지 않고, 아일랜드의 세션·진행 단계·대화도 그대로입니다.

아랍어에서는 아일랜드 카드가 오른쪽에서 왼쪽으로 읽힙니다. Mochi, 필, 머리 부분은 제자리에 있고, 명령·코드·파일
경로는 왼쪽에서 오른쪽 그대로입니다. 세션 진행 표시줄에 이미 적힌 단계는 맥처럼 적혔을 때의 언어를 유지합니다.

번역은 맥의 것을 그대로 씁니다(`NotchBuddy/Resources/Localizable.xcstrings` 를 `node scripts/gen-strings.mjs` 가
`src/i18n/strings.json` 으로 바꿈). 윈도우·리눅스에만 나오는 문구는 `src/i18n/extra.json` 에 있습니다. 둘 다 영어 문장을
키로 쓰며, 어떤 언어에 빠진 문구는 영어로 나옵니다. Rust 쪽(트레이, 오류)도 같은 두 파일을 품고 있습니다.

## 채팅과 키

**Settings… → Claude** 에 Anthropic API 키를 넣습니다. 키는 **Windows 자격 증명 관리자**에 보관되며, 디스크에도
화면에도 절대 나오지 않습니다. 아일랜드는 키가 있는지만 물어볼 수 있습니다. 모든 연동 키도 마찬가지입니다.

채팅은 **Google AI(Gemini)**, **OpenAI**, **OpenRouter** 와도 대화합니다. **Settings… → Chat providers** 에 키를
넣은 뒤, 맥처럼 채팅 입력창 위의 모델 이름을 클릭해 제공자와 모델을 바꾸세요. 모델 목록은 그 제공자를 고르고
키가 있을 때만 가져옵니다. 대화 도중 바꾸면 대화가 일반 텍스트로 넘어가므로, 한 제공자 형식의 데이터가 다른
제공자로 보내지는 일은 없습니다. 이 제공자들은 웹 검색도 도구도 쓰지 않습니다 — 답만 하고, 내 PC 에서 무언가를
실행하지는 않습니다.

**로컬 모델**: **Settings… → Local models** 에서 **Ollama** 나 **LM Studio**(주소를 비워 두면 이 PC 의 기본 주소,
Ollama 의 `OLLAMA_HOST` 도 따릅니다), 또는 OpenAI API 를 말하는 아무 서버(vLLM, llama.cpp…)에 연결합니다. 키는 선택이며
자격 증명 저장소에 보관됩니다. 답은 쓰이는 대로 흘러나오고, 추론 모델의 `<think>` 블록은 숨겨집니다. 떨어뜨린 텍스트
파일은 내용째 함께 갑니다(최대 24 000자). 이미지와 PDF 는 이름만 갑니다. 설정 화면이 주소가 이 PC 인지 알려 주고
(그렇다면 아무것도 밖으로 나가지 않습니다), 키가 암호화되지 않은 `http://` 로 다른 컴퓨터에 갈 것 같으면 먼저 경고합니다.

모든 제공자의 답은 **마크다운**으로 보입니다: 제목, 목록, 굵게, 인라인 코드, 인용, 복사 버튼이 달린 코드 블록.
텍스트 노드로만 만들고 HTML 로 해석하지 않으며, `http`/`https` 링크만 열립니다. 계정에 이름이 있으면 Mochi 가
이름으로 인사합니다(윈도우 표시 이름, 리눅스는 GECOS 전체 이름. 로그인 아이디만 있으면 쓰지 않습니다).

Claude 채팅을 Anthropic 호환 게이트웨이로 보내려면 `COUCOU_ANTHROPIC_BASE_URL` 을 설정하세요(예:
`https://gateway.example.com`. 뒤에 `/v1/messages` 가 붙습니다). `https://` 여야 하고, `http://` 는 이 PC 일 때만
됩니다. Claude Code 자체의 `ANTHROPIC_BASE_URL` 은 일부러 무시합니다. 키는 Coucou 에 보내라고 한 곳으로만 갑니다.
게이트웨이 주소는 로그에 한 번 적히고, 키는 절대 적히지 않습니다.

원격 측정(텔레메트리)은 없습니다. Coucou 가 보내는 네트워크 요청은 직접 설정한 서비스로 가는 것뿐입니다.

## GitHub

**Settings… → Integrations → GitHub** 에 토큰을 넣으면 — `repo` 권한의 클래식 토큰, 또는 Pull requests·Commit statuses·
Actions 읽기 권한이 있는 세분화(fine-grained) 토큰 — GitHub 필이 보여 줍니다:

- **My PRs**: 내가 연 풀 리퀘스트와 그 CI(자동 빌드·테스트) 상태.
- **To review**: 내 리뷰를 기다리는 풀 리퀘스트.
- **Default branch CI**: 최근 푸시한 저장소 10개의 기본 브랜치 CI.
- 카드 머리 부분에 별 수와 **최근 7일 기여**. 클릭하면 지난 23주가 나오고, 날짜에 포인터를 올리거나 클릭하면 그날 수가 나옵니다.

줄을 클릭하면 목록이, 항목을 클릭하면 github.com 에서 열립니다. 내 풀 리퀘스트의 CI 가 빨강이나 초록으로 바뀔 때
(두 번 확인하는 사이에 끝난 빠른 실행 포함), 기본 브랜치가 깨질 때, 누군가 내 리뷰를 요청할 때 필에 배지와 소리가
납니다. 풀 리퀘스트는 5분마다, CI 가 도는 동안은 1분마다, 1분보다 오래된 데이터로 카드를 열면 바로 확인합니다.
기여는 30분마다 확인합니다. 필이 꺼져 있거나 Coucou 가 일시정지 중이면 아무것도 가져오지 않습니다.

## 직접 빌드하기

[Rust](https://rustup.rs), [Node 20+](https://nodejs.org), 그리고 **MSVC 빌드 도구**(Visual Studio Build Tools 의
"C++를 사용한 데스크톱 개발")가 필요합니다. WebView2 는 Windows 10/11 에 들어 있습니다.

```powershell
cd windows
npm install
npm run tauri dev      # 고치면 바로 반영되는 개발용 빌드
npm run pack           # 설치 파일을 만들어 windows/release/ 에 둠
```

`npm run dev` 만 실행하면 화면 부분을 일반 브라우저에서 띄웁니다. 아일랜드 겉모습을 손보기엔 이걸로 충분합니다.
`dev/upload-preview.html` 도 함께 띄우는데, 파일 끌어다 놓기 동작 전체를 반복 재생합니다 — 탐색기에서 실제로
끌어 와야만 볼 수 있는 유일한 화면이라서요. `dev/recap-preview.html` 은 예시 한 주로 만든 주간 결산 카드와 공유
그림을 보여 줍니다. 이 페이지들은 앱에 들어가지 않습니다.

`npm run pack` 은 `windows/release/` 에 파일 두 개를 남깁니다. 배포 워크플로가 올리는 것과 같은 이름입니다.

```
Coucou-Windows-X.Y.Z-setup.exe    버전이 붙은 설치 파일
Coucou-Windows-setup.exe          같은 파일, 항상 최신을 가리키는 이름
```

설치는 하지 않아도 됩니다 — `target/release/coucou.exe` 만으로 실행됩니다. 작업 표시줄 창도 콘솔도 없습니다.
화면 위의 아일랜드와 알림 영역의 Mochi 가 앱의 전부이고, 종료(Quit)는 그 메뉴에 있습니다.

29개 소리는 macOS 앱의 파일을 그대로 쓰며, 이 폴더에 복사해 두지 않습니다. 경로는 `vite.config.ts` 맨 위의
`SOUNDS_DIR` 한 곳에만 적혀 있습니다 — 소리가 `shared/sounds/` 로 옮겨 가면 그 한 줄만 바꾸세요.

앱 아이콘과 트레이 아이콘도 Mochi 처럼 코드로 그립니다.

```powershell
npm run icons          # scripts/gen-icons.mjs 로 src-tauri/icons 를 다시 만듦
```

### 폴더 구성

```
windows/
  src/                 아일랜드 화면 (TypeScript, 프레임워크 없음)
    mochi/             Mochi 와 시작 인사, Canvas 2D 로 그림
    desktop/           Mochi 가 바탕화면에 살 때 쓰는 작은 창
    island/            상태 기계, 훅, 연동
    views/             아일랜드의 모든 화면
    settings/          설정 창
  src-tauri/           Rust 본체: 창, 이름 있는 파이프, Claude API, 주기적 확인
  hook/                coucou-hook.exe, Claude Code 중계 프로그램
  scripts/             아이콘 생성기
```

### 로그

`%LOCALAPPDATA%\Coucou\coucou.log` — 훅 이벤트, 권한 결정, 주기적 확인에서 생긴 문제. 내 컴퓨터에만 남습니다.
주간 결산 기록은 그 옆 `recap.json` 에 있습니다.

## 지원하는 에이전트

아래 에이전트는 모두 **Settings → Agents** 에서 Claude Code 와 같은 절차로 설치합니다: 정확한 diff, 날짜별 백업 경로,
클릭 전에는 쓰지 않음, 제거할 때는 Coucou 항목만 지움. Coucou 가 읽을 수 없는 설정이나 예상 밖의 내용이 있는 설정은
건드리지 않고 이유를 보여 줍니다. 에이전트마다 자기 필(`agent_<이름>`, 맥과 같은 ID 와 색)이 생깁니다. 파일은 맥과
같은 것으로, 윈도우는 `%USERPROFILE%`, 리눅스는 `~` 아래에 있습니다.

| 에이전트 | 설치되는 곳 | 권한 요청 |
|---|---|---|
| Claude Code | `.claude\settings.json` (**Settings → Claude Code**) | 아일랜드에서 허용 / 거부, 질문 답변 |
| Codex | `.codex\hooks.json` — 그다음 Codex 에서 `/hooks` 로 훅을 한 번 신뢰 | 아일랜드에서 허용 / 거부 |
| GitHub Copilot CLI | `.copilot\hooks\coucou.json` | 아일랜드에서 허용 / 거부 |
| Muse Code | `.config\muse\settings.json` | 아일랜드에서 허용 / 거부 |
| Gemini CLI | `.gemini\settings.json` | Gemini CLI 에서 물음 |
| Antigravity | `.gemini\config\hooks.json` (`coucou` 훅 묶음) | Antigravity 에서 물음 |
| Cursor Agent | `.cursor\hooks.json` — Cursor 터미널 안의 Claude Code 도 Claude Code 훅을 거쳐 Cursor 필에 나옴 | Cursor 에서 물음 |
| Claude Desktop (윈도우) | 설치할 것 없음: Claude 앱의 Claude Code 세션은 중계 프로그램이 표시를 붙임 | Claude 앱에서 물음 |
| OpenCode | 플러그인 `.config\opencode\plugins\coucou.js` | OpenCode 에서 물음 |
| Amp | 플러그인 `.config\amp\plugins\coucou.ts` | Amp 에서 물음 |
| Hermes Agent | 플러그인 `.hermes\plugins\coucou\` — 그다음 `hermes plugins enable coucou` 를 한 번 | Hermes 에서 물음 |
| 그 밖의 도구 | 도구의 훅에서 `coucou-hook --agent <이름> [<이벤트>]` 실행 | 그 도구에서 물음 |

중계 프로그램은 에이전트마다 다른 이벤트·필드 이름(Gemini CLI 의 `BeforeTool`, Copilot 의 `preToolUse`, Cursor 의
`beforeSubmitPrompt`…)을 Claude Code 의 것으로 맞추고, 각 에이전트가 기대하는 방식으로 답합니다. 스스로 무언가를
통과시키는 일은 없습니다. 클릭이 없으면 아무 결정도 내보내지 않으므로(JSON 이 필요한 에이전트엔 `{}`, 막는 쪽이
기본인 Copilot 엔 `"ask"`), 에이전트는 Coucou 가 없을 때와 똑같이 자기 터미널에서 묻습니다 — Coucou 가 꺼져 있을 때도요.

플러그인은 중간에 셸 없이 중계 프로그램을 바로 실행하고, 기다리지 않습니다. Amp 의 단계는 각 도구가 끝날 때마다
나타납니다. Amp 의 "실행 전" 훅은 판정을 돌려줘야 하는데, Coucou 는 판정을 내리지 않기 때문입니다.

**윈도우에서 에이전트별로 중계 프로그램을 실행하는 방식.** 훅 명령은 그것을 실행하는 셸에 맞춰 씁니다: Claude Code 는
Git Bash(따옴표, 슬래시), Gemini CLI 와 Copilot CLI 는 PowerShell(`& '…\coucou-hook.exe'`), Codex 는 `cmd /C`.
Cursor, Antigravity, Muse Code 는 셸을 문서로 밝히지 않아서, 경로에 공백이나 특수 문자가 없으면 그대로 — cmd,
PowerShell, 직접 실행 모두에서 동작 — 있으면 큰따옴표로 감싸 씁니다. 이 셋은 윈도우에서 시험해 보지 않았습니다.

필은 Coucou 가 위 파일들에서 자기 항목을 찾으면 **연결됨**입니다 — Settings → Agents 와 같은 확인입니다(Claude Code 는
Coucou 중계 프로그램을 실행하는 SessionStart 훅, Cursor 필은 Claude Code 훅도 셉니다). Coucou 는 아일랜드가 열릴 때마다
이 파일들을 읽기만 합니다. 아일랜드의 권한 카드는 Claude Code(어느 터미널이든, Cursor 터미널도), Codex, Copilot CLI,
Muse Code 에 나옵니다. 다른 에이전트와 Claude Desktop 은 자기 창에서 묻습니다.

## 맥판과 다른 점

- 노치가 없어서 아일랜드는 화면 맨 위 가운데에 살고, 노치에 숨는 대신 위쪽 가장자리로 들어갑니다.
- 권한 승인이 **어느** 터미널에서든 됩니다. 맥판은 VS Code 세션만 듣습니다.
- "Open terminal" 은 중계 프로그램 프로세스에서 위로 거슬러 올라가 세션을 돌리는 터미널이나 편집기를 찾습니다.
  옛날 콘솔 창(Windows Terminal 없는 `cmd.exe` 나 PowerShell)의 세션은 그런 부모가 없어서 — 그 창은 conhost 가
  갖고 있음 — 대신 그 폴더를 VS Code 로 엽니다. `code` 가 `PATH` 에 있고 아무것도 못 찾았을 때도 마찬가지입니다.
  리눅스도 같은 방식으로 거슬러 올라가며, 무엇을 앞으로 가져올 수 있는지는 데스크톱 환경에 달렸습니다(리눅스 절 참고).
- 전역 단축키는 윈도우에선 핫키, X11 리눅스에선 키 잡기, Wayland 에선 데스크톱의 GlobalShortcuts 포털을 거칩니다
  ([리눅스](#리눅스) 참고). 기다리는 카드는 **⌃** 나 아일랜드 안 `Esc` 로 접고, 아일랜드 클릭, 트레이의 Open,
  **Go to alert** 로 다시 엽니다.
- Apple Music 은 맥 목록 중 여기서 뒷받침할 게 없는 유일한 필이라 뺐습니다. Spotify 는 리눅스에만 있습니다
  ([리눅스](#리눅스) 참고). 윈도우에는 아직 읽어 올 방법이 없습니다.
- 이 버전에 없는 것: 떨어뜨린 파일을 이메일로 보내기, Mochi 를 창 위로 끌어다 그 창을 대화 맥락으로 붙이기.
  맥에서 이메일은 Resend 나 Apple Mail 스크립트로 보내는데, 여기서 파일을 안전하게 첨부할 마땅한 대안이 없고,
  파일 카드에 세 번째 버튼 자리도 없습니다.
- Cal.com 은 맥의 달력 대신 다음 예약을 목록으로 보여 줍니다.
- Cursor 필에는 Cursor Agent 자체 훅과 Cursor 터미널 안 Claude Code 가 함께 나옵니다.
- Hermes: Coucou 가 플러그인은 쓰지만 `hermes` 명령은 실행하지 않으므로, 한 번은 손으로 켜야 합니다. Hermes 는
  리눅스에서 기본으로 돌고, 윈도우에서는 시험해 보지 않았습니다.
- 요금제 사용량: 원래 쓰던 상태 줄을 윈도우에선 Git Bash 로 실행합니다(리눅스·맥은 `/bin/sh`). Git Bash 가 없으면
  `cmd` 로 어림짐작하지 않고 실행하지 않습니다. Codex 명령줄 도구는 `PATH` 와 npm, Volta, Bun, pnpm 폴더(리눅스는 nvm 도)에서 찾습니다.
- 채팅의 모델 선택은 팝오버 대신 채팅 카드 안에서 열리고, 맥에 없는 OpenRouter 와 OpenAI 호환 서버도 고를 수 있습니다.
  Google AI, OpenAI, OpenRouter 는 떨어뜨린 이미지를 실제로 봅니다(내용째 보냄). 맥은 이름만 보냅니다.
- 실시간 diff: 중계 프로그램은 수정이 끝난 뒤에야(PostToolUse) 수정 내용을 통째로 넘기며, 문자열당 256 KB, 이벤트당
  512 KB 까지입니다. 그보다 큰 수정은 틀린 숫자 대신 숫자 없는 "Edits · 파일" 단계로 보여 줍니다. diff 의 ↗ 는
  `PATH` 에 `code` 가 있어야 합니다. 없으면 파일 자체가 아니라 파일이 있는 폴더를 엽니다. 줄 수와 diff 는 Claude Code 의
  Edit, MultiEdit, Write 에서만 오며, 세션이 어느 필에 있든(VS Code, Cursor, Claude Desktop) 나옵니다. 다른 에이전트의
  수정은 일반 단계로만 나옵니다.
- 필이 꺼져 있는 동안 GitHub 목록을 대신 가져와 줄 아이폰이 없습니다.
- 단축키는 맥의 `⌃⌥` 자리에 `Ctrl+Alt` 를 쓰고 키도 다릅니다([키보드 단축키](#키보드-단축키) 참고). 아일랜드 안에서는
  맥의 `⌘` 자리에 `Ctrl` 을 씁니다. "Bring the terminal forward" 는 여기서 "Open terminal" 입니다. 이 버전에 없는 것:
  맨 앞 창 붙이기(그 ID 는 나중을 위해 남겨 둠). 아일랜드는 키보드를 가지고 있을 때만 자기 단축키를 읽습니다:
  채팅 중이거나, 전역 단축키로 열었을 때. `Ctrl+↓` `Ctrl+↑` `Ctrl+O` 는 맥처럼 GitHub 목록을 오르내리며, 맥의 세 줄
  목록과 달리 강조된 줄이 보이도록 스크롤됩니다. 채팅 입력란에서는 `Ctrl+↑` `Ctrl+↓` 가 그대로 커서를 움직입니다.
  **Go to alert** 는 어느 에이전트든 기다리는 카드를 그 필에서 띄웁니다. **Toggle the island** 는 아일랜드 안 `Esc`
  처럼 기다리는 카드를 버리지 않고 접습니다. **Send Mochi to the desktop**(`Ctrl+Alt+D`, 맥의 `⌃⌥D`)은 Mochi 를
  자기 자리로 — 그 자리가 사라진 모니터였다면 오른쪽 아래 구석으로 — 날려 보내고, 다시 누르면 집으로 돌아옵니다.
- 주간 결산:
  - 공유는 따로 뜨는 창 대신 아일랜드 안에서 하고, 저장은 어디에 둘지 묻지 않고 사진(또는 다운로드) 폴더에 바로
    씁니다. 시스템 공유 창은 없고, 복사는 웹 클립보드를 씁니다.
  - 질문은 에이전트가 물을 때(`AskUserQuestion`) 셉니다. 아일랜드에서 답하든 터미널에서 답하든요. 권한은 카드를 받는
    모든 에이전트에 대해 아일랜드 카드의 허용·거부 클릭을 셉니다.
  - 백그라운드 반복 없이는 들을 수 있는 절전/깨어남 알림이 없어서, 컴퓨터가 깨어난 뒤 월요일 카드는 첫 에이전트가
    시작하거나 아일랜드에 포인터를 올릴 때까지 기다립니다.
  - 카드가 맥보다 24 px 높습니다. 맥이 그림에만 넣는 최다 에이전트, 프로젝트, 가장 바빴던 날, 가장 긴 세션, 권한,
    질문도 카드에 적기 때문입니다. 세션이 끝나 중간에 잘린 턴도 셉니다.
  - 트레이 → Pause 는 기록을 멈추지 않습니다(어차피 컴퓨터에만 남음). Settings → General 에서 끄세요.
- 옷장은 Mochi 오른쪽 클릭, 트레이 메뉴, 또는 전역 단축키(기본 `Ctrl+Alt+G`)로 엽니다. 작은 아일랜드에서는 키 큰
  모자가 화면 위쪽 가장자리에 잘립니다. 맥에서 노치에 잘리는 것처럼요.
- 언어: 시스템과 상관없이 설정에서 고르고, 다시 시작하지 않고 바로 적용됩니다(맥의 **Restart Coucou** 가 필요 없음).
  아랍어는 아일랜드 글자를 오른쪽에서 왼쪽으로 돌리지만 배치는 그대로입니다. Mochi 와 필은 자기 자리를 지킵니다.
- 바탕화면의 Mochi 는 리눅스에서 Spotify 에 맞춰서만 춤춥니다. 윈도우에는 아직 춤출 음악 연동이 없습니다. 춤추는
  동안은 깨어 있습니다(맥은 춤추다 졸기도 합니다). Mochi 를 창 위에 떨어뜨려도 그 창이 채팅에 붙지 않습니다.
  잠든 동안에는 Mochi 주변의 투명한 사각형(120 px)이 첫 마우스 움직임을 받아 Mochi 를 깨우고, 나머지는 바탕화면에 돌려줍니다.

## 리눅스

같은 앱이 리눅스용으로도 빌드됩니다. 다른 부분은 모두 `src-tauri/src/platform/` 에, 중계 프로그램의 통신 방식은
`hook/src/unix.rs` 에 있습니다.

```bash
sudo apt install build-essential pkg-config \
  libwebkit2gtk-4.1-dev libgtk-layer-shell-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev libdbus-1-dev patchelf \
  gstreamer1.0-plugins-base gstreamer1.0-plugins-good
npm install
npm run tauri dev      # 고치면 바로 반영되는 개발용 빌드
npm run pack           # windows/release/ 에 AppImage, .deb, .rpm
```

Arch Linux 에서는 `linux/arch/` 의 패키지를 빌드해 설치합니다.

```bash
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/linux/arch
makepkg -si
```

`webkit2gtk-4.1`, `gtk-layer-shell`, `libayatana-appindicator` 가 필요합니다(의존성으로 함께 설치됨).
API 키는 GNOME Keyring 이나 KWallet 에 보관하세요.

리눅스에서 달라지는 점:

- **아일랜드**는 지원하는 컴포지터(COSMIC, KDE Plasma, Hyprland, Sway 등 wlroots 계열)에서 위쪽 가장자리에 붙은
  gtk-layer-shell 오버레이로, 위쪽 패널보다 위에 뜹니다. GNOME 은 layer-shell 이 없고 Wayland 창이 원하는 위치를
  무시하므로, 거기서는 XWayland 를 거쳐 도크 창으로 돕니다: 위쪽 가운데, 모든 작업 공간에, Super+D 뒤에도 남아 있음.
  `COUCOU_X11=0` 은 네이티브 Wayland 창을 유지하고, `COUCOU_DOCK=0` 은 도크 대신 유틸리티 창으로 만듭니다.
  `COUCOU_LAYER_SHELL=0` 은 어디서나 일반 창을 강제합니다.
- **클릭 통과**는 창의 입력 영역을 아일랜드 모양과 똑같이 유지하는 방식이라, 나머지 클릭은 컴포지터가 그 아래로 보냅니다.
- **Mochi 의 눈**은 포인터가 아일랜드 위에 있을 때만 따라갑니다. Wayland 는 그 밖의 커서 위치를 어떤 앱에도 알려 주지
  않습니다. 바탕화면에서도 마찬가지로 Mochi 위에 있을 때만 따라가고, "커서가 멀리 있다"(그래서 잠들 수 있다)는
  몇 초 동안 Mochi 위에 오지 않았다는 뜻입니다.
- **바탕화면의 Mochi** 는 아일랜드가 있는 모니터의 layer-shell 표면(KDE Plasma, COSMIC, Hyprland, Sway…)으로, 여백으로
  위치를 잡고 그 모니터 안에서만 끌 수 있습니다. X11 에서는 어디든 갈 수 있는 평범한 항상-위 창입니다. **Wayland 의
  GNOME** 은 layer-shell 이 없고 앱이 자기 창 위치를 정하지 못하게 하므로, 거기서는 Mochi 가 아일랜드를 떠날 수 없습니다.
  끌어도 아무 일이 없고, 바탕화면 단축키를 누르면 투덜대기만 합니다.
- **Claude Code 훅**은 `~/.local/share/coucou/bin/coucou-hook` 과 `$XDG_RUNTIME_DIR/coucou.sock` 유닉스 소켓을 거칩니다.
  양쪽 다 상대가 같은 사용자로 돌고 있는지 확인합니다. 다른 에이전트도 모두 같은 중계 프로그램을 `sh` 용 작은따옴표로
  감싸 쓰고, 설정은 `~` 아래에 있습니다(지원하는 에이전트 참고). 설정 파일이 심볼릭 링크(dotfiles)면 원본에 쓰고 권한도 유지합니다.
- **전역 단축키**는 X11 세션에서 X11 키 잡기입니다. X11 에서 AltGr 은 별도 수식 키라 `Ctrl+Alt` 와 겹치지 않습니다.
  데스크톱이 이미 쓰는 조합(GNOME 의 `Ctrl+Alt+T` 터미널, `Ctrl+Alt+←`/`→` 작업 공간 전환)은 *In use* 로 나옵니다.
  Wayland 에는 키 잡기가 없으므로, 거기서는 XDG **GlobalShortcuts 포털**(`xdg-desktop-portal`, 이를 지원하는 백엔드:
  KDE Plasma 6, GNOME 48+, Hyprland, COSMIC…)을 통해 데스크톱에 단축키를 넘깁니다. 각 단축키는 설명과 함께, 내가 정한
  키를 희망 키로 해서 넘어갑니다. 데스크톱이 확인하거나 다른 키를 고르는 자기 창을 띄울 수 있고, 최종 결정은 데스크톱이
  합니다. 이미 아는 단축키는 거기서 받은 키를 유지하며, 바꾸려면 데스크톱의 단축키 설정에서 바꿉니다.
  **Settings → Shortcuts** 는 단축키가 데스크톱에 등록됐다고 알려 주고, 각 단축키 옆에 데스크톱이 보고한 키
  (*Desktop: …*)를 보여 주며, 빠진 것을 표시합니다. Coucou 에서 단축키를 바꾸거나 끄면 새 포털 세션에서 새 묶음을
  등록합니다. 포털이 허락하면(`xdg-desktop-portal` 1.19+) Coucou 는 자신을 `fr.louisraille.coucou` 로 알립니다.
  앱 이름을 요구하는 데스크톱은 그렇지 않으면 거부할 수 있습니다.
  그런 포털이 없거나 데스크톱이 거부하면 아무것도 등록하지 않고, **Settings → Shortcuts** 에 데스크톱 키보드 설정에
  직접 묶을 명령을 대신 보여 줍니다(포털이 있어도 동작하지 않는 단축키에 대해서는 보여 줍니다):
  `coucou --shortcut openChat`(또는 AppImage 경로)은 이미 켜진 Coucou 에서 그 동작을 실행합니다. ID 는
  `toggleIsland`, `openChat`, `goToAlert`, `jumpToTerminal`, `nextPill`, `prevPill`, `muteToggle`, `desktopToggle`,
  `wardrobeToggle` 입니다.
- **키**는 Secret Service(GNOME Keyring, KWallet)에 보관합니다.
- **Spotify**(Settings → Integrations)는 세션 버스의 Spotify D-Bus 인터페이스인 MPRIS 로 읽습니다. 필은 곡을 보여 주고
  포인터를 올리면 재생·일시정지·건너뛰기를 하며, 카드에는 앨범 표지, 진행 막대(끌어서 이동), 셔플, 이전, 다음, 반복,
  음량이 있고, 재생 중에는 Mochi 가 춤춥니다 — 작은 아일랜드, Spotify 카드, 필, 바탕화면에서. 주기적으로 묻지 않습니다:
  필이 선언된 동안 스레드 하나가 버스에서 Spotify 의 신호를 기다리고, 선언이 풀리면 멈춥니다.
  **Open Spotify** 는 Spotify 를 앞으로 가져오거나 `PATH` 의 `spotify`, 없으면 Flatpak(`com.spotify.Client`)이나 Snap 을
  실행합니다. 하나도 없으면 **Get Spotify** 가 다운로드 페이지를 엽니다. 앨범 표지는 Spotify 이미지 CDN(또는 로컬 곡의
  파일)에서 페이지가 아닌 Coucou 가 직접 가져옵니다. MPRIS 로 재생 위치를 알려 주지 않는 Spotify 빌드는 곡이 시작된
  지점부터 막대를 보여 줍니다. 맥의 자동화(Automation) 권한 요청에 해당하는 것은 여기 없습니다.
- **요금제 사용량**: 상태 줄 중계는 `~/.local/share/coucou/bin/coucou-hook --statusline` 이며, Claude Code 처럼 원래 상태
  줄을 `/bin/sh -c` 로 실행합니다. Codex 는 `$PATH`, `~/.local/bin`, npm 전역 경로, Volta, Bun, pnpm, nvm(최신 Node 먼저)
  에서 찾습니다. 데스크톱에서 실행하면 셸보다 `$PATH` 가 짧은 경우가 많기 때문입니다.
- **Mochi 의 인사**는 계정 GECOS 필드의 전체 이름을 씁니다(`chfn` 으로 설정). 없으면 채팅은 이름 없이 인사합니다.
- **파일**: 설정은 `~/.config/coucou/`, 로그는 `~/.local/share/coucou/coucou.log`, 주간 결산 기록은 그 옆 `recap.json`.
  저장한 결산 그림은 `~/.config/user-dirs.dirs` 에 적힌 사진 폴더, 없으면 `~/Pictures`, 없으면 `~/Downloads` 로 갑니다.
- **언어**: 힌디어, 벵골어, 중국어, 아랍어는 그 문자를 가진 글꼴이 필요합니다. Coucou 는 Noto 글꼴을 찾습니다
  (Debian·Ubuntu 는 `fonts-noto-core`, `fonts-noto-cjk`, Arch 는 `noto-fonts`, `noto-fonts-cjk`). **System** 은 웹뷰에서
  언어를 읽으며, 웹뷰는 `LANGUAGE` / `LANG` 을 따릅니다.
- **결산 그림 복사**는 이미지 클립보드를 지원하는 WebKitGTK 가 필요합니다. 없으면 아일랜드가 알려 주며, 저장은 그대로 됩니다.
- 윈도우판에서 빠진 것은 여기서도 빠집니다: 파일을 이메일로 보내기, Mochi 를 창 위로 끌기.
- **Open terminal** 은 중계 프로그램에서 `/proc` 를 따라 위로 올라가(systemd, 로그인, ssh, 세션 관리자, 패널, 다른 사용자의
  프로세스에서 멈춤) 세션이 도는 터미널이나 편집기를 찾고, 그 창을 앞으로 가져옵니다 — 창이 여럿이면 세션 폴더 이름이
  제목인 창을:
  - **X11**(어느 데스크톱이든): EWMH — `_NET_WM_PID` 가 가장 가까운 부모인 창에 `_NET_ACTIVE_WINDOW` 요청을 그 작업
    공간에서 보냅니다. `_NET_WM_PID` 를 설정하지 않는 터미널(일부 옛 `xterm` 빌드)은 찾지 못합니다.
  - **Wayland 의 KDE Plasma**: 몇 줄짜리 KWin 스크립트를 `$XDG_RUNTIME_DIR` 에서 D-Bus 로 불러와 프로세스 ID 로 창을
    활성화하고, 바로 내립니다. Plasma 5 와 6.
  - **kitty** 는 어디서나: 원격 제어가 소켓에서 듣고 있으면(kitty.conf 의 `allow_remote_control`, `listen_on`) 탭까지 고릅니다.
  - **Wayland 의 GNOME** 과 다른 Wayland 컴포지터는 어떤 앱도 다른 앱의 네이티브 창을 앞으로 가져오지 못하게 합니다
    (기본 설정의 VS Code 같은 XWayland 창은 여전히 됩니다). 그런 곳과, 터미널이 부모가 아닌 tmux·screen·ssh 안의
    세션은 예전처럼 폴더를 VS Code 로 엽니다.
- **Claude Desktop** 필은 없습니다: Claude 앱에 리눅스판이 없습니다.
