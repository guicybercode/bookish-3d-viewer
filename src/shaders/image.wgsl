struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var texture_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = uniforms.model;
    let view_proj = uniforms.view_proj;
    out.clip_position = view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    out.uv = vec2<f32>((model.position.x + 1.0) * 0.5, 1.0 - (model.position.y + 1.0) * 0.5);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.uv);
}

