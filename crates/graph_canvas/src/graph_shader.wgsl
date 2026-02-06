
const STROKE_WIDTH: f32 = 1.;

struct Instruction {
    opcode: u32,
    a: f32,
    b: f32,
}

// OpCodes
const OP_CONST: u32 = 0;
const OP_X: u32 = 1;
const OP_X_POLY: u32 = 2;
const OP_ADD: u32 = 3;
const OP_SUB: u32 = 10;
const OP_MUL: u32 = 4;
const OP_DIV: u32 = 11;
const OP_POW: u32 = 5;
const OP_COS: u32 = 6;
const OP_SIN: u32 = 7;
const OP_TAN: u32 = 8;
const OP_LOG: u32 = 9;

struct Uniforms {
    viewport_origin: vec2f,
    viewport_size: vec2f,
    pan_offset: vec2f,
    pixel_ratio: f32,
    instruction_count: u32,
};

@group(0) @binding(0)
var<uniform> u: Uniforms;

@group(1) @binding(0)
var<storage, read> instructions: array<Instruction>;

struct VertexIn {
    @builtin(vertex_index)
    vertex_index: u32,
}

struct VertexOut {
    @builtin(position)
    position: vec4f,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let uv = vec2f(vec2u((in.vertex_index << 1) & 2, in.vertex_index & 2));
    let position = vec4f(uv * 2. - 1., 0., 1.);
    return VertexOut(position);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // ~= thickness of lines
    let d = u.pixel_ratio * STROKE_WIDTH;

    let local_pos = in.position.xy - u.viewport_origin;
    let scaled_pos = (local_pos - u.viewport_size * 0.5) * u.pixel_ratio;

    if abs(scaled_pos.x) < d && abs(scaled_pos.y) < d {
        return vec4f(0.5, 0.5, 0.5, 1.);
    }

    var p = scaled_pos + u.pan_offset;
    p.y = -p.y; // invert y axis for mathematics


    // Maths //
    // Calculate the expected y
    let curve_y = eval_function(p.x);

    // Calculate distance from curve //

    // Central difference (+dx -dx) for more precision
    let dx = 0.001 * max(1.0, p.x);
    let dy = (eval_function(p.x + dx) - eval_function(p.x - dx)) / (2.0 * dx);
    
    // Vertical distance to the curve
    let vertical_dist = p.y - curve_y;

    // Normalize
    // We divide by the length of the gradient vector vec2f(1.0, dy)
    // This turns vertical distance into perpendicular distance
    let dist = abs(vertical_dist) / sqrt(1.0 + dy * dy);

    if abs(dist) < d {
        return vec4f(1., 1., 1., 1.);
    }

    // x and y axis
    if abs(p.x) < d || abs(p.y) < d {
        return vec4f(0.3, 0.3, 0.3, 1.);
    }

    return vec4f(0., 0., 0., 1.);
}

fn spow(a: f32, b: f32) -> f32 {
    if (a >= 0.0) {
        return pow(a, b);
    } else {
        let abs_pow = pow(-a, b);

        // If y is even, result is positive; else negative
        let b_is_even = fract(b * 0.5) == 0.0;
        return select(-abs_pow, abs_pow, b_is_even);
    }
}

// Interprets a mathematical operation on the GPU
fn eval_function(x: f32) -> f32 {
    var stack: array<f32, 16>; // simple fixed-size stack
    var sp: u32 = 0;

    for (var i: u32 = 0u; i < u.instruction_count; i = i + 1u) {
        let op = instructions[i];

        switch op.opcode {
        case OP_CONST: { // => a
            stack[sp] = op.a;
            sp = sp + 1u;
        }
        case OP_X: { // => x
            stack[sp] = x;
            sp = sp + 1u;
        }
        case OP_X_POLY: { // => a * (x**b)
            stack[sp] = op.a * spow(x, op.b);
            sp = sp + 1u;
        }
        case OP_ADD: { // stack[-2] += stack[-1]
            let b = stack[sp - 1u];
            sp = sp - 1u;
            let a = stack[sp - 1u];
            stack[sp - 1u] = a + b;
        }
        case OP_SUB: { // stack[-2] -= stack[-1]
            let b = stack[sp - 1u];
            sp = sp - 1u;
            let a = stack[sp - 1u];
            stack[sp - 1u] = a - b;
        }
        case OP_MUL: { // stack[-2] *= stack[-1]
            let b = stack[sp - 1u];
            sp = sp - 1u;
            let a = stack[sp - 1u];
            stack[sp - 1u] = a * b;
        }
        case OP_DIV: { // stack[-2] /= stack[-1]
            let b = stack[sp - 1u];
            sp = sp - 1u;
            let a = stack[sp - 1u];
            stack[sp - 1u] = a / b;
        }
        case OP_POW: { // stack[-2] **= stack[-1]
            let b = stack[sp - 1u];
            sp = sp - 1u;
            let a = stack[sp - 1u];
            stack[sp - 1u] = pow(a, b);
        }
        case OP_COS: { // cos(stack[-1])
            stack[sp - 1u] = cos(stack[sp - 1u]);
        }
        case OP_SIN: { // sin(stack[-1])
            stack[sp - 1u] = sin(stack[sp - 1u]);
        }
        case OP_TAN: { // tan(stack[-1])
            stack[sp - 1u] = tan(stack[sp - 1u]);
        }
        case OP_LOG: { // log(stack[-1])
            stack[sp - 1u] = log(stack[sp - 1u]);
        }
        default: {}
        }
    }
    return stack[0];
}


    // vim: et sw=4 sts=4 ts=4
