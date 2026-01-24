use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy::window::{CursorOptions, WindowMode};
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster,
};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod plugins;
use plugins::player::PlayerPlugin;

const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            PlayerPlugin,
        ))
        .add_systems(Startup, (spawn_lights, generate_dungeon))
        .run();
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, 0.75),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

fn generate_dungeon(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)));
    let wall_left = meshes.add(Plane3d::new(Vec3::X, Vec2::splat(10.0)));
    let wall_right = meshes.add(Plane3d::new(-Vec3::X, Vec2::splat(10.0)));
    let materials = materials.add(Color::WHITE);

    commands.spawn((
        Mesh3d(floor),
        MeshMaterial3d(materials.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(wall_left),
        MeshMaterial3d(materials.clone()),
        Transform::from_xyz(-5.0, 5.0, 5.0),
    ));
    commands.spawn((
        Mesh3d(wall_right),
        MeshMaterial3d(materials.clone()),
        Transform::from_xyz(5.0, 5.0, 5.0),
    ));
}
