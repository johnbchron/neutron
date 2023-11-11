mod node_material;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::node_material::NodeMaterial;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          present_mode: bevy::window::PresentMode::AutoNoVsync,
          ..default()
        }),
        ..default()
      }),
      node_material::NodeMaterialPlugin,
      WorldInspectorPlugin::new(),
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<NodeMaterial>>,
) {
  commands.spawn(Camera2dBundle::default());
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    transform: Transform::default().with_scale(Vec3::new(150.0, 100.0, 1.0)),
    material: materials.add(NodeMaterial {
      color:        Color::PURPLE,
      bounds:       Vec2::new(150.0, 100.0),
      border_width: 4.0,
      radius:       10.0,
    }),
    ..default()
  });
}
