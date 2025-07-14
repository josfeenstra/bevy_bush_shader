#import bevy_pbr::mesh_functions::{mesh_position_local_to_world, get_world_from_local}
#import bevy_pbr::mesh_view_bindings::{globals, view}
#import bevy_pbr::{
    view_transformations::position_world_to_clip,
}

fn rotate_vec2(v: vec2<f32>, angle: f32) -> vec2<f32> {
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    return vec2<f32>(
        v.x * cos_a - v.y * sin_a,
        v.x * sin_a + v.y * cos_a
    );
}

fn mod3() -> vec3<f32> {
    return vec3(0.1031, 0.11369, 0.13787);
}

fn hash31(p3: vec3<f32>) -> f32 {
    var q = fract(p3 * mod3());
    q += dot(q, q.yzx + vec3(19.19));
    return -1.0 + 2.0 * fract((q.x + q.y) * q.z);
}

fn hash33(p3: vec3<f32>) -> vec3<f32> {
    var q = fract(p3 * mod3());
    q += dot(q, q.yxz + vec3(19.19));
    return -1.0 + 2.0 * fract(vec3((q.x + q.y) * q.z, (q.x + q.z) * q.y, (q.y + q.z) * q.x));
}

fn simplex_noise(p: vec3<f32>) -> f32 {
    let K1 = 0.333333333;
    let K2 = 0.166666667;

    let i = floor(p + dot(p, vec3(K1)));
    let d0 = p - (i - vec3(K2) * dot(i, vec3(1.0)));

    let e = step(vec3(0.0), d0 - d0.yzx);
    let i1 = e * (1.0 - e.zxy);
    let i2 = vec3(1.0) - e.zxy * (1.0 - e);

    let d1 = d0 - (i1 - vec3(K2));
    let d2 = d0 - (i2 - vec3(2.0 * K2));
    let d3 = d0 - (vec3(1.0) - vec3(3.0 * K2));

    let h = max(vec4(0.6) - vec4(dot(d0, d0), dot(d1, d1), dot(d2, d2), dot(d3, d3)), vec4(0.0));
    let n = h * h * h * h * vec4(
        dot(d0, hash33(i)),
        dot(d1, hash33(i + i1)),
        dot(d2, hash33(i + i2)),
        dot(d3, hash33(i + vec3(1.0)))
    );

    return dot(vec4(31.316), n);
}

struct CustomMaterial {
    light: vec4<f32>,
    mid: vec4<f32>,
    dark: vec4<f32>,
    offset_size: f32,
    rotation_factor: f32,
};

@group(2) @binding(0)
var<uniform> material: CustomMaterial;
@group(2) @binding(1)
var sprite: texture_2d<f32>;
@group(2) @binding(2)
var sprite_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) quad_index: u32,
}



@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    var out: VertexOutput;
    let model = get_world_from_local(vertex.instance_index);
    let model_position = mesh_position_local_to_world(model, vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    let world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    let position = position_world_to_clip(world_position.xyz);
    
    let offset_size = material.offset_size * 1000.0;
    let viewport_size = view.viewport.zw;
    let viewport_pos = view.viewport.xy; 

    let per_quad_offset = hash31(vec3(f32(vertex.quad_index), f32(100 + vertex.quad_index), 0.0));

    let noise = simplex_noise((model_position+ vec3(0.0,0.0, per_quad_offset * 100.0) * 10.0)) * 7.0;

    let rotation = noise * material.rotation_factor;
    let raw_offset = vec2(vertex.uv.x - 0.5, 0.5-vertex.uv.y);
    let rotated = rotate_vec2(raw_offset, rotation);

    let offset = rotated * offset_size / viewport_size;
    let clip_offset = offset;

    // Or access the transformation matrices:
    // let clip_from_world = view.clip_from_world;
    // let world_from_view = view.world_from_view;
    
    // Example of using viewport information
    let base_x = viewport_pos.x;
    let base_y = viewport_pos.y;

    out.position = position + vec4(clip_offset, 0.0, 0.0);
    out.normal = vertex.normal;
    out.uv = vertex.uv;
    return out;
}

struct VertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    // @location(0) world_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_tangent: vec4<f32>,
    @location(3) color: vec4<f32>,
    // @location(5) @interpolate(flat) instance_index: u32,
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {

    let shade_1 = textureSample(sprite, sprite_sampler, input.uv).x;
    if shade_1 < 0.01 {
        discard;
    }

    // let color = vec4(input.uv, 0.0, 1.0);

    let sun_dir = normalize(vec3(1.0, 1.0, 4.0));
    let shade_2 = (1.0 + dot(input.normal, sun_dir)) * 0.5;

    let shade = mix(shade_1, shade_2, 0.5);

    // interpolate between 3 colors: dark, mid, light.
    let shade_color = select(
        mix(material.dark, material.mid, shade * 2.0),
        mix(material.mid, material.light, (shade - 0.50) * 2.0),
        shade >= 0.50
    );
    return shade_color;
}