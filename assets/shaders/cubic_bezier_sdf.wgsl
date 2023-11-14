
// fn sinOSC(min: f32, max: f32, T: f32) -> f32 {
//   let v = (sin(T) + 1.0) * 0.5; // map T to [0.0, 1.0];
//   return min + v * (max - min);   // map T to [min, max];
// }

// fn rotateAroundZ(p: vec2<f32>, a: f32) -> vec2<f32> {
//   let s = sin(a);
//   let c = cos(a);

//   return vec2(
//     p.x * c - p.y * s,
//     p.x * s + p.y * c
//   );
// }

// fn cmul(l: vec2<f32>, r: vec2<f32>) -> vec2<f32> {
//   return vec2(l.x * r.x - l.y * r.y, l.x * r.y + r.x * l.y);
// }

// fn cmul_(_l: vec2<f32>, r: vec2<f32>) -> vec2<f32> {
//   var l = _l;
//   let p = vec2(l.x * r.x - l.y * r.y, l.x * r.y + r.x * l.y);
    
//   l.x = p.x;
//   l.y = p.y;

//   return l;
// }

// fn cdiv(l: vec2<f32>, r: vec2<f32>) -> vec2<f32> {
//   let inverseDivider = 1.0 / (r.x * r.x + r.y * r.y);

//   return vec2(
//     (l.x * r.x + r.y * l.y) * inverseDivider, 
//     (r.x * l.y - l.x * r.y) * inverseDivider
//   );
// }

// // /* https://en.wikipedia.org/wiki/Durand%E2%80%93Kerner_method */
// fn findComplexRootsDurandKerner(
//     _roots: array<vec2<f32>, 5>,
//     _coef: array<vec2<f32>, 6>
// ) -> array<vec2<f32>, 5> {
//     var roots = _roots;
//     var coef = _coef;
//     let arb = vec2(0.4, 0.9);

//     roots[0] = arb;

//     for (var r = 1; r < 5; r++) {
//         roots[r] = roots[r - 1] * arb;
//     }

//     for (var i = 0; i < 20; i++) {
//         for (var r = 0; r < 5; r++) {
//             var numer = coef[5];
//             var power = roots[r];

//             for (var c = 5 - 1; c >= 0; c--) {
//                 numer += cmul(coef[c], power);
//                 power = cmul_(power, roots[r]);
//             }

//             var denom = vec2(1.0, 0.0);

//             for (var k = 0; k < 5; k++) {
//                 if (k == r) { continue; }

//                 denom = cmul_(denom, roots[r] - roots[k]);
//             }

//             roots[r] -= cdiv(numer, denom);
//         }
//     }

//     return roots;
// }

// fn P(t: f32, A: vec2<f32>, B: vec2<f32>, C: vec2<f32>, P0: vec2<f32>) -> vec2<f32> {
//     return (P0 + t*(C + t*(B + t*A)));
// }

// fn cubic_bezier_sdf(
//   P0: vec2<f32>,
//   P1: vec2<f32>,
//   P2: vec2<f32>,
//   P3: vec2<f32>,
//   NDC: vec2<f32>,
// ) -> f32 {
//   let A = -P0 + 3.0 * P1 - 3.0 * P2 + P3;
//   let B = 3.0 * P0 - 6.0 * P1 + 3.0 * P2;
//   let C = -3.0 * P0 + 3.0 * P1;
//   let D = P0 - NDC;
  
//   var coef = array<vec2<f32>, 6>(
//     vec2(6.0 * dot(A, A), 0.0),
//     vec2(10.0 * dot(A, B), 0.0),
//     vec2(4.0 * (2.0 * dot(A, C) + dot(B, B)), 0.0),
//     vec2(6.0 * (dot(A, D) + dot(B, C)), 0.0),
//     vec2(2.0 * (2.0 * dot(B, D) + dot(C, C)), 0.0),
//     vec2(2.0 * dot(C, D), 0.0)
//   );
    
//   var minSqDist = 9.0e30;
    
//   coef[1] = cdiv(coef[1], coef[0]);
//   coef[2] = cdiv(coef[2], coef[0]);
//   coef[3] = cdiv(coef[3], coef[0]);
//   coef[4] = cdiv(coef[4], coef[0]);
//   coef[5] = cdiv(coef[5], coef[0]);
//   coef[0] = cdiv(coef[0], coef[0]);

