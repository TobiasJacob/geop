// Vertex shader
struct Uniforms {
    view_project: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    // @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    // @location(1) normal: vec3<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    // out.normal = (uniforms.world * vec4<f32>(in.normal, 0.0)).xyz;
    out.position = uniforms.view_project * vec4<f32>(in.position, 1.0);
    out.color = in.color; //vec4<f32>(in.color.rgb * in.color.a, in.color.a);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    
    out.color = in.color;

    return out;
}
 