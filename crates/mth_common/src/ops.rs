// OpCodes
pub const OP_CONST: u32 = 0;
pub const OP_X: u32 = 1;
pub const OP_X_POLY: u32 = 2;
pub const OP_ADD: u32 = 3;
pub const OP_SUB: u32 = 10;
pub const OP_MUL: u32 = 4;
pub const OP_DIV: u32 = 11;
pub const OP_POW: u32 = 5;
pub const OP_COS: u32 = 6;
pub const OP_SIN: u32 = 7;
pub const OP_TAN: u32 = 8;
pub const OP_LOG: u32 = 9;
pub const OP_EQ: u32 = 12;
pub const OP_LT: u32 = 14; // <
pub const OP_LE: u32 = 15; // <=
pub const OP_GT: u32 = 16; // >
pub const OP_GE: u32 = 17; // >=
pub const OP_NE: u32 = 18; // !=
pub const OP_Y: u32 = 13;

#[derive(Copy, Clone, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Instruction {
    pub opcode: u32,
    pub a: f32,
    pub b: f32,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            opcode: OP_CONST,
            a: 0.,
            b: 0.,
        }
    }
}

#[macro_export]
macro_rules! inst {
    () => {
        $crate::ops::Instruction {
            opcode: OP_CONST,
            a: 0.,
            b: 0.,
        }
    };
    ($opcode:expr) => {
        $crate::ops::Instruction {
            opcode: $opcode,
            a: 0.,
            b: 0.,
        }
    };
    ($opcode:expr, $a:expr) => {
        $crate::ops::Instruction {
            opcode: $opcode,
            a: $a,
            b: 0.,
        }
    };
    ($opcode:expr, $a:expr, $b:expr) => {
        $crate::ops::Instruction {
            opcode: $opcode,
            a: $a,
            b: $b,
        }
    };
}
