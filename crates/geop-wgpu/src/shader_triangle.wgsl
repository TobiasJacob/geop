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
    @location(3) min_normal: vec3<f32>,
    @location(4) max_normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = camera.view_proj * vec4<f32>(vec3<f32>(in.min_position + in.max_position) / 2.0, 1.0);
    out.color = in.color;
    out.normal = vec3<f32>(vec3<f32>(in.min_normal + in.max_normal) / 2.0);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    let ambient_color = 0.5;

    let light_dir = normalize(light.direction);
    let diffuse_strength = max(dot(in.normal, -light_dir), 0.0);
    let diffuse_color = diffuse_strength * (1 - ambient_color);

    let result = (ambient_color + diffuse_color) * in.color.xyz;

    out.color = vec4<f32>(result, in.color.a);

    return out;
}
 