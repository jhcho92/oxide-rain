//! 공유 컴포넌트 모듈
//!
//! 여러 시스템에서 공통으로 사용하는 컴포넌트들을 정의합니다.
//! 컴포넌트는 순수한 데이터 구조체로, 어떠한 동작(behavior)도 포함하지 않습니다.

use bevy::prelude::*;

// =============================================================================
// 게임 엔티티 마커 컴포넌트
// =============================================================================
// 마커 컴포넌트는 빈 구조체로, 엔티티를 식별하고 쿼리에서 필터링하는 데 사용됩니다.

/// 플레이어 엔티티를 식별하는 마커 컴포넌트입니다.
#[derive(Component)]
pub struct Player;

/// 투사체(총알) 엔티티를 식별하는 마커 컴포넌트입니다.
#[derive(Component)]
pub struct Projectile;

/// 적 엔티티를 식별하는 마커 컴포넌트입니다.
#[derive(Component)]
pub struct Enemy;

// =============================================================================
// 물리/이동 컴포넌트
// =============================================================================

/// 이동 속도를 저장하는 컴포넌트입니다.
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// 충돌 반경을 저장하는 컴포넌트입니다.
#[derive(Component)]
pub struct CollisionRadius(pub f32);

// =============================================================================
// UI 마커 컴포넌트
// =============================================================================
// UI 엔티티를 식별하여 상태 전환 시 정리(cleanup)할 때 사용합니다.

/// 메인 메뉴 UI의 루트 엔티티를 식별하는 마커입니다.
/// OnExit(MainMenu)에서 이 마커가 붙은 엔티티를 모두 삭제합니다.
#[derive(Component)]
pub struct MainMenuUI;

/// 게임 플레이 중 UI (점수 표시 등)를 식별하는 마커입니다.
#[derive(Component)]
pub struct InGameUI;

/// 게임 오버 UI의 루트 엔티티를 식별하는 마커입니다.
#[derive(Component)]
pub struct GameOverUI;

/// 점수 텍스트를 식별하는 마커입니다.
/// 점수 업데이트 시스템에서 이 컴포넌트로 텍스트 엔티티를 찾습니다.
#[derive(Component)]
pub struct ScoreText;

/// 신기록 축하 텍스트를 식별하는 마커입니다.
/// 펄스 애니메이션을 적용할 때 사용합니다.
#[derive(Component)]
pub struct NewRecordText;

/// 닉네임 입력 커서 깜빡임을 위한 마커입니다.
#[derive(Component)]
pub struct CursorBlink;

/// 버튼 동작을 식별하는 열거형 컴포넌트입니다.
/// 버튼 클릭 시 어떤 동작을 수행할지 결정합니다.
#[derive(Component, Clone, Copy)]
pub enum ButtonAction {
    /// 게임 재시작 (GameOver → InGame)
    RestartGame,
    /// 메인 메뉴로 돌아가기 (GameOver → MainMenu)
    MainMenu,
}

// =============================================================================
// 애니메이션 컴포넌트
// =============================================================================

/// 펄스(맥박) 애니메이션을 위한 컴포넌트입니다.
///
/// # 필드
/// - `timer`: 애니메이션 주기를 제어하는 타이머
/// - `min_scale`: 최소 스케일 값
/// - `max_scale`: 최대 스케일 값
#[derive(Component)]
pub struct PulseAnimation {
    pub timer: Timer,
    pub min_scale: f32,
    pub max_scale: f32,
}

impl Default for PulseAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            min_scale: 1.0,
            max_scale: 1.2,
        }
    }
}
