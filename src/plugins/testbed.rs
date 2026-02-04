use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::{RED, SILVER},
    prelude::*,
};

use crate::plugins::player::VIEW_MODEL_RENDER_LAYER;

pub struct Testbed;

impl Plugin for Testbed {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_testbed, spawn_light, spawn_wall));
    }
}
pub static DEFAULT_RENDER_LAYER: usize = 0;

fn setup_testbed(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10);
    let outward_normal = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    commands.spawn((
        Mesh3d(meshes.add(floor)),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        RigidBody::Static,
        Collider::half_space(outward_normal),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

fn spawn_wall(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall = Cuboid::new(1.0, 4.0, 6.0);
    let wall_material = materials.add(Color::from(RED));
    commands.spawn((
        Mesh3d(meshes.add(wall)),
        MeshMaterial3d(wall_material),
        Transform::from_xyz(4.0, 0.5, 0.0),
        RigidBody::Static,
        Collider::cuboid(1.01, 4.01, 6.01),
    ));
}