//   var roots: array<vec2<f32>, 5>;
  
//   roots = findComplexRootsDurandKerner(roots, coef);

//   var diff = vec2(0.0);

//   for (var i = 0; i < 5; i++) {
//     diff = vec2(P(
//         clamp(roots[i].x, 0.0, 1.0),
//         A, B, C, P0,
//     )) - NDC;
//     minSqDist = min(minSqDist, dot(diff, diff));
//   }
  
//   // let R = sinOSC(0.0, 0.3, iTime);
//   return sqrt(minSqDist) - 1.0;
// }

// --------------------------------------------------------------------------

// fn length2( v: vec2<f32> ) -> f32 { return dot(v,v); }

// fn sdSegmentSq( p: vec2<f32>, a: vec2<f32>, b: vec2<f32> ) -> f32 {
//   let pa = p-a;
//   let ba = b-a;
//   let h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
//   return length2( pa - ba*h );
// }

// fn sdSegment( p: vec2<f32>, a: vec2<f32>, b: vec2<f32> ) -> f32 {
//   return sqrt(sdSegmentSq(p,a,b));
// }

// // slow, do not use in production. Can probably do better than
// // tesselation in linear segments.
// fn udBezier(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, pos: vec2<f32>) -> vec2<f32> {
//   let kNum = 50;
//   var res = vec2(1e10,0.0);
//   var a = p0;
//   for ( var i = 1; i < kNum; i++ ) {
//     let knum_minus_one = kNum - 1;
//     let t = f32(i) / f32(knum_minus_one);
//     let s = 1.0-t;
//     let b = p0*s*s*s + p1*3.0*s*s*t + p2*3.0*s*t*t + p3*t*t*t;
//     let d = sdSegmentSq( pos, a, b );
//     if ( d < res.x ) {res = vec2(d,t);}
//     a = b;
//   }
    
//   return vec2(sqrt(res.x),res.y);
// }

// fn cubic_bezier_sdf(P0: vec2<f32>, P1: vec2<f32>, P2: vec2<f32>, P3: vec2<f32>, pos: vec2<f32> ) -> f32 {
//   return udBezier(P0, P1, P2, P3, pos).x;
// }

// --------------------------------------------------------------------------

fn cubic_bezier_sdf(
  P0: vec2<f32>,
  P1: vec2<f32>,
  P2: vec2<f32>,
  P3: vec2<f32>,
  p: vec2<f32>
) -> f32 {
  let A = -P0 + 3.0*P1 - 3.0*P2 + P3;
  let B = 3.0*(P0 - 2.0*P1 + P2);
  let C = 3.0*(P1 - P0);
  let D = P0;
    
  let a5 = 6.0*dot(A,A);
  let a4 = 10.0*dot(A,B);
  let a3 = 8.0*dot(A,C) + 4.0*dot(B,B);
  let a2 = 6.0*dot(A,D-p) + 6.0*dot(B,C);
  let a1 = 4.0*dot(B,D-p) + 2.0*dot(C,C);
  let a0 = 2.0*dot(C,D-p);
    
  // calculate distances to the control points
  // let d0 = length(p-P0);
  // let d1 = length(p-P1);
  // let d2 = length(p-P2);
  // let d3 = length(p-P3);
  // let d = min(d0, min(d1, min(d2,d3)));
    
    
  var t: f32 = 0.5;
       
  // iterate
  for (var i = 0; i < 10; i++) {
    let t2 = t*t;
    let t3 = t2*t;
    let t4 = t3*t;
    let t5 = t4*t;
    
    let f = a5*t5 + a4*t4 + a3*t3 + a2*t2 + a1*t + a0;
    let df = 5.0*a5*t4 + 4.0*a4*t3 + 3.0*a3*t2 + 2.0*a2*t + a1;
        
    t = t - f/df;
  }
    
  t = clamp(t, 0.0, 1.0);
    
  // get the point on the curve
  let P = A*t*t*t + B*t*t + C*t + D;
        
  // return min(length(p-P), min(d0, d3));
  return length(p-P);
}
