// OpCodes
pub const OP_CONST: u32 = 0;
pub const OP_X: u32 = 1;
pub const OP_X_POLY: u32 = 2;
pub const OP_ADD: u32 = 3;
pub const OP_MUL: u32 = 4;
pub const OP_POW: u32 = 5;
pub const OP_COS: u32 = 6;
pub const OP_SIN: u32 = 7;
pub const OP_TAN: u32 = 8;
pub const OP_LOG: u32 = 9;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Instruction {
    pub opcode: u32,
    pub a: f32,
    pub b: f32,
}

#[macro_export]
macro_rules! inst {
    () => {
        Instruction {
            opcode: OP_CONST,
            a: 0.,
            b: 0.,
        }
    };
    ($opcode:expr) => {
        Instruction {
            opcode: $opcode,
            a: 0.,
            b: 0.,
        }
    };
    ($opcode:expr, $a:expr) => {
        Instruction {
            opcode: $opcode,
            a: $a,
            b: 0.,
        }
    };
    ($opcode:expr, $a:expr, $b:expr) => {
        Instruction {
            opcode: $opcode,
            a: $a,
            b: $b,
        }
    };
}
