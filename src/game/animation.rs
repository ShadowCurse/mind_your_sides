use bevy::prelude::*;

use benimator::FrameRate;

use super::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate.in_set(OnUpdate(GameState::InGame)));
    }
}

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
pub struct Animation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

#[derive(Bundle)]
pub struct AnimationBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    animation: Animation,
    state: AnimationState,
}

impl AnimationBundle {
    pub fn new(
        texture_atlas: Handle<TextureAtlas>,
        size: f32,
        frames: usize,
        fps: f64,
        position: Vec3,
    ) -> Self {
        Self {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                texture_atlas,
                transform: Transform::from_translation(position),
                ..default()
            },
            animation: Animation(benimator::Animation::from_indices(
                0..=frames,
                FrameRate::from_fps(fps),
            )),
            state: AnimationState::default(),
        }
    }

    pub fn new_with_size(
        texture_atlas: Handle<TextureAtlas>,
        custom_size: Vec2,
        frames: usize,
        fps: f64,
        position: Vec3,
    ) -> Self {
        Self {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(custom_size),
                    ..default()
                },
                texture_atlas,
                transform: Transform::from_translation(position),
                ..default()
            },
            animation: Animation(benimator::Animation::from_indices(
                0..=frames,
                FrameRate::from_fps(fps),
            )),
            state: AnimationState::default(),
        }
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(
        &Animation,
        &Transform,
        &mut AnimationState,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animation, transform, mut player, mut texture) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();

        if transform.translation.x > 0.0 {
            texture.flip_x = true;
        }
    }
}
