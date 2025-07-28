
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
const OP_MUL: u32 = 4;
const OP_POW: u32 = 5;
const OP_COS: u32 = 6;
const OP_SIN: u32 = 7;
const OP_TAN: u32 = 8;
const OP_LOG: u32 = 9;

struct Uniforms {
    resolution: vec2f,
    center: vec2f,
    scale: f32,
    _pad: f32,
    viewport_origin: vec2f,
    _pad2: vec2f,
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

    for (var i: u32 = 0u; i < 6; i = i + 1u) {
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
        case OP_MUL: { // stack[-2] *= stack[-1]
            let b = stack[sp - 1u];
            sp = sp - 1u;
            let a = stack[sp - 1u];
            stack[sp - 1u] = a * b;
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

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    let d = u.scale * STROKE_WIDTH;

    let point = u.center + (in.position.xy - u.resolution * 0.5) * u.scale;
    var p = point + u.viewport_origin / u.resolution;
    p.y = -p.y; // invert y axis for mathematics

    let curve_y = eval_function(p.x);

    // Derivative -> normal vector -> distance from curve
    let dx = 0.001;
    let curve_yd = eval_function(p.x + dx);
    let tangent = normalize(vec2f(dx, curve_yd - curve_y));
    let normal = vec2f(-tangent.y, tangent.x);

    let dist = dot(p - vec2f(p.x, curve_y), normal);

    if abs(dist) < d {
        return vec4f(1., 1., 1., 1.);
    }

    // x and y Axis
    if abs(p.x) < d || abs(p.y) < d {
        return vec4f(0.1, 0.1, 0.1, 1.);
    }

    return vec4f(0., 0., 0., 1.);
}


    // vim: et sw=4 sts=4 ts=4
