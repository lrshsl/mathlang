
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
const OP_EQ: u32 = 12;
const OP_LT: u32 = 14;  // <
const OP_LE: u32 = 15;  // <=
const OP_GT: u32 = 16;  // >
const OP_GE: u32 = 17;  // >=
const OP_NE: u32 = 18;  // !=
const OP_Y: u32 = 13;

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

// Helper function to check if instructions contain equality operations
fn is_boolean_condition() -> bool {
    for (var i: u32 = 0u; i < u.instruction_count; i = i + 1u) {
        if instructions[i].opcode == OP_EQ {
            return true;
        }
    }
    return false;
}

fn is_on_curve(x: f32, y: f32, d: f32) -> bool {
    let curve_y = eval_function(x, 0.0); // y coordinate not used for 1D functions

    // Calculate distance from curve //

    // Central difference (+dx -dx) for more precision
    let dx = 0.001 * max(1.0, x);
    let dy = (eval_function(x + dx, 0.0) - eval_function(x - dx, 0.0)) / (2.0 * dx);
    
    // Vertical distance to the curve
    let vertical_dist = y - curve_y;

    // Normalize
    // We divide by length of gradient vector vec2f(1.0, dy)
    // This turns vertical distance into perpendicular distance
    let dist = abs(vertical_dist) / sqrt(1.0 + dy * dy);

    return dist < d;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // ~= thickness of lines
    let d = u.pixel_ratio * STROKE_WIDTH;

    let local_pos = in.position.xy - u.viewport_origin;
    let scaled_pos = (local_pos - u.viewport_size * 0.5) * u.pixel_ratio;

    // Draw viewport origin indicator
    if abs(scaled_pos.x) < d && abs(scaled_pos.y) < d {
        return vec4f(0.5, 0.5, 0.5, 1.);
    }

    var p = scaled_pos + u.pan_offset;
    p.y = -p.y; // invert y axis for mathematics

    // Draw function
    var color = 0.0;
    if is_boolean_condition() {

        // Boolean condition rendering
        color = eval_function(p.x, p.y);

    } else if is_on_curve(p.x, p.y, d) {

        // Traditional curve rendering
        color = 1.0;

    } else if abs(p.x) < d || abs(p.y) < d {

        // x and y axis
        color = 0.3;
    }

    return vec4f(color, color, color, 1.0);
}

fn spow(a: f32, b: f32) -> f32 {
    if a >= 0.0 {
        return pow(a, b);
    }

    let abs_pow = pow(-a, b);

    // If y is even, result is positive; else negative
    let b_is_even = fract(b * 0.5) == 0.0;
    return select(-abs_pow, abs_pow, b_is_even);
}

// Extracted function to handle individual instruction execution
fn execute_instruction(op: Instruction, x: f32, y: f32, sp: ptr<function, u32>, stack: ptr<function, array<f32, 16>>) {
    switch op.opcode {
        case OP_CONST: { // => a
            stack[*sp] = op.a;
            *sp = *sp + 1u;
        }
        case OP_X: { // => x
            stack[*sp] = x;
            *sp = *sp + 1u;
        }
        case OP_Y: { // => y
            stack[*sp] = y;
            *sp = *sp + 1u;
        }
        case OP_X_POLY: { // => a * (x**b)
            stack[*sp] = op.a * spow(x, op.b);
            *sp = *sp + 1u;
        }
        case OP_ADD: { // stack[-2] += stack[-1]
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = a + b;
        }
        case OP_SUB: { // stack[-2] -= stack[-1]
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = a - b;
        }
        case OP_MUL: { // stack[-2] *= stack[-1]
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = a * b;
        }
        case OP_DIV: { // stack[-2] /= stack[-1]
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = a / b;
        }
        case OP_POW: { // stack[-2] **= stack[-1]
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = spow(a, b);
        }
        case OP_COS: { // cos(stack[-1])
            stack[*sp - 1u] = cos(stack[*sp - 1u]);
        }
        case OP_SIN: { // sin(stack[-1])
            stack[*sp - 1u] = sin(stack[*sp - 1u]);
        }
        case OP_TAN: { // tan(stack[-1])
            stack[*sp - 1u] = tan(stack[*sp - 1u]);
        }
        case OP_LOG: { // log(stack[-1])
            stack[*sp - 1u] = log(stack[*sp - 1u]);
        }
        case OP_EQ: { // stack[-2] == stack[-1] ? 1.0 : 0.0
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = step(abs(a - b), 0.001);
        }
        case OP_NE: { // stack[-2] != stack[-1] ? 1.0 : 0.0
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = 1.0 - step(abs(a - b), 0.001);
        }
        case OP_LT: { // stack[-2] < stack[-1] ? 1.0 : 0.0
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = step(b - a, 0.001);
        }
        case OP_LE: { // stack[-2] <= stack[-1] ? 1.0 : 0.0
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = step(b - a + 0.001, 0.001);
        }
        case OP_GT: { // stack[-2] > stack[-1] ? 1.0 : 0.0
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = step(a - b, 0.001);
        }
        case OP_GE: { // stack[-2] >= stack[-1] ? 1.0 : 0.0
            let b = stack[*sp - 1u];
            *sp = *sp - 1u;
            let a = stack[*sp - 1u];
            stack[*sp - 1u] = step(a - b + 0.001, 0.001);
        }
        default: {}
    }
}

// Unified function for both 1D and 2D evaluation
fn eval_function(x: f32, y: f32) -> f32 {
    var stack: array<f32, 16>; // simple fixed-size stack
    var sp: u32 = 0;

    for (var i: u32 = 0u; i < u.instruction_count; i = i + 1u) {
        let op = instructions[i];
        execute_instruction(op, x, y, &sp, &stack);
    }
    return stack[0];
}


    // vim: et sw=4 sts=4 ts=4
