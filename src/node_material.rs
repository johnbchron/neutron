use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
  sprite::{Material2d, Material2dPlugin},
};

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
pub struct NodeMaterial {
  #[uniform(0)]
  pub color:        Color,
  #[uniform(0)]
  pub bounds:       Vec2,
  #[uniform(0)]
  pub border_width: f32,
  #[uniform(0)]
  pub radius:       f32,
}

impl Material2d for NodeMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/node_material.wgsl".into()
  }
}

pub struct NodeMaterialPlugin;

impl Plugin for NodeMaterialPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<NodeMaterial>::default())
      .register_asset_reflect::<NodeMaterial>();
  }
}
