use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Component)]
pub struct Setting<T>(pub T);

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsSound,
    #[default]
    Disabled,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnSoundSettingsMenuScreen;

#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Component)]
pub struct SelectedOption;
