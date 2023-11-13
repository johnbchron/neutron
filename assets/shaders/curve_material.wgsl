
#import bevy_sprite::{
  mesh2d_functions as mesh_functions,
  mesh2d_view_bindings::view,
}

#import "shaders/sdfs.wgsl"::cubic_bezier_sdf

struct Vertex {
  @builtin(instance_index) instance_index: u32,
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
}

struct CurveMaterial {
  point_a: vec2<f32>,
  point_b: vec2<f32>,
  point_c: vec2<f32>,
  point_d: vec2<f32>,
  color: vec4<f32>,
  width: f32,
};

@group(1) @binding(0) var<uniform> material: CurveMaterial;

@fragment
fn fragment(
  in: VertexOutput,
) -> @location(0) vec4<f32> {
  let p = in.world_position.xy;
  var distance: f32 = cubic_bezier_sdf(
    material.point_a,
    material.point_b,
    material.point_c,
    material.point_d,
    p
  );
  distance = abs(distance) - material.width;

  return mix(
    material.color,
    vec4(material.color.xyz, 0.0),
    smoothstep(0.0, 0.5, distance)
  );
}

