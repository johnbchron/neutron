use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
  sprite::{Material2d, Material2dPlugin},
};

use crate::style::ThemeColor;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
pub struct BoxMaterial {
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

impl Default for BoxMaterial {
  fn default() -> Self {
    Self {
      color:        ThemeColor::Foreground.color(),
      size:         Vec2::new(320.0, 240.0),
      border_width: 1.0,
      border_color: ThemeColor::Outline.color(),
      radius:       16.0,
    }
  }
}

impl Material2d for BoxMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/box_material.wgsl".into()
  }
  fn vertex_shader() -> ShaderRef {
    "shaders/box_material.wgsl".into()
  }
}

pub struct BoxMaterialPlugin;

impl Plugin for BoxMaterialPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<BoxMaterial>::default())
      .register_asset_reflect::<BoxMaterial>();
  }
}
