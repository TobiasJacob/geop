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

struct InstanceInput {
    @location(2) instance_position: vec3<f32>,
    @location(3) instance_color: vec4<f32>,
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
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = uniforms.view_project * vec4<f32>(in.position + instance.instance_position, 1.0);
    out.color = in.color * instance.instance_color;
    //out.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    
    out.color = in.color;

    return out;
}
 