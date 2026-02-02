use avian3d::{PhysicsPlugins, math::Vector, prelude::Collider};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::plugins::character_controller::{CameraSensitivity, CharacterControllerBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_view_model)
            .add_plugins(PhysicsPlugins::default());
    }
}

// --- Components ---
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct WorldModelCamera;

#[derive(Component)]
pub struct ViewModelCamera;

pub static VIEW_MODEL_RENDER_LAYER: usize = 1;

// --- Systems ---
fn spawn_view_model(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let player_body = Capsule3d::new(0.2, 0.5);
    commands.spawn((
        Player,
        Mesh3d(meshes.add(player_body)),
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(30.0, 0.92, (30.0 as f32).to_radians()),
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ),
            (
                ViewModelCamera,
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
        ],
    ));
}
