
struct Uniforms {
	resolution: vec2f,
	center: vec2f,
	scale: f32,
	f2: f32,
	f1: f32,
	f0: f32,
}

@group(0) @binding(0) var<uniform> u: Uniforms;

struct VertexIn {
	@builtin(vertex_index) vertex_index: u32,
}

struct VertexOut {
	@builtin(position) position: vec4f,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
	let uv = vec2f(vec2u((in.vertex_index << 1) & 2, in.vertex_index & 2));
	let position = vec4f(uv * 2. - 1., 0., 1.);
	return VertexOut(position);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
	let p = u.center + (in.position.xy - u.resolution * .5) * u.scale;

	let res = u.f2 * p.x * p.x + u.f1 * p.x + u.f0;

	if abs(res - p.y) < 1e-3 {
		return vec4f(1., 1., 1., 1.);
	}
	return vec4f(0., 0., 0., 1.);
}

// vim: ft=rs
