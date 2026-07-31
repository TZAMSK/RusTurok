use bevy::prelude::*;

use bevy::{color::palettes::css::CRIMSON, ecs::component::Mutable};

use crate::ui::game_menu::components::{
    GameState, MenuButtonAction, MenuState, OnMainMenuScreen, OnSettingsMenuScreen,
    OnSoundSettingsMenuScreen, SelectedOption, Setting, Volume,
};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn game_menu_spawn(mut commands: Commands) {
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: px(30),
        position_type: PositionType::Absolute,
        left: px(10),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    Text::new("Menu"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        button_icon_node.clone(),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        button_icon_node.clone(),
                        (
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        button_icon_node,
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}

pub fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands.spawn((
        DespawnOnExit(MenuState::Settings),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            Children::spawn(SpawnIter(
                [
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        action,
                        children![(Text::new(text), button_text_style.clone())],
                    )
                })
            ))
        )],
    ));
}

pub fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    let volume = *volume;
    let button_node_clone = button_node.clone();
    commands.spawn((
        DespawnOnExit(MenuState::SettingsSound),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSoundSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        Spawn((Text::new("Volume"), button_text_style.clone())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: px(30),
                                        height: px(65),
                                        ..button_node_clone.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Setting(Volume(volume_setting)),
                                ));
                                if volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style)]
                )
            ]
        )],
    ));
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn setting_button<T: Resource + PartialEq + Copy>(
    interaction_query: Query<
        (&Interaction, &Setting<T>, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != button_setting.0 {
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = button_setting.0;
        }
    }
}
