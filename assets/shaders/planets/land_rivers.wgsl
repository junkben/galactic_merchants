// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
//#import bevy_pbr::mesh_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals
#import "shaders/planets/_utils.wgsl"::{
    rotate, spherify, to_linear, modified_time, dither, fbm
}

struct LandRiversMaterialConfig {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,

    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    light_border_1: f32,

    light_border_2: f32,
    river_cutoff: f32,
    size: f32,
    octaves: u32,

    seed: f32,
}

struct LandRiversMaterialColors {
    a: vec4<f32>,
    b: vec4<f32>,
    c: vec4<f32>,
    d: vec4<f32>,
    e: vec4<f32>,
    f: vec4<f32>,
}

// Access to current time through globals
@group(0) @binding(1) var<uniform> globals: Globals;

// Needs to be group 2 when extending Material2d
@group(2) @binding(0) var<uniform> info: LandRiversMaterialConfig;
@group(2) @binding(1) var<uniform> colors: LandRiversMaterialColors;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // pixelize uv
    var uv: vec2<f32> = floor(mesh.uv * info.pixels) / info.pixels;

    var can_dither: u32 = dither(uv, mesh.uv, info.pixels);

	// cut out a circle
    let d_circle: f32 = distance(uv, vec2(0.5));

	// stepping over 0.5 instead of 0.49999 makes some pixels a little buggy
    let a: f32 = step(d_circle, 0.49999);
	
	// map to sphere
    uv = spherify(uv);

    // distance to light source
    var d_light: f32 = distance(uv, info.light_origin);

    // apply rotation
    uv = rotate(uv, info.rotation);
	
	// some scrolling noise for landmasses
    let time: f32 = modified_time(globals.time, info.time_speed);
    let base_fbm_uv: vec2<f32> = uv * info.size + vec2(time, 0.0);
	
	// use multiple fbm's at different places so we can determine what color land gets
    var fbm1: f32 = fbm(base_fbm_uv, info.size, info.seed, info.octaves);
    var fbm2: f32 = fbm(base_fbm_uv - info.light_origin * fbm1, info.size, info.seed, info.octaves);
    var fbm3: f32 = fbm(base_fbm_uv - info.light_origin * 1.5 * fbm1, info.size, info.seed, info.octaves);
    var fbm4: f32 = fbm(base_fbm_uv - info.light_origin * 2.0 * fbm1, info.size, info.seed, info.octaves);

    var river_fbm: f32 = fbm(base_fbm_uv + fbm1 * 6.0, info.size, info.seed, info.octaves);
    river_fbm = step(info.river_cutoff, river_fbm);
	
	// size of edge in which colors should be dithered
    let dither_border: f32 = (1.0 / info.pixels) * info.dither_size;
	// lots of magic numbers here
	// you can mess with them, it changes the color distribution
    if d_light < info.light_border_1 {
        fbm4 *= 0.9;
    }
    if d_light > info.light_border_1 {
        fbm2 *= 1.05;
        fbm3 *= 1.05;
        fbm4 *= 1.05;
    }
    if d_light > info.light_border_2 {
        fbm2 *= 1.3;
        fbm3 *= 1.4;
        fbm4 *= 1.8;

        if d_light < info.light_border_2 + dither_border {
            if can_dither == 0u || info.should_dither == 0u {
                fbm4 *= 0.5;
            }
        }
    } 
	
	// increase contrast on d_light
    d_light = pow(d_light, 2.0) * 0.4;
    var final_color: vec4<f32> = colors.d;
    if fbm4 + d_light < fbm1 * 1.5 {
        final_color = colors.c;
    }
    if fbm3 + d_light < fbm1 * 1.0 {
        final_color = colors.b;
    }
    if fbm2 + d_light < fbm1 {
        final_color = colors.a;
    }
    if river_fbm < fbm1 * 0.5 {
        final_color = colors.f;
        if fbm4 + d_light < fbm1 * 1.5 {
            final_color = colors.e;
        }
    }

    final_color.a *= a;
    return vec4<f32>(to_linear(final_color.rgb), final_color.a);
}