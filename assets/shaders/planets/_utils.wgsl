// Had to run a conversion on the final color, as it looked washed out.
// Credit to Andrew Baxter
// https://github.com/bevyengine/bevy/discussions/8937#discussioncomment-6269822
fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

fn glslmod(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}

fn glslmodv(x: vec2<f32>, y: vec2<f32>) -> vec2<f32> {
    return x - y * floor(x / y);
}

fn rand(coord: vec2<f32>, size: f32, seed: f32) -> f32 {
    let coord_mod = glslmodv(coord, (vec2(1.0, 1.0) * round(size)));
    return fract(sin(dot(coord_mod.xy, vec2(12.9898, 78.233))) * 15.5453 * seed);
}

fn noise(coord: vec2<f32>, size: f32, seed: f32) -> f32 {
    let i: vec2<f32> = floor(coord);
    let f: vec2<f32> = fract(coord);

    let a: f32 = rand(i, size, seed);
    let b: f32 = rand(i + vec2(1.0, 0.0), size, seed);
    let c: f32 = rand(i + vec2(0.0, 1.0), size, seed);
    let d: f32 = rand(i + vec2(1.0, 1.0), size, seed);

    let cubic: vec2<f32> = f * f * (3.0 - 2.0 * f);

    return mix(a, b, cubic.x) + (c - a) * cubic.y * (1.0 - cubic.x) + (d - b) * cubic.x * cubic.y;
}

fn fbm(coord: vec2<f32>, size: f32, seed: f32, octaves: u32) -> f32 {
    var value = 0.0;
    var scale = 0.5;
    var coord_copy = coord;

    for (var i = 0; i < i32(octaves); i++) {
        value += noise(coord_copy, size, seed) * scale;
        coord_copy += 2.0;
        scale *= 0.5;
    }

    return value;
}

// by Leukbaars from https://www.shadertoy.com/view/4tK3zR
fn circle_noise(uv: vec2<f32>, size: f32, seed: f32) -> f32 {
    var uv_copy = uv;
    let uv_y: f32 = floor(uv_copy.y);
    uv_copy.x = uv_y * 0.31;

    let f: vec2<f32> = fract(uv_copy);
    let h: f32 = rand(vec2(floor(uv_copy.x), floor(uv_y)), size, seed);

    let m: f32 = length(f - 0.25 - (h * 0.5));
    let r: f32 = h * 0.25;
    return smoothstep(0.0, r, m * 0.75);
}

fn cloud_alpha(uv: vec2<f32>, size: f32, seed: f32, octaves: u32, time: f32, time_speed: f32) -> f32 {
	// more iterations for more turbulence
    var c_noise: f32 = 0.0;
    for (var i = 0; i < 9; i++) {
        c_noise += circle_noise(
            (uv * size * 0.3) + (f32(i + 1) + 10.0) + (vec2(time * time_speed, 0.0)), size, seed
        );
    }
    return fbm(uv * size + c_noise + vec2(time * time_speed, 0.0), size, seed, octaves);
}

fn dither(uv_pixel: vec2<f32>, uv_real: vec2<f32>, pixels: f32) -> bool {
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
    return new_coord + 0.5;
}
