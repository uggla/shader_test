#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shader_utils.wgsl"::NEG_HALF_PI
#import "shader_utils.wgsl"::shader_toy_default
#import "shader_utils.wgsl"::rotate2D
#import "shader_utils.wgsl"::TAU
#import "shader_utils.wgsl"::PI
#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;


fn rand2(uv: vec2<f32>) -> f32 {
    return fract(sin(dot(uv, vec2<f32>(13.337, 61.998))) * 48675.75647);
}

fn rotate(uv: vec2<f32>, a: f32) -> vec2<f32> {
    return vec2<f32>(
        uv.y * cos(a) + uv.x * sin(a),
        uv.x * cos(a) - uv.y * sin(a)
    );
}

fn rand2x2(uv: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(rand2(uv), rand2(-uv));
}

fn rand2x3(uv: vec2<f32>) -> vec3<f32> {
    return vec3<f32>(
        rand2(uv),
        rand2(-uv),
        rand2(vec2<f32>(-uv.x - 5., uv.y + 1.))
    );
}

fn perlin_noise(uv: vec2<f32>, t: f32) -> f32 {
    let id: vec2<f32> = floor(uv);
    let loc: vec2<f32> = fract(uv);
    let sloc: vec2<f32> = smoothstep(vec2<f32>(0.), vec2<f32>(1.), loc);

    return mix(
        mix(
            dot(loc, rotate(vec2<f32>(1.), rand2(id) * (PI * 2. + t))),
            dot(loc - vec2<f32>(1., 0.), rotate(vec2<f32>(1.), rand2(id + vec2<f32>(1., 0.)) * (PI * 2. + t))),
            sloc.x
        ),
        mix(
            dot(loc - vec2<f32>(0., 1.), rotate(vec2<f32>(1.), rand2(id + vec2<f32>(0., 1.)) * (PI * 2. + t))),
            dot(loc - vec2<f32>(1., 1.), rotate(vec2<f32>(1.), rand2(id + vec2<f32>(1., 1.)) * (PI * 2. + t))),
            sloc.x
        ),
        sloc.y
    );
}

fn fperlin_noise(uv: vec2<f32>, t: f32, iter: f32) -> f32 {
    var o: f32 = 0.;
    var k: f32 = 0.;
    for (var i: f32 = 0.; i < iter; i = i + 1.) {
        o += perlin_noise(uv * pow(2., i), t * pow(2., i)) / pow(2., i);
        k += 1. / pow(2., i);
    }
    return o / k;
}

// fn fnebula(uv: vec2<f32>, iter: f32, t: f32) -> f32 {
//     var o: f32 = 0.;
//     for (var i: f32 = 0.; i < iter; i = i + 1.) {
//         o += fperlin_noise(rotate(uv + vec2<f32>(t, 0.) / pow(1.5, i), i) * pow(1.8, i) / 2.0, 0.0, 6.0);
//     }
//     return o;
// }

fn fnebula(uv: vec2<f32>, iter: f32, t: f32) -> f32 {
    var o: f32 = 0.0;
    var p: f32 = 1.0;
    for (var i: f32 = 0.; i < iter; i = i + 1.) {
        o += fperlin_noise(rotate(uv + vec2<f32>(t, 0.) / p, i) * p / 2., 0., 6.);
        p *= 1.5;
    }
    return o;
}

fn vor(uv: vec2<f32>) -> f32 {
    let id = floor(uv);
    let loc = fract(uv);
    var o: f32 = 100.;
    for (var x: f32 = -1.; x <= 1.; x = x + 1.) {
        for (var y: f32 = -1.; y <= 1.; y = y + 1.) {
            o = min(o, distance(sin(2.5 * PI * rand2x2(id + vec2<f32>(x, y))) * 0.8 + 0.2, loc - vec2<f32>(x, y)));
        }
    }
    return o;
}

fn vorid3(uv: vec2<f32>) -> vec3<f32> {
    let id = floor(uv);
    let loc = fract(uv);
    var o: f32 = 1000.;
    var ou = vec3<f32>(0., 0., 0.);
    for (var x: f32 = -1.; x <= 1.; x = x + 1.) {
        for (var y: f32 = -1.; y <= 1.; y = y + 1.) {
            let d = distance(sin(2.5 * PI * rand2x2(id + vec2<f32>(x, y))) * 0.8 + 0.2, loc - vec2<f32>(x, y));
            if o > d {
                o = d;
                ou = rand2x3(id + vec2<f32>(x, y));
            }
        }
    }
    return ou;
}

fn star(uv: vec2<f32>) -> vec3<f32> {
    var val = vor(uv * 3.);
    val = 0.01 / val;
    val = pow(val, 1.7);
    var col = vec3<f32>(val) * vorid3(uv * 3.);
    return col * fperlin_noise(uv / 2., 0., 2.);
}

fn fstar(uv: vec2<f32>, iter: f32, t: f32) -> vec3<f32> {
    var o = vec3<f32>(0., 0., 0.);
    var p: f32 = 1.;
    for (var i: f32 = 0.; i < iter; i = i + 1.) {
        o += star(rotate(uv + vec2<f32>(t, 0.) / p, i) * p);
        p *= 1.5;
    }
    return o;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let time: f32 = globals.time;
    var uv: vec2<f32> = in.uv;
    uv = uv * 2.0 - 1.0; // center the uv coordinates
    var resolution: vec2<f32> = view.viewport.zw; // z is the width, w is the height
    if resolution.x > resolution.y {
        uv.x = uv.x * resolution.x / resolution.y;
    } else if resolution.y > resolution.x {
        uv.y = uv.y * resolution.y / resolution.x;
    }
    // nebula
    var n: f32 = fnebula(uv, 7., time / 5.);
    n = n * 0.4;
    n = clamp(n, 0., 1.);
    n = 1. - n;
    n = 0.5 / n;
    n = n - 0.5;

    // stars
    var col = fstar(uv, 7., time / 5.);
    col *= 10.;
    col = pow(col, vec3<f32>(1.));
    col = col.r * vec3<f32>(1., 0.45, 0.4) + col.g * vec3<f32>(0.4, 0.4, 1.) + col.b * vec3<f32>(1.);
    col = vec3<f32>(0., 0., 0.05) + clamp(vec3<f32>(0., 0., 0.03) + col, vec3<f32>(0.), vec3<f32>(1.));

    var vnb: vec3<f32> = n * vec3<f32>(0.7, 0.1, 1.);
    vnb = clamp(vnb, vec3<f32>(0.), vec3<f32>(1.));
    return vec4<f32>(vnb + col, 1.);
}
