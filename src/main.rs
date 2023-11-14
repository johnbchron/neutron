mod box_material;
mod camera;
mod curve_material;
mod style;

use bevy::{
  prelude::*,
  render::view::NoFrustumCulling,
  sprite::{Anchor, MaterialMesh2dBundle},
  text::Text2dBounds,
  window::{WindowMode, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use curve_material::CurveMaterial;

use crate::{box_material::BoxMaterial, style::ThemeColor};

const NODE_TEXT_PADDING: f32 = 16.0;

fn main() {
  App::new()
    .insert_resource(ClearColor(ThemeColor::Background.color()))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        resolution: WindowResolution::default().with_scale_factor_override(2.0),
        mode: WindowMode::BorderlessFullscreen,
        ..default()
      }),
      ..default()
    }))
    .add_plugins((
      box_material::BoxMaterialPlugin,
      curve_material::CurveMaterialPlugin,
      camera::CameraPlugin,
      WorldInspectorPlugin::new(),
    ))
    .add_systems(Startup, (setup_test_node, setup_test_curve))
    .run();
}

fn setup_test_node(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<BoxMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands
    .spawn((
      MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(BoxMaterial::default()),
        ..default()
      },
      NoFrustumCulling,
    ))
    .with_children(|parent| {
      let font = asset_server.load("fonts/FiraMono-Medium.ttf");
      let text_style = TextStyle {
        font:      font.clone(),
        font_size: 36.0,
        color:     ThemeColor::Text.color(),
      };
      let text_alignment = TextAlignment::Left;
      parent.spawn(Text2dBundle {
        text: Text::from_section("Test Node Title", text_style.clone())
          .with_alignment(text_alignment),
        text_anchor: Anchor::TopLeft,
        text_2d_bounds: Text2dBounds {
          size: Vec2::new(
            320.0 - NODE_TEXT_PADDING * 2.0,
            240.0 - NODE_TEXT_PADDING * 2.0,
          ),
          ..default()
        },
        transform: Transform::from_translation(Vec3::new(
          -160.0 + NODE_TEXT_PADDING,
          120.0 - NODE_TEXT_PADDING,
          0.0,
        )),
        ..default()
      });
    });
}

fn setup_test_curve(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<CurveMaterial>>,
) {
  commands.spawn((
    MaterialMesh2dBundle {
      mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      material: materials.add(CurveMaterial {
        point_a: Vec2::new(160.0, 0.0),
        point_b: Vec2::new(200.0, 0.0),
        point_c: Vec2::new(200.0, 40.0),
        point_d: Vec2::new(240.0, 40.0),
        ..default()
      }),
      ..default()
    },
    NoFrustumCulling,
  ));
}
