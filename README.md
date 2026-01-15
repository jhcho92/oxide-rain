# Oxide Rain

Oxide Rain은 Bevy 0.14 엔진을 사용하여 개발된 네온 사이버펑크 스타일의 2D 탑다운 슈팅 게임입니다.

## 🚀 프로젝트 특징

- **Bevy ECS 기반**: 효율적인 개체-컴포넌트-시스템 아키텍처를 사용하여 개발되었습니다.
- **네온 비주얼**: HDR 카메라와 Bloom 효과를 적용하여 화려한 비주얼을 제공합니다.
- **게임 루프**: 닉네임 입력, 게임 플레이, 스코어 기록, 게임 오버 등 완전한 게임 루프를 갖추고 있습니다.
- **최고 기록 시스템**: 로컬 세션 내에서의 최고 점수를 기록하고 관리합니다.

## 🛠️ 설치 및 실행 방법

### 1. Rust 설치하기

이 프로젝트를 빌드하려면 Rust 도구 모음이 필요합니다. Rustup을 통해 설치하는 것을 권장합니다.

**Windows (PowerShell):**
```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://ps1.rustup.rs'))
```

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

설치 완료 후 터미널을 재시작하고 다음 명령어로 설치 여부를 확인하세요:
```bash
rustc --version
```

### 2. 빌드 및 실행

프로젝트 루트 폴더에서 다음 명령어를 입력하여 게임을 실행할 수 있습니다.

```bash
# 개발 모드 실행 (컴파일 속도가 빠름)
cargo run

# 릴리스 모드 실행 (최적화된 성능)
cargo run --release
```

*참고: 첫 실행 시 의존성 라이브러리를 다운로드하고 컴파일하므로 시간이 다소 걸릴 수 있습니다.*

## 🎮 게임 조작법

- **이름 입력**: 메인 메뉴에서 닉네임을 입력하세요 (최대 10자).
- **이동**: `W`, `A`, `S`, `D` 키
- **공격**: `Space` 바
- **시작**: 메인 메뉴에서 `Enter`
- **재시작**: 게임 오버 화면에서 `Enter`

## 📂 프로젝트 구조

- `src/main.rs`: 프로그램 진입점 및 플러그인 설정
- `src/player.rs`: 플레이어 로직 및 컨트롤
- `src/enemy.rs`: 적 스폰 및 AI 로직
- `src/projectile.rs`: 발사체 시스템
- `src/collision.rs`: 충돌 감지 처리
- `src/ui.rs`: 메뉴 및 HUD 인터페이스
- `assets/`: 폰트 및 이미지 에셋

---
Built with [Bevy Engine](https://bevyengine.org/)
