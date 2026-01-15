//! Oxide Rain - 2D 탑다운 슈팅 게임
//!
//! Bevy 엔진을 사용한 완전한 게임 루프를 갖춘 슈팅 게임입니다.
//!
//! # 게임 상태 (AppState)
//! - MainMenu: 닉네임 입력 및 게임 시작 화면
//! - InGame: 게임 플레이 중
//! - GameOver: 게임 오버 화면 (닉네임과 함께 결과 표시)
//!
//! # 프로젝트 구조
//! ```
//! src/
//! ├── main.rs        - 진입점, 상태 관리, 플러그인 등록
//! ├── components.rs  - 공유 컴포넌트 정의
//! ├── resources.rs   - 전역 리소스 및 상수, AppState, PlayerName
//! ├── player.rs      - 플레이어 로직
//! ├── projectile.rs  - 투사체 로직
//! ├── enemy.rs       - 적 로직
//! ├── collision.rs   - 충돌 감지
//! └── ui.rs          - 메뉴, HUD, 게임 오버 UI
//! ```
//!
//! # 에셋 구조
//! ```
//! assets/
//! ├── fonts/
//! │   └── font.ttf   - 한글 지원 폰트
//! ├── player.png     - 플레이어 스프라이트
//! ├── enemy.png      - 적 스프라이트
//! └── bullet.png     - 투사체 스프라이트
//! ```

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
};

// =============================================================================
// 모듈 선언
// =============================================================================

mod components;
mod resources;
mod player;
mod projectile;
mod enemy;
mod collision;
mod ui;

// 리소스 가져오기
use resources::{
    AppState, EnemySpawnTimer, HighScore, IsNewRecord, PlayerName, Score, BACKGROUND_COLOR,
};

// =============================================================================
// 메인 함수
// =============================================================================

fn main() {
    App::new()
        // ─────────────────────────────────────────────────────────────────────
        // 기본 플러그인
        // ─────────────────────────────────────────────────────────────────────
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Oxide Rain".into(),
                resolution: (800, 600).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // ─────────────────────────────────────────────────────────────────────
        // 상태 설정
        // ─────────────────────────────────────────────────────────────────────
        .init_state::<AppState>()
        // ─────────────────────────────────────────────────────────────────────
        // 리소스 초기화
        // ─────────────────────────────────────────────────────────────────────
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<Score>()
        .init_resource::<HighScore>()
        .init_resource::<IsNewRecord>()
        .init_resource::<PlayerName>() // 플레이어 닉네임 리소스
        // ─────────────────────────────────────────────────────────────────────
        // 게임 플러그인
        // ─────────────────────────────────────────────────────────────────────
        .add_plugins((
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            enemy::EnemyPlugin,
            collision::CollisionPlugin,
            ui::UiPlugin,
        ))
        // ─────────────────────────────────────────────────────────────────────
        // 전역 시스템
        // ─────────────────────────────────────────────────────────────────────
        .add_systems(Startup, setup_camera)
        .run();
}

// =============================================================================
// 전역 시스템
// =============================================================================

/// 2D 카메라를 설정하는 시스템입니다.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Tonemapping::TonyMcMapface,
    ));
}
