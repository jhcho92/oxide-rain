//! UI 모듈
//!
//! 게임의 모든 사용자 인터페이스를 담당합니다.
//! - 메인 메뉴: 닉네임 입력 (Enter로 시작)
//! - 인게임 UI: 미니멀한 점수 표시 (숫자만)
//! - 게임 오버 화면: 닉네임과 함께 결과 표시
//!
//! # 주의: Bevy 0.14
//! ReceivedCharacter 이벤트가 제거되어 ButtonInput<KeyCode>로 입력을 처리합니다.

use bevy::prelude::*;

use crate::components::{
    ButtonAction, GameOverUI, InGameUI, MainMenuUI, NewRecordText, PulseAnimation, ScoreText,
};
use crate::resources::{AppState, HighScore, IsNewRecord, PlayerName, Score, MAX_NAME_LENGTH};

// =============================================================================
// 추가 UI 컴포넌트
// =============================================================================

/// 닉네임 표시 텍스트를 식별하는 마커입니다.
#[derive(Component)]
pub struct NicknameDisplay;

// =============================================================================
// UI 플러그인
// =============================================================================

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // 메인 메뉴
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                nickname_input_system.run_if(in_state(AppState::MainMenu)),
            )
            // 인게임
            .add_systems(OnEnter(AppState::InGame), setup_ingame_ui)
            .add_systems(OnExit(AppState::InGame), cleanup_ingame_ui)
            .add_systems(
                Update,
                update_score_text.run_if(in_state(AppState::InGame)),
            )
            // 게임 오버
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_ui)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_ui)
            .add_systems(
                Update,
                (animate_new_record_text, button_interaction_system)
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}

// =============================================================================
// 스타일 상수
// =============================================================================

const NEON_CYAN: Color = Color::srgb(0.0, 1.0, 1.0);
const NEON_PINK: Color = Color::srgb(1.0, 0.2, 0.8);
const GOLD: Color = Color::srgb(1.0, 0.85, 0.0);
const OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);

const BUTTON_NORMAL: Color = Color::srgba(0.1, 0.1, 0.2, 0.9);
const BUTTON_HOVERED: Color = Color::srgba(0.2, 0.2, 0.35, 0.95);
const BUTTON_PRESSED: Color = Color::srgba(0.05, 0.25, 0.35, 1.0);

// =============================================================================
// 메인 메뉴 시스템
// =============================================================================

/// 심플한 메인 메뉴 UI를 생성하는 시스템입니다.
/// Enter 키로 게임을 시작합니다.
fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_name: Res<PlayerName>,
) {
    let font: Handle<Font> = asset_server.load("fonts/font.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // 1. 게임 타이틀
            parent.spawn(TextBundle::from_section(
                "OXIDE RAIN",
                TextStyle {
                    font: font.clone(),
                    font_size: 80.0,
                    color: NEON_CYAN,
                },
            ));

            // 2. 닉네임 입력 안내
            parent.spawn(
                TextBundle::from_section(
                    "Enter Nickname (English/Numbers Only):",
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                }),
            );

            // 3. 닉네임 표시 (커서 포함)
            let display_text = if player_name.0.is_empty() {
                "_".to_string()
            } else {
                format!("{}_", player_name.0)
            };

            parent.spawn((
                TextBundle::from_section(
                    display_text,
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        color: GOLD,
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                }),
                NicknameDisplay,
            ));

            // 4. 시작 안내
            parent.spawn(
                TextBundle::from_section(
                    "Press [ENTER] to Start",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(60.0)),
                    ..default()
                }),
            );

            // 5. 조작법 안내
            parent.spawn(
                TextBundle::from_section(
                    "WASD: Move | SPACE: Shoot",
                    TextStyle {
                        font,
                        font_size: 18.0,
                        color: Color::srgb(0.4, 0.4, 0.4),
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(100.0)),
                    ..default()
                }),
            );
        });
}

/// 메인 메뉴 UI를 정리하는 시스템입니다.
fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 닉네임 입력 처리 시스템입니다.
///
/// # Bevy 0.14 호환
/// ReceivedCharacter 이벤트를 사용하여 텍스트 입력을 처리합니다.
/// 영문자와 숫자만 허용합니다.
fn nickname_input_system(
    mut char_evr: EventReader<ReceivedCharacter>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_name: ResMut<PlayerName>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<&mut Text, With<NicknameDisplay>>,
) {
    let mut text_changed = false;

    // Backspace: 마지막 문자 삭제
    if keyboard.just_pressed(KeyCode::Backspace) {
        if player_name.0.pop().is_some() {
            text_changed = true;
        }
    }

    // 문자 입력 처리
    for ev in char_evr.read() {
        for c in ev.char.chars() {
            // ASCII 영문자 및 숫자만 허용
            if c.is_ascii_alphanumeric() {
                if player_name.0.len() < MAX_NAME_LENGTH {
                    player_name.0.push(c);
                    text_changed = true;
                }
            }
        }
    }

    // 텍스트 업데이트 (이름 + 커서)
    if text_changed {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = if player_name.0.is_empty() {
                "_".to_string()
            } else {
                format!("{}_", player_name.0)
            };
        }
    }

    // Enter: 게임 시작
    if keyboard.just_pressed(KeyCode::Enter) {
        if !player_name.0.is_empty() {
            println!("Game Starting with player: {}", player_name.0);
            next_state.set(AppState::InGame);
        }
    }
}

