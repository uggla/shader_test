#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
#import "custom_material_import.wgsl"::COLOR_MULTIPLIER
#import bevy_render::view::View
#import bevy_sprite::mesh2d_view_bindings::globals

@group(0) @binding(0) var<uniform> view: View;
@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
// @group(2) @binding(1) var base_color_texture: texture_2d<f32>;
// @group(2) @binding(2) var base_color_sampler: sampler;

const TAU:f32 =  6.28318530718;
const PI:f32 = 3.1415926535897932384626433832795;

fn rotate2D(angle: f32) -> mat2x2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat2x2<f32>(
        vec2<f32>(c, -s),
        vec2<f32>(s, c)
    );
}

fn sdCircle(p: vec2f, r: f32) -> f32 {
    return length(p) - r;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // return material_color * textureSample(base_color_texture, base_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
    // return material_color ;
    let uv = mesh.uv;
    var normalised_uv = (uv.xy * 2.0) - 1.0;
    //let resolution = view.viewport.zw;
    //normalised_uv.x *= resolution.x / resolution.y;
    //normalised_uv *= rotate2D(PI * globals.time / -2.0); // Our uvs are by default a -1,-1 in uppermost left cnr, this rotates you around.   let normalised_uv = (uv.xy * 2.0) - 1.0; // If you want 0,0 to be at the 'center' of your Mesh's geometry.

    var distance = sdCircle(normalised_uv, 0.5);
    distance = sin(distance * 8.0 + globals.time) / 8.0;
    distance = abs(distance);
    distance = smoothstep(0.0, 0.1, distance);


    // Map the distance to a grayscale color
    //let color_value = 1.0 - clamp(distance * 10.0, 0.0, 1.0);
    let color_value = vec3f(distance, distance, distance);

    return vec4f(vec3f(color_value), 1.0);
}

//@fragment
//fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//    var uv = (in.uv * 2.0) - 1.0;
//    var col = vec3f(0.);
//
//    let distance_to_center = vec2(0.25) - uv;
//    let angle = atan2(distance_to_center.y, distance_to_center.x);
//    let radius = length(distance_to_center) * 5.0;
//
//    col = hsv_to_srgb(vec3f((angle / TAU) + globals.time / 3.0, radius, 1.0));
//    return vec4f(col, 1.0);
//}

// From the bevy source code
fn hsv_to_srgb(c: vec3<f32>) -> vec3<f32> {
    let K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, vec3(0.0), vec3(1.0)), c.y);
}
