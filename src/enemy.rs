//! 적(Enemy) 모듈
//!
//! 적의 주기적 스폰과 이동을 담당합니다.
//! InGame 상태에서만 동작하며, 상태 전환 시 자동으로 정리됩니다.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::components::{CollisionRadius, Enemy, Velocity};
use crate::resources::{
    AppState, EnemySpawnTimer, ENEMY_COLLISION_RADIUS, ENEMY_SCALE, ENEMY_SPEED,
};

// =============================================================================
// 적 플러그인
// =============================================================================

/// 적 관련 시스템을 모아놓은 플러그인입니다.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // 게임 시작 시 스폰 타이머 리셋
            .add_systems(OnEnter(AppState::InGame), reset_spawn_timer)
            // 게임 종료 시 모든 적 정리
            .add_systems(OnExit(AppState::InGame), cleanup_enemies)
            // 게임플레이 시스템 (InGame 상태에서만 실행)
            .add_systems(
                Update,
                (enemy_spawning, enemy_movement, despawn_offscreen_enemies)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

// =============================================================================
// 시스템 (Systems)
// =============================================================================

/// 스폰 타이머를 리셋하는 시스템입니다.
///
/// 게임 재시작 시 타이머가 이전 상태를 유지하지 않도록
/// 명시적으로 리셋합니다.
fn reset_spawn_timer(mut spawn_timer: ResMut<EnemySpawnTimer>) {
    spawn_timer.0.reset();
}

/// 모든 적 엔티티를 정리하는 시스템입니다.
fn cleanup_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 주기적으로 적을 스폰하는 시스템입니다.
fn enemy_spawning(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.just_finished() {
        // 랜덤 X 위치 생성
        let mut rng = rand::thread_rng();
        let half_width = window.width() / 2.0 - ENEMY_COLLISION_RADIUS * 2.0;
        let spawn_x = rng.gen_range(-half_width..half_width);
        let spawn_y = window.height() / 2.0 + 50.0;

        // enemy.png 이미지 로드
        let texture: Handle<Image> = asset_server.load("enemy.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(spawn_x, spawn_y, 0.0),
                    scale: Vec3::splat(ENEMY_SCALE),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Velocity(Vec2::new(0.0, -ENEMY_SPEED)),
            CollisionRadius(ENEMY_COLLISION_RADIUS),
        ));
    }
}

/// 모든 적을 속도에 따라 이동시키는 시스템입니다.
fn enemy_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

/// 화면 아래로 나간 적을 제거하는 시스템입니다.
fn despawn_offscreen_enemies(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let min_y = -window.height() / 2.0 - 50.0;

    for (entity, transform) in query.iter() {
        if transform.translation.y < min_y {
            commands.entity(entity).despawn();
        }
    }
}
