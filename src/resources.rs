//! 전역 리소스 모듈
//!
//! 리소스(Resource)는 엔티티에 붙지 않는 전역 싱글톤 데이터입니다.
//! 타이머, 점수, 게임 상태, 플레이어 이름 등을 관리합니다.

use bevy::prelude::*;

// =============================================================================
// 게임 상태 (App State)
// =============================================================================

/// 게임의 전체 상태를 나타내는 열거형입니다.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// 메인 메뉴 상태 (기본값)
    #[default]
    MainMenu,

    /// 게임 플레이 상태
    InGame,

    /// 게임 오버 상태
    GameOver,
}

// =============================================================================
// 게임 상수 (Game Constants)
// =============================================================================

/// 배경색 (아주 어두운 네이비 블루)
pub const BACKGROUND_COLOR: Color = Color::srgb(0.02, 0.02, 0.08);

// 플레이어 설정
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_COLLISION_RADIUS: f32 = 20.0;
pub const PLAYER_SCALE: f32 = 0.5;

// 투사체 설정
pub const PROJECTILE_SPEED: f32 = 500.0;
pub const PROJECTILE_COLLISION_RADIUS: f32 = 8.0;
pub const PROJECTILE_SCALE: f32 = 0.4;

// 적 설정
pub const ENEMY_SPEED: f32 = 150.0;
pub const ENEMY_COLLISION_RADIUS: f32 = 18.0;
pub const ENEMY_SCALE: f32 = 0.5;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;

// 점수 설정
pub const SCORE_PER_ENEMY: u32 = 100;

// 닉네임 설정
pub const MAX_NAME_LENGTH: usize = 12;

// =============================================================================
// 리소스 정의
// =============================================================================

/// 적 스폰 타이머 리소스입니다.
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            ENEMY_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

/// 현재 게임 점수를 저장하는 리소스입니다.
#[derive(Resource, Default)]
pub struct Score(pub u32);

/// 세션 최고 기록을 저장하는 리소스입니다.
#[derive(Resource, Default)]
pub struct HighScore(pub u32);

/// 신기록 달성 여부를 저장하는 리소스입니다.
#[derive(Resource, Default)]
pub struct IsNewRecord(pub bool);

/// 플레이어 닉네임을 저장하는 리소스입니다.
///
/// 메인 메뉴에서 키보드 입력으로 설정합니다.
/// 게임 오버 화면에서 이름과 함께 결과를 표시합니다.
#[derive(Resource)]
pub struct PlayerName(pub String);

impl Default for PlayerName {
    fn default() -> Self {
        Self(String::new())
    }
}
