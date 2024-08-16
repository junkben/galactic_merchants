// PixelPlanets/Planets/Rivers/LandRivers.gdshader

#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals
#import "shaders/planets/_utils.wgsl"::{
    rotate, spherify, cloud_alpha, to_linear
}

@group(0) @binding(1) var<uniform> globals: Globals;