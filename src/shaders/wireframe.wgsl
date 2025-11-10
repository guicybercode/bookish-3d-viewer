struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = uniforms.model;
    let view_proj = uniforms.view_proj;
    out.clip_position = view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return uniforms.color;
}

