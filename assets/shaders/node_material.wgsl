
#import bevy_pbr::forward_io::VertexOutput

struct NodeMaterial {
  color: vec4<f32>,
  bounds: vec2<f32>,
  border_width: f32,
  radius: f32,
};

@group(1) @binding(0) var<uniform> material: NodeMaterial;

@fragment
fn fragment(
  in: VertexOutput,
) -> @location(0) vec4<f32> {
  return material.color;
}
