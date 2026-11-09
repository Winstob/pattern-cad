struct DrawIndirectArgs {
    vertex_count: atomic<u32>,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
};

struct ViewTransform {
    matrix: mat4x4<f32>,
    viewport_size: vec2<f32>,
    _padding: vec2<f32>,
}

struct GpuVertex {
    position: vec4<f32>,
    color: vec4<f32>,
};

struct GpuPoint {
    position: vec2<f32>,
    element_id: u32,
    _padding: u32,
};

@group(0) @binding(0)
var<storage, read_write> vertices: array<GpuVertex>;

@group(0) @binding(1)
var<storage, read_write> draw_args: DrawIndirectArgs;

@group(0) @binding(2)
var<uniform> view_transform: ViewTransform;

@group(0) @binding(3)
var<storage, read> points: array<GpuPoint>;

@group(0) @binding(4)
var<uniform> num_points: u32;

struct Triangle {
    v0: GpuVertex,
    v1: GpuVertex,
    v2: GpuVertex,
};

const PI = 3.14159265358979f;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let point_index = global_id.x;

    if (point_index >= num_points) {
        return;
    }
    let point = points[point_index];

    let num_triangles: u32 = 16; // must be at least 3.
                                 // also need to make sure there is enough space in
                                 // the vertex buffer
    let vertex_buf_index = atomicAdd(
        &draw_args.vertex_count,
        3u * num_triangles
    );

    let center_world = vec4<f32>(point.position.x, -point.position.y, 0.0, 1.0);
    let center_ndc = (view_transform.matrix * center_world).xy;

    let half_size = 3.0; // pixels
    var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    let radius_hypotenuse_ndc = vec2<f32>(2f*half_size/view_transform.viewport_size.x, 2f*half_size/view_transform.viewport_size.y);

    for (var triangle_index = 0u; triangle_index < num_triangles; triangle_index = triangle_index + 1) {
        let start_angle = PI*2f / f32(num_triangles) * f32(triangle_index) + PI/2f;
        let end_angle = PI*2f / f32(num_triangles) * f32(triangle_index + 1) + PI/2f;

        let v1 = center_ndc.xy;
        let v2 = center_ndc.xy + vec2<f32>(cos(start_angle), sin(start_angle))*radius_hypotenuse_ndc;
        let v3 = center_ndc.xy + vec2<f32>(cos(end_angle), sin(end_angle))*radius_hypotenuse_ndc;

        vertices[vertex_buf_index + triangle_index*3 + 0u].position = vec4<f32>(v1, 0.0, 1.0);
        vertices[vertex_buf_index + triangle_index*3 + 0u].color = color;
        vertices[vertex_buf_index + triangle_index*3 + 1u].position = vec4<f32>(v2, 0.0, 1.0);
        vertices[vertex_buf_index + triangle_index*3 + 1u].color = color;
        vertices[vertex_buf_index + triangle_index*3 + 2u].position = vec4<f32>(v3, 0.0, 1.0);
        vertices[vertex_buf_index + triangle_index*3 + 2u].color = color;
    }
}
