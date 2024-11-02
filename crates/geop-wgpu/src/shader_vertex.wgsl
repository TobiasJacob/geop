// Vertex shader
struct Camera {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Light {
    direction: vec3<f32>,
}
@group(1) @binding(0)
var<uniform> light: Light;

struct VertexInput {
    @location(0) min_position: vec3<f32>,
    @location(1) max_position: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct InstanceInput {
    @location(3) min_instance_position: vec3<f32>,
    @location(4) max_instance_position: vec3<f32>,
    @location(5) instance_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
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

    out.position = camera.view_proj * vec4<f32>((in.min_position + in.max_position) / 2.0 + (instance.min_instance_position + instance.max_instance_position), 1.0);
    out.color = in.color * instance.instance_color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    
    out.color = in.color;

    return out;
}
 