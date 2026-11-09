struct DrawIndirectArgs {
    vertex_count: atomic<u32>,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
};

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
var<uniform> view_transform: mat4x4<f32>;

@group(0) @binding(3)
var<storage, read> points: array<GpuPoint>;

@group(0) @binding(4)
var<uniform> num_points: u32;

struct Triangle {
    v0: GpuVertex,
    v1: GpuVertex,
    v2: GpuVertex,
};

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let point_index = global_id.x;

    if (point_index >= num_points) {
        return;
    }

    // Generate your triangle vertices here.
    let triangle = generate_triangle(points[point_index]);

    let vertex_index = atomicAdd(
        &draw_args.vertex_count,
        3u
    );

    vertices[vertex_index + 0u] = triangle.v0;
    vertices[vertex_index + 1u] = triangle.v1;
    vertices[vertex_index + 2u] = triangle.v2;
}

fn generate_triangle(point: GpuPoint) -> Triangle {
    let center_world = vec4<f32>(point.position.x, -point.position.y, 0.0, 1.0);
    let center_ndc = view_transform * center_world;

    let half_size = 0.05;

    let v1 = center_ndc + vec4<f32>(-half_size, -half_size, 0.0, 0.0);
    let v2 = center_ndc + vec4<f32>(0.0, half_size, 0.0, 0.0);
    let v3 = center_ndc + vec4<f32>(half_size, -half_size, 0.0, 0.0);

    let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    return Triangle(
        GpuVertex(v1, color),
        GpuVertex(v2, color),
        GpuVertex(v3, color),
    );
}
