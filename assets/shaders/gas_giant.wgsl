// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
//#import bevy_pbr::mesh_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var<uniform> pixels: f32;
@group(2) @binding(1) var<uniform> cloud_cover: f32;
@group(2) @binding(2) var<uniform> light_origin: vec2<f32>;
@group(2) @binding(3) var<uniform> time_speed: f32;
@group(2) @binding(4) var<uniform> stretch: f32;
@group(2) @binding(5) var<uniform> cloud_curve: f32;
@group(2) @binding(6) var<uniform> light_border_1: f32;
@group(2) @binding(7) var<uniform> light_border_2: f32;
@group(2) @binding(8) var<uniform> rotation: f32;
@group(2) @binding(9) var<uniform> color_a: vec4<f32>;
@group(2) @binding(10) var<uniform> color_b: vec4<f32>;
@group(2) @binding(11) var<uniform> color_c: vec4<f32>;
@group(2) @binding(12) var<uniform> color_d: vec4<f32>;
@group(2) @binding(13) var<uniform> size: f32;
@group(2) @binding(14) var<uniform> seed: f32;

//@group(2) @binding() var<uniform> octaves: i32;

// Had to run a conversion on the final color, as it looked washed out.
// Credit to Andrew Baxter
// https://github.com/bevyengine/bevy/discussions/8937#discussioncomment-6269822
fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

// ASSUMING 60 FPS
fn time() -> f32 {
    return globals.time;
}

fn glslmod(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}

fn glslmodv(x: vec2<f32>, y: vec2<f32>) -> vec2<f32> {
    return x - y * floor(x / y);
}

fn rand(coord: vec2<f32>) -> f32 {
    let coord_mod = glslmodv(coord, (vec2(1.0, 1.0) * round(size)));
    return fract(sin(dot(coord_mod.xy, vec2(12.9898, 78.233))) * 15.5453 * seed);
}

fn noise(coord: vec2<f32>) -> f32 {
    let i: vec2<f32> = floor(coord);
    let f: vec2<f32> = fract(coord);

    let a: f32 = rand(i);
    let b: f32 = rand(i + vec2(1.0, 0.0));
    let c: f32 = rand(i + vec2(0.0, 1.0));
    let d: f32 = rand(i + vec2(1.0, 1.0));

    let cubic: vec2<f32> = f * f * (3.0 - 2.0 * f);

    return mix(a, b, cubic.x) + (c - a) * cubic.y * (1.0 - cubic.x) + (d - b) * cubic.x * cubic.y;
}

fn fbm(coord: vec2<f32>) -> f32 {
    var value = 0.0;
    var scale = 0.5;
    var coord_copy = coord;

    var i: i32 = 0;
    loop {
        //if i >= octaves { break; }
        if i >= 5 { break; }

        value += noise(coord_copy) * scale;
        coord_copy += 2.0;
        scale *= 0.5;

        i = i + 1;
    }

    return value;
}

// by Leukbaars from https://www.shadertoy.com/view/4tK3zR
fn circle_noise(uv: vec2<f32>) -> f32 {
    var uv_copy = uv;
    let uv_y: f32 = floor(uv_copy.y);
    uv_copy.x = uv_y * 0.31;

    let f: vec2<f32> = fract(uv_copy);
    let h: f32 = rand(vec2(floor(uv_copy.x), floor(uv_y)));

    let m: f32 = length(f - 0.25 - (h * 0.5));
    let r: f32 = h * 0.25;
    return smoothstep(0.0, r, m * 0.75);
}

fn cloud_alpha(uv: vec2<f32>) -> f32 {
    var c_noise: f32 = 0.0;
	
	// more iterations for more turbulence
    var i: i32 = 0;
    var iterations: i32 = 9;
    loop {
        if i >= iterations { break; }

        c_noise += circle_noise(
            (uv * size * 0.3) + (f32(i + 1) + 10.0) + (vec2(time() * time_speed, 0.0))
        );

        i = i + 1;
    }

    return fbm(uv * size + c_noise + vec2(time() * time_speed, 0.0));
}

fn dither(uv_pixel: vec2<f32>, uv_real: vec2<f32>) -> bool {
    let v = glslmod((uv_pixel.x + uv_real.y), (2.0 / pixels));
    return v <= 1.0 / pixels;
}

fn spherify(uv: vec2<f32>) -> vec2<f32> {
    // stuck in top-left
    //let centered: vec2<f32> = uv * 2.0 - 1.0;
    //
    // kinda fixed
    //let centered: vec2<f32> = uv * 2.0 - 2.0;
    //
    let centered: vec2<f32> = 2.0 * (uv - 1.0);
    let z: f32 = pow(1.0 - dot(centered.xy, centered.xy), 0.5);
    let sphere: vec2<f32> = centered / (z + 1.0);
    return sphere * 0.5 + 0.5;
}

fn rotate(coord: vec2<f32>, angle: f32) -> vec2<f32> {
    var new_coord = coord;
    new_coord -= 0.5;
    new_coord *= mat2x2(
        vec2(cos(angle), -sin(angle)),
        vec2(sin(angle), cos(angle)),
    );
    return coord + 0.5;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // pixelize uv
    var uv = floor(mesh.uv * pixels) / pixels;
	
	// distance to light source
    let d_light: f32 = distance(uv, light_origin);
	
	// cut out a circle
    let d_circle: f32 = distance(uv, vec2(0.5));

	// stepping over 0.5 instead of 0.49999 makes some pixels a little buggy
    let a: f32 = step(d_circle, 0.49999);

    uv = rotate(uv, rotation);
	
	// map to sphere
    uv = spherify(uv);
	
	// slightly make uv go down on the right, and up in the left
    uv.y += smoothstep(0.0, cloud_curve, abs(uv.x - 0.4));

    let c: f32 = cloud_alpha(uv * vec2(1.0, stretch));
	
	// assign some colors based on cloud depth & distance from light
    var col: vec4<f32> = color_a;
    if c < cloud_cover + 0.03 {
        col = color_b;
    }
    if d_light + c * 0.2 > light_border_1 {
        col = color_c;
    }
    if d_light + c * 0.2 > light_border_2 {
        col = color_d;
    }

    col.a *= step(cloud_cover, c) * a;

    return to_linear(col);
}