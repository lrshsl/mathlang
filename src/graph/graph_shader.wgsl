
const STROKE_WIDTH: f32 = 1.;

struct Op {
	opcode: u32,
	operands: vec2f,
}

struct Uniforms {
	resolution: vec2f,
	center: vec2f,
	program: array<Op, 6>,
	scale: f32,
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

// Interprets a mathematical operation on the GPU
fn eval_function(x: f32) -> f32 {
    var stack: array<f32, 16>; // simple fixed-size stack
    var sp: u32 = 0;

    for (var i: u32 = 0u; i < 6; i = i + 1u) {
        let op = u.program[i];

        switch op.opcode {
            case 0u: { // CONST
                stack[sp] = op.operand;
                sp = sp + 1u;
            }
            case 1u: { // X
                stack[sp] = x;
                sp = sp + 1u;
            }
            case 2u: { // SIN
                stack[sp - 1u] = sin(stack[sp - 1u]);
            }
            case 3u: { // POW
                let b = stack[sp - 1u];
                sp = sp - 1u;
                let a = stack[sp - 1u];
                stack[sp - 1u] = pow(a, b);
            }
            case 4u: { // ADD
                let b = stack[sp - 1u];
                sp = sp - 1u;
                let a = stack[sp - 1u];
                stack[sp - 1u] = a + b;
            }
            case 5u: { // MUL
                let b = stack[sp - 1u];
                sp = sp - 1u;
                let a = stack[sp - 1u];
                stack[sp - 1u] = a * b;
            }
				default: {}
            // More ops like COS, LOG, EXP, etc.
        }
    }

    return stack[0];
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
	let d = u.scale * STROKE_WIDTH;

	var p = u.center + (in.position.xy - u.resolution * .5) * u.scale;
	p.y = -p.y; // invert y axis for mathematics

	let curve_y = eval_function(p.x);

	// Derivative -> normal vector -> distance from curve
	//let dx = 0.001;
	//let curve_yd = eval_function(p.x + dx);
	//let tangent = normalize(vec2f(dx, curve_yd - curve_y));
	//let normal = vec2f(-tangent.y, tangent.x);

	//let dist = abs(dot(p - vec2f(p.x, curve_y), normal));
	let dist = abs(p.y - curve_y);

	if dist < d {
		return vec4f(1., 1., 1., 1.);
	}
	if abs(p.x) < d || abs(p.y) < d {
		return vec4f(0.1, 0.1, 0.1, 1.);
	}

	return vec4f(0., 0., 0., 1.);
}

