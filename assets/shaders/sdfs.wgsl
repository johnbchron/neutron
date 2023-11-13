
fn dot2(e1: vec2<f32>) -> f32 {
  return dot(e1, e1);
}

fn round_box_sdf(size: vec2<f32>, radius: f32, p: vec2<f32>) -> f32 {
  let d = abs(p) - size + radius;
  return min(max(d.x, d.y), 0.0) + length(max(d, vec2(0.0))) - radius;
}

fn quad_bezier_sdf(A: vec2<f32>, B: vec2<f32>, C: vec2<f32>, pos: vec2<f32>) -> f32 {
  let a = B - A;
  let b = A - 2.0 * B + C;
  let c = a * 2.0;
  let d = A - pos;
  let kk = 1.0 / dot(b, b);
  let kx = kk * dot(a, b);
  let ky = kk * (2.0 * dot(a, a) + dot(d, b)) / 3.0;
  let kz = kk * dot(d, a);
  var res = 0.0;
  let p = ky - kx * kx;
  let p3 = p * p * p;
  let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
  var h: f32 = q * q + 4.0 * p3;

  if ( h >= 0.0) { 
    h = sqrt(h);
    let x = (vec2(h, -h) -q) / 2.0;
    let uv = sign(x) * pow( abs(x), vec2(1.0 / 3.0) );
    let t = clamp( uv.x + uv.y - kx, 0.0, 1.0 );
    res = dot2( d + (c + b * t) * t);
  } else {
    let z = sqrt(-p);
    let v = acos( q / (p * z * 2.0) ) / 3.0;
    let m = cos(v);
    let n = sin(v) * 1.732050808;
    let t = clamp(vec3(m + m, -n - m, n - m) * z - kx, vec3(0.0), vec3(1.0));
    res = min( dot2(d + (c + b * t.x) * t.x),
               dot2(d + (c + b * t.y) * t.y));
  }
  return sqrt( res );
}

fn bad_cubic_bezier_sdf(A: vec2<f32>, B: vec2<f32>, C: vec2<f32>, D: vec2<f32>, p: vec2<f32>) -> f32 {
  return min(quad_bezier_sdf(A, B, D, p), quad_bezier_sdf(A, C, D, p));
}

fn cubic_bezier_sdf(
  NDC: vec2<f32>,
  P0: vec2<f32>,
  P1: vec2<f32>,
  P2: vec2<f32>,
  P3: vec2<f32>,
) {
  let A = -P0 + 3.0 * P1 - 3.0 * P2 + P3;
  let B = 3.0 * P0 - 6.0 * P1 + 3.0 * P2;
  let C = -3.0 * P0 + 3.0 * P1;
  let D = P0 - NDC;
  
  var coef = array<vec2<f32>, 6>(
    vec2(6.0 * dot(A, A), 0.0),
    vec2(10.0 * dot(A, B), 0.0),
    vec2(4.0 * (2.0 * dot(A, C) + dot(B, B)), 0.0),
    vec2(6.0 * (dot(A, D) + dot(B, C)), 0.0),
    vec2(2.0 * (2.0 * dot(B, D) + dot(C, C)), 0.0),
    vec2(2.0 * dot(C, D), 0.0)
  );
    
  let minSqDist = 9.0e30;
    
  coef[1] = cdiv(coef[1], coef[0]);
  coef[2] = cdiv(coef[2], coef[0]);
  coef[3] = cdiv(coef[3], coef[0]);
  coef[4] = cdiv(coef[4], coef[0]);
  coef[5] = cdiv(coef[5], coef[0]);
  coef[0] = cdiv(coef[0], coef[0]);

  var roots: array<vec2<f32>, 5>;
  
  findComplexRootsDurandKerner(roots, coef);

  let diff = vec2(0.0);

  for (int i = 0; i < 5; ++i) {
    diff = vec2(P(clamp(roots[i].x, 0.0, 1.0))) - NDC;
    minSqDist = min(minSqDist, dot(diff, diff));
  }
  
  float R = sinOSC(0.0, 0.3, iTime);
  return sqrt(minSqDist) - R;
}
