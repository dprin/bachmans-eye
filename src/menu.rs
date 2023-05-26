use bevy::prelude::*;
use crate::global::ui::Button;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let create_text = |text: &str, font_size: f32| (
        TextBundle::from_section(
            text, 
            TextStyle {
                font: asset_server.load("fonts/AtkinsonHyperlegible-Regular.ttf"),
                font_size,
                color: Color::WHITE,
            }
        ),
        Label
    );

    let button = || {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                // margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // background_color: Color::hsl(30.0, 0.5, 0.1).into(),
            background_color: Color::rgba(0.1, 0.0, 0.0, 1.0).into(),
            ..Default::default()
        }
    };

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::hsl(30.0, 0.1, 0.1).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(
                create_text("Bachman's eye", 120.0),
            );

            parent.spawn(
                button()
            ).with_children(|parent| {
                parent.spawn(create_text("Layer 1", 20.0));
            });
        });
}

fn button_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        // (Changed<Interaction>, With<Button>),
        With<Button>,
    >
) {
    for (interaction, mut background_color) in &mut query {
        dbg!(interaction);

        match *interaction {
            Interaction::Clicked => {
                *background_color = Color::ORANGE_RED.into();
            },
            Interaction::Hovered => {
                *background_color = Color::ORANGE_RED.into();
            },
            Interaction::None => {
                *background_color = Color::BLUE.into();
            },
        }
    }
}