// =============================================================================
// 인게임 UI 시스템 (미니멀 HUD)
// =============================================================================

/// 미니멀한 점수 HUD를 생성하는 시스템입니다.
/// 배경 없이 숫자만 우상단에 표시합니다.
fn setup_ingame_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/font.ttf");

    // 점수 텍스트 (우상단, 배경 없음)
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font,
                    font_size: 60.0,
                    color: GOLD,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(30.0),
                top: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        ScoreText,
        InGameUI,
    ));
}

/// 인게임 UI를 정리하는 시스템입니다.
fn cleanup_ingame_ui(mut commands: Commands, query: Query<Entity, With<InGameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 점수 텍스트를 업데이트하는 시스템입니다.
fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format_score(score.0);
        }
    }
}

/// 점수를 천 단위 구분 기호가 포함된 문자열로 변환합니다.
fn format_score(score: u32) -> String {
    let s = score.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

// =============================================================================
// 게임 오버 UI 시스템
// =============================================================================

/// 게임 오버 UI를 생성하는 시스템입니다.
fn setup_game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
    mut is_new_record: ResMut<IsNewRecord>,
    player_name: Res<PlayerName>,
) {
    let font: Handle<Font> = asset_server.load("fonts/font.ttf");

    // 신기록 판정
    let new_record = score.0 > high_score.0 && score.0 > 0;
    is_new_record.0 = new_record;

    if new_record {
        high_score.0 = score.0;
    }

    let name = if player_name.0.is_empty() {
        "Player".to_string()
    } else {
        player_name.0.clone()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    ..default()
                },
                background_color: OVERLAY_COLOR.into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            // GAME OVER
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: font.clone(),
                    font_size: 80.0,
                    color: Color::srgb(1.0, 0.2, 0.2),
                },
            ));

            // 플레이어 이름 + 점수
            parent.spawn(
                TextBundle::from_section(
                    format!("{}'s Score", name),
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: NEON_CYAN,
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                }),
            );

            // 점수 값
            parent.spawn(TextBundle::from_section(
                format_score(score.0),
                TextStyle {
                    font: font.clone(),
                    font_size: 70.0,
                    color: GOLD,
                },
            ));

            // 최고 기록
            parent.spawn(TextBundle::from_section(
                format!("High Score: {}", format_score(high_score.0)),
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgb(0.6, 0.7, 0.8),
                },
            ));

            // 신기록 또는 도발 메시지
            if new_record {
                parent.spawn((
                    TextBundle::from_section(
                        format!("NEW RECORD, {}!", name),
                        TextStyle {
                            font: font.clone(),
                            font_size: 36.0,
                            color: NEON_PINK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    }),
                    NewRecordText,
                    PulseAnimation::default(),
                ));
            } else {
                parent.spawn(
                    TextBundle::from_section(
                        format!("Nice try, {}!", name),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::srgb(0.6, 0.6, 0.7),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    }),
                );
            }

            // 재시작 버튼
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(40.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: BUTTON_NORMAL.into(),
                        border_color: NEON_CYAN.into(),
                        ..default()
                    },
                    ButtonAction::RestartGame,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "RESTART",
                        TextStyle {
                            font: font.clone(),
                            font_size: 28.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            // 메인 메뉴 버튼
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(10.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.05, 0.05, 0.1, 0.8).into(),
                        border_color: Color::srgb(0.4, 0.4, 0.5).into(),
                        ..default()
                    },
                    ButtonAction::MainMenu,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "MAIN MENU",
                        TextStyle {
                            font,
                            font_size: 22.0,
                            color: Color::srgb(0.7, 0.7, 0.8),
                        },
                    ));
                });
        });
}

/// 게임 오버 UI를 정리하는 시스템입니다.
fn cleanup_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 신기록 텍스트에 펄스 애니메이션을 적용하는 시스템입니다.
fn animate_new_record_text(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PulseAnimation), With<NewRecordText>>,
) {
    for (mut transform, mut pulse) in query.iter_mut() {
        pulse.timer.tick(time.delta());

        let progress = pulse.timer.fraction() * std::f32::consts::PI * 2.0;
        let pulse_factor = (progress.sin() + 1.0) / 2.0;
        let scale = pulse.min_scale + (pulse.max_scale - pulse.min_scale) * pulse_factor;

        transform.scale = Vec3::splat(scale);
    }
}

// =============================================================================
// 버튼 상호작용 시스템
// =============================================================================

/// 버튼 클릭을 감지하고 상태를 전환하는 시스템입니다.
fn button_interaction_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut score: ResMut<Score>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, mut border_color, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BUTTON_PRESSED.into();

                match action {
                    ButtonAction::StartGame | ButtonAction::RestartGame => {
                        score.0 = 0;
                        next_state.set(AppState::InGame);
                    }
                    ButtonAction::MainMenu => {
                        next_state.set(AppState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BUTTON_HOVERED.into();
                *border_color = Color::WHITE.into();
            }
            Interaction::None => {
                *bg_color = BUTTON_NORMAL.into();
                *border_color = NEON_CYAN.into();
            }
        }
    }
}
