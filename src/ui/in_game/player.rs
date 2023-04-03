use std::thread::spawn;
use bevy::a11y::AccessibilityNode;
use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use crate::GlobalState;
use crate::ui::{spawn_button, UiConfig};

pub struct PlayerUIPlugin;

impl Plugin for PlayerUIPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)));
    }
}
#[derive(Debug,Clone,Copy,Component)]
struct PlayerUIMarker;


fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {



            // LEFT BLACK PANEL
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                });



            // RIGHT BLACK PANEL
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::width(Val::Px(200.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                });
        });
}

