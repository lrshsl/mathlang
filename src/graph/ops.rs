
pub const OP_CONST: u32 = 0;
pub const OP_INPUT_X: u32 = 1;
pub const OP_ADD: u32 = 3;
pub const OP_MUL: u32 = 4;
pub const OP_SIN: u32 = 5;
pub const OP_SUB: u32 = 6;
pub const OP_DIV: u32 = 7;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Instruction {
    pub opcode: u32,
    pub a: f32,
    pub b: f32,
}

#[macro_export]
macro_rules! inst {
    ($opcode:expr, $a:expr $(, $b:expr)? ) => {
        Instruction {
            opcode: $opcode,
            a: $a,
            b: $($b)?
        }
    };
}
