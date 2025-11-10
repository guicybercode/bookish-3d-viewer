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
    @location(0) normal: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = uniforms.model;
    let view_proj = uniforms.view_proj;
    out.clip_position = view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    out.normal = normalize((model_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.7));
    let ndotl = max(dot(in.normal, light_dir), 0.3);
    return uniforms.color * vec4<f32>(ndotl, ndotl, ndotl, 1.0);
}

