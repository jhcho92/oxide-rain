//! 충돌 감지 모듈
//!
//! 간단한 거리 기반(원형) 충돌 감지를 구현합니다.
//! 적-플레이어 충돌 시 GameOver 상태로 전환합니다.

use bevy::prelude::*;

use crate::components::{CollisionRadius, Enemy, Player, Projectile};
use crate::resources::{AppState, Score, SCORE_PER_ENEMY};

// =============================================================================
// 충돌 플러그인
// =============================================================================

/// 충돌 감지 관련 시스템을 모아놓은 플러그인입니다.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_projectile_enemy_collision,
                check_enemy_player_collision,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

// =============================================================================
// 시스템 (Systems)
// =============================================================================

/// 투사체와 적의 충돌을 검사하는 시스템입니다.
///
/// 충돌 시:
/// - 투사체와 적 모두 삭제
/// - 점수 증가
fn check_projectile_enemy_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    projectiles: Query<(Entity, &Transform, &CollisionRadius), With<Projectile>>,
    enemies: Query<(Entity, &Transform, &CollisionRadius), With<Enemy>>,
) {
    for (proj_entity, proj_transform, proj_radius) in projectiles.iter() {
        for (enemy_entity, enemy_transform, enemy_radius) in enemies.iter() {
            let distance = proj_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate());

            let collision_distance = proj_radius.0 + enemy_radius.0;

            if distance < collision_distance {
                // 충돌! 둘 다 제거
                commands.entity(proj_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();

                // 점수 증가
                score.0 += SCORE_PER_ENEMY;

                break;
            }
        }
    }
}

/// 적과 플레이어의 충돌을 검사하는 시스템입니다.
///
/// # 상태 전환
/// 적이 플레이어에 닿으면 GameOver 상태로 전환합니다.
/// NextState<AppState>를 사용하여 상태 전환을 요청합니다.
fn check_enemy_player_collision(
    mut next_state: ResMut<NextState<AppState>>,
    player: Query<(&Transform, &CollisionRadius), With<Player>>,
    enemies: Query<(&Transform, &CollisionRadius), With<Enemy>>,
) {
    // 플레이어가 없으면 조기 종료
    let Ok((player_transform, player_radius)) = player.get_single() else {
        return;
    };

    for (enemy_transform, enemy_radius) in enemies.iter() {
        let distance = player_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());

        let collision_distance = player_radius.0 + enemy_radius.0;

        if distance < collision_distance {
            // 게임 오버! 상태 전환 요청
            // NextState::set()으로 다음 프레임에 상태가 변경됩니다.
            next_state.set(AppState::GameOver);
            return;
        }
    }
}
