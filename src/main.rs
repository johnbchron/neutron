mod camera;
mod curve_material;
mod node;
mod rounded_box;
mod style;

use bevy::{
  prelude::*,
  render::view::NoFrustumCulling,
  sprite::{Anchor, MaterialMesh2dBundle},
  text::Text2dBounds,
  window::{PresentMode, WindowMode, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use curve_material::CurveMaterial;

use crate::{rounded_box::RoundedBoxMaterial, style::ThemeColor};

const NODE_PADDING: f32 = 16.0;
const NODE_WIDTH: f32 = 320.0;
const NODE_HEIGHT: f32 = 240.0;
const NODE_LEVEL_SEP: f32 = 64.0;
const NODE_MARGIN: f32 = 40.0;

fn main() {
  App::new()
    .insert_resource(ClearColor(ThemeColor::Background.color()))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        resolution: WindowResolution::default().with_scale_factor_override(2.0),
        mode: WindowMode::BorderlessFullscreen,
        present_mode: PresentMode::Immediate,
        ..default()
      }),
      ..default()
    }))
    .add_plugins((
      rounded_box::RoundedBoxPlugin,
      curve_material::CurveMaterialPlugin,
      camera::CameraPlugin,
      node::NodePlugin,
      WorldInspectorPlugin::new(),
    ))
    // .add_systems(Startup, (setup_test_node, setup_test_curve))
    .run();
}

fn setup_test_node(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<RoundedBoxMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands
    .spawn((
      MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(RoundedBoxMaterial::default()),
        ..default()
      },
      NoFrustumCulling,
    ))
    .with_children(|parent| {
      let font = asset_server.load("fonts/FiraMono-Medium.ttf");
      let title_style = TextStyle {
        font:      font.clone(),
        font_size: 32.0,
        color:     ThemeColor::Text.color(),
      };
      let text_style = TextStyle {
        font:      font.clone(),
        font_size: 16.0,
        color:     ThemeColor::Text.color(),
      };
      let text_alignment = TextAlignment::Left;
      parent.spawn(Text2dBundle {
        text: Text::from_sections(vec![
          TextSection::new("Test Node Title\n", title_style.clone()),
          TextSection::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
             eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            text_style.clone(),
          ),
        ])
        .with_alignment(text_alignment),
        text_anchor: Anchor::TopLeft,
        text_2d_bounds: Text2dBounds {
          size: Vec2::new(
            NODE_WIDTH - NODE_PADDING * 2.0,
            NODE_HEIGHT - NODE_PADDING * 2.0,
          ),
          ..default()
        },
        transform: Transform::from_translation(Vec3::new(
          -(NODE_WIDTH / 2.0) + NODE_PADDING,
          (NODE_HEIGHT / 2.0) - NODE_PADDING,
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
      material: materials.add(CurveMaterial {
        point_a: Vec2::new(NODE_WIDTH / 2.0, 0.0),
        point_b: Vec2::new((NODE_WIDTH / 2.0) + NODE_LEVEL_SEP, 0.0),
        point_c: Vec2::new((NODE_WIDTH / 2.0) + NODE_LEVEL_SEP, 80.0),
        point_d: Vec2::new((NODE_WIDTH / 2.0) + (NODE_LEVEL_SEP * 2.0), 80.0),
        ..default()
      }),
      ..default()
    },
    NoFrustumCulling,
  ));
}
