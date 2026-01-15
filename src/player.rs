//! 플레이어 모듈
//!
//! 플레이어의 스폰, 이동, 입력 처리를 담당합니다.
//! InGame 상태에서만 동작하며, 상태 전환 시 자동으로 정리됩니다.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{CollisionRadius, Player, Projectile, Velocity};
use crate::resources::{
    AppState, PLAYER_COLLISION_RADIUS, PLAYER_SCALE, PLAYER_SPEED, PROJECTILE_COLLISION_RADIUS,
    PROJECTILE_SCALE, PROJECTILE_SPEED,
};

// =============================================================================
// 플레이어 플러그인
// =============================================================================

/// 플레이어 관련 시스템을 모아놓은 플러그인입니다.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // ─────────────────────────────────────────────────────────────────
            // 상태 진입/종료 시스템
            // ─────────────────────────────────────────────────────────────────
            // OnEnter(InGame): 게임 시작 시 플레이어 스폰
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            // OnExit(InGame): 게임 종료 시 플레이어 정리
            .add_systems(OnExit(AppState::InGame), cleanup_player)
            // ─────────────────────────────────────────────────────────────────
            // 게임플레이 시스템 (InGame 상태에서만 실행)
            // ─────────────────────────────────────────────────────────────────
            .add_systems(
                Update,
                (player_movement, player_shooting).run_if(in_state(AppState::InGame)),
            );
    }
}

// =============================================================================
// 시스템 (Systems)
// =============================================================================

/// 플레이어 엔티티를 스폰하는 시스템입니다.
///
/// # AssetServer
/// Bevy의 에셋 로딩 시스템입니다.
/// `asset_server.load("파일명")`으로 assets/ 폴더의 파일을 로드합니다.
/// 로딩은 비동기로 진행되며, Handle<T>을 즉시 반환합니다.
///
/// # SpriteBundle with Texture
/// 이전에는 Sprite { color: ... }로 단색 사각형을 그렸지만,
/// 이제는 texture 필드에 이미지 핸들을 전달하여 실제 이미지를 렌더링합니다.
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // player.png 이미지 로드
    // assets/player.png 경로에서 자동으로 로드됩니다.
    let texture: Handle<Image> = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            texture,
            // Transform으로 위치와 크기 조절
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 0.0),
                // 스프라이트 크기 조절 (원본이 크면 축소)
                scale: Vec3::splat(PLAYER_SCALE),
                ..default()
            },
            ..default()
        },
        Player,
        CollisionRadius(PLAYER_COLLISION_RADIUS),
    ));
}

/// 플레이어와 관련 엔티티를 정리하는 시스템입니다.
///
/// 게임 오버 또는 메인 메뉴로 돌아갈 때 호출됩니다.
fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 키보드 입력을 받아 플레이어를 이동시키는 시스템입니다.
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // 플레이어가 없으면 조기 종료 (안전 처리)
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };
    let window = window_query.single();

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        let movement = direction * PLAYER_SPEED * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }

    // 화면 경계 제한 (충돌 반경 고려)
    let half_width = window.width() / 2.0 - PLAYER_COLLISION_RADIUS;
    let half_height = window.height() / 2.0 - PLAYER_COLLISION_RADIUS;

    transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
    transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
}

/// 스페이스바를 누르면 투사체를 발사하는 시스템입니다.
fn player_shooting(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 플레이어가 없으면 조기 종료
        let Ok(player_transform) = query.get_single() else {
            return;
        };

        // bullet.png 이미지 로드
        let texture: Handle<Image> = asset_server.load("bullet.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y + 40.0,
                        0.0,
                    ),
                    scale: Vec3::splat(PROJECTILE_SCALE),
                    ..default()
                },
                ..default()
            },
            Projectile,
            Velocity(Vec2::new(0.0, PROJECTILE_SPEED)),
            CollisionRadius(PROJECTILE_COLLISION_RADIUS),
        ));
    }
}
