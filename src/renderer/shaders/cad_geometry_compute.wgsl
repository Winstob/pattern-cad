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
var<storage, read> points: array<GpuPoint>;

@group(0) @binding(3)
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
    //let center = view_transform(point.position);
    //let center = point.position;
    //let center = vec2<f32>(0.0, 0.0);
    let center = vec2<f32>(point.position.x, -point.position.y) / 100.0;

    let half_size = 0.05;

    let v1 = center + vec2<f32>(-half_size, -half_size);
    let v2 = center + vec2<f32>(0.0, half_size);
    let v3 = center + vec2<f32>(half_size, -half_size);

    let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    return Triangle(
        GpuVertex(vec4<f32>(v1, 0.0, 1.0), color),
        GpuVertex(vec4<f32>(v2, 0.0, 1.0), color),
        GpuVertex(vec4<f32>(v3, 0.0, 1.0), color)
    );
}
