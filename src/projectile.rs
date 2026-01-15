//! 투사체 모듈
//!
//! 투사체의 이동과 화면 밖 제거를 담당합니다.
//! 투사체 발사는 player.rs에서 처리합니다.
//! InGame 상태에서만 동작합니다.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{Projectile, Velocity};
use crate::resources::AppState;

// =============================================================================
// 투사체 플러그인
// =============================================================================

/// 투사체 관련 시스템을 모아놓은 플러그인입니다.
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // 게임 종료 시 모든 투사체 정리
            .add_systems(OnExit(AppState::InGame), cleanup_projectiles)
            // 게임플레이 시스템 (InGame 상태에서만 실행)
            .add_systems(
                Update,
                (projectile_movement, despawn_offscreen_projectiles)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

// =============================================================================
// 시스템 (Systems)
// =============================================================================

/// 모든 투사체 엔티티를 정리하는 시스템입니다.
fn cleanup_projectiles(mut commands: Commands, query: Query<Entity, With<Projectile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 모든 투사체를 속도에 따라 이동시키는 시스템입니다.
fn projectile_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

/// 화면 밖으로 나간 투사체를 제거하는 시스템입니다.
fn despawn_offscreen_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let max_y = window.height() / 2.0 + 50.0;

    for (entity, transform) in query.iter() {
        if transform.translation.y > max_y {
            commands.entity(entity).despawn();
        }
    }
}
