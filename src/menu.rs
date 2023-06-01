use bevy::prelude::*;

use crate::{global, AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.on_startup())
            .add_system(button_system.in_set(OnUpdate(AppState::Menu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::Menu)));
    }
}

#[derive(Resource)]
struct MenuData {
    root: Entity,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let create_text = |text: &str, font_size: f32| {
        (
            TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/AtkinsonHyperlegible-Regular.ttf"),
                    font_size,
                    color: Color::WHITE,
                },
            ),
            Label,
        )
    };

    let button = || ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: Color::rgba(0.1, 0.0, 0.0, 1.0).into(),
        ..Default::default()
    };

    // root node
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: global::color::BACKGROUND.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(create_text("Bachman's eye", 120.0));

            parent.spawn(button()).with_children(|parent| {
                parent.spawn(create_text("Start", 20.0));
            });
        })
        .id();

    commands.insert_resource(MenuData { root });
}

fn button_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), (With<Button>, Changed<Interaction>)>,
) {
    for (interaction, mut background_color) in &mut query {
        dbg!(interaction);

        match *interaction {
            Interaction::Clicked => {
                *background_color = Color::RED.into();
                next_state.set(AppState::Simulation);
            }
            Interaction::Hovered => {
                *background_color = Color::ORANGE_RED.into();
            }
            Interaction::None => {
                *background_color = Color::BLUE.into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.root).despawn_recursive();
}
