// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
//#import bevy_pbr::mesh_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals
#import "shaders/planets/_utils.wgsl"::{
    rotate, spherify, cloud_alpha, to_linear, modified_time
}

struct GasGiantMaterialConfig {
    pixels: f32,
    cloud_cover: f32,
    light_origin: vec2<f32>,

    time_speed: f32,
    stretch: f32,
    cloud_curve: f32,
    light_border_1: f32,

    light_border_2: f32,
    rotation: f32,
    size: f32,
    octaves: u32,

    seed: f32,
}

struct GasGiantMaterialColors {
    base: vec4<f32>,
    outline: vec4<f32>,
    shadow_base: vec4<f32>,
    shadow_outline: vec4<f32>
}

// Access to current time through globals
@group(0) @binding(1) var<uniform> globals: Globals;

// Needs to be group 2 when extending Material2d
@group(2) @binding(0) var<uniform> info: GasGiantMaterialConfig;
@group(2) @binding(1) var<uniform> colors: GasGiantMaterialColors;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // pixelize uv
    var uv = floor(mesh.uv * info.pixels) / info.pixels;
	
	// distance to light source
    let d_light: f32 = distance(uv, info.light_origin);
	
	// cut out a circle
    let d_circle: f32 = distance(uv, vec2(0.5));

	// stepping over 0.5 instead of 0.49999 makes some pixels a little buggy
    let a: f32 = step(d_circle, 0.49999);

    // apply rotation
    uv = rotate(uv, info.rotation);
	
	// map to sphere
    uv = spherify(uv);
	
	// slightly make uv go down on the right, and up in the left
    uv.y += smoothstep(0.0, info.cloud_curve, abs(uv.x - 0.4));

    let time: f32 = modified_time(globals.time, info.time_speed);
    let c: f32 = cloud_alpha(
        uv * vec2(1.0, info.stretch),
        info.size,
        info.seed,
        info.octaves,
        time,
    );
	
	// assign some colors based on cloud depth & distance from light
    var final_color: vec4<f32> = colors.base;
    if c < info.cloud_cover + 0.03 {
        final_color = colors.outline;
    }
    if d_light + c * 0.2 > info.light_border_1 {
        final_color = colors.shadow_base;
    }
    if d_light + c * 0.2 > info.light_border_2 {
        final_color = colors.shadow_outline;
    }

    final_color.a *= step(info.cloud_cover, c) * a;
    return vec4<f32>(to_linear(final_color.rgb), final_color.a);
}