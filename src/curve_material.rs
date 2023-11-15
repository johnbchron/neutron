use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
  sprite::{Material2d, Material2dPlugin},
};

use crate::style::ThemeColor;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
pub struct CurveMaterial {
  #[uniform(0)]
  pub point_a: Vec2,
  #[uniform(0)]
  pub point_b: Vec2,
  #[uniform(0)]
  pub point_c: Vec2,
  #[uniform(0)]
  pub point_d: Vec2,
  #[uniform(0)]
  pub color:   Color,
  #[uniform(0)]
  pub width:   f32,
}

impl Default for CurveMaterial {
  fn default() -> Self {
    Self {
      point_a: Vec2::new(-160.0, 120.0),
      point_b: Vec2::new(-100.0, 120.0),
      point_c: Vec2::new(100.0, -120.0),
      point_d: Vec2::new(160.0, -120.0),
      color:   ThemeColor::Outline.color(),
      width:   2.0,
    }
  }
}

impl Material2d for CurveMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/curve_material.wgsl".into()
  }
  fn vertex_shader() -> ShaderRef {
    "shaders/curve_material.wgsl".into()
  }
}

pub struct CurveMaterialPlugin;

impl Plugin for CurveMaterialPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<CurveMaterial>::default())
      .register_asset_reflect::<CurveMaterial>();
  }
}
