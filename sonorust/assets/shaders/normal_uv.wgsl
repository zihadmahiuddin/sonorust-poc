@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;
@group(2) @binding(0)
var<uniform> quad_corners: QuadCorners;
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

struct QuadCorners {
    top_left: vec2<f32>,
    top_right: vec2<f32>,
    bottom_left: vec2<f32>,
    bottom_right: vec2<f32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOut {
    var out: VertexOut;
    out.clip_position = view_proj * vec4(position, 1.0);
    out.uv = uv;
    return out;
}

@fragment
fn fragment(in: VertexOut) -> @location(0) vec4<f32> {
    let ndc = in.clip_position.xy / in.clip_position.w;
    let frag_pos = (ndc + vec2(1.0)) * 0.5;

    let uv = solve_uv(frag_pos,
                      quad_corners.bottom_left,
                      quad_corners.bottom_right,
                      quad_corners.top_left,
                      quad_corners.top_right);
    return textureSample(base_color_texture, base_color_sampler, uv);
}

fn solve_uv(P: vec2<f32>, P00: vec2<f32>, P10: vec2<f32>, P01: vec2<f32>, P11: vec2<f32>) -> vec2<f32> {
    let a = P00 - P10 - P01 + P11;
    let b = P10 - P00;
    let c = P01 - P00;
    let d = P - P00;

    let det = a.x * c.y - a.y * c.x;
    var v: f32 = 0.0;
    var u: f32 = 0.0;

    if abs(det) > 1e-6 {
        v = (d.x * b.y - d.y * b.x) / det;
        u = (d.x - c.x*v) / (b.x + a.x*v);
    } else {
        // fallback: simple linear interpolation
        u = d.x / (P10.x - P00.x);
        v = d.y / (P01.y - P00.y);
    }
    return vec2(u,v);
}
