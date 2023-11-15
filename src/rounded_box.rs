use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
  sprite::{Material2d, Material2dPlugin},
};

use crate::{style::ThemeColor, NODE_HEIGHT, NODE_WIDTH};

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
pub struct RoundedBoxMaterial {
  #[uniform(0)]
  pub color:        Color,
  #[uniform(0)]
  pub size:         Vec2,
  #[uniform(0)]
  pub border_width: f32,
  #[uniform(0)]
  pub border_color: Color,
  #[uniform(0)]
  pub radius:       f32,
}

impl Default for RoundedBoxMaterial {
  fn default() -> Self {
    Self {
      color:        ThemeColor::Foreground.color(),
      size:         Vec2::new(NODE_WIDTH, NODE_HEIGHT),
      border_width: 1.0,
      border_color: ThemeColor::Outline.color(),
      radius:       16.0,
    }
  }
}

impl Material2d for RoundedBoxMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/box_material.wgsl".into()
  }
  fn vertex_shader() -> ShaderRef {
    "shaders/box_material.wgsl".into()
  }
}

pub struct RoundedBoxPlugin;

impl Plugin for RoundedBoxPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<RoundedBoxMaterial>::default())
      .register_asset_reflect::<RoundedBoxMaterial>();
  }
}
