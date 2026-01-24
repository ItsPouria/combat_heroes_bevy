use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod plugins;
use plugins::dungeon::DungeonPlugin;
use plugins::player::PlayerPlugin;

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
            DungeonPlugin,
        ))
        .run();
}
