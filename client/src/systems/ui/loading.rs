use crate::state::AppState;
use bevy::prelude::*;

pub fn set_loading(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Loading).unwrap();
}

#[derive(Resource, Default, Debug)]
pub struct LoadingData {
    pub texture_atlas: bool,
    pub block_states: bool,
    pub ui: Option<Entity>,
}

pub fn setup_loading_ui(mut commands: Commands, mut data: ResMut<LoadingData>) {
    let ui = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgb(0.361, 0.42, 0.753).into(),
            ..default()
        })
        .id();

    data.ui = Some(ui);
}

pub fn remove_loading_ui(mut commands: Commands, data: ResMut<LoadingData>) {
    if let Some(ui) = data.ui {
        commands.entity(ui).despawn();
    }
    commands.remove_resource::<LoadingData>();
}

pub fn check_loading(data: Res<LoadingData>, mut app_state: ResMut<State<AppState>>) {
    if !data.is_changed() {
        return;
    }

    // Once every part is done loading.rs, show the main menu
    if data.texture_atlas && data.block_states {
        // If we're still in loading.rs mode, the block states being loaded means we're ready for the main menu. This may be changed in the future
        if app_state.current() == &AppState::Loading {
            app_state.set(AppState::MainMenu).unwrap();
        }
    }
}
