use glam::Vec2;

const CONST: u32 = 0;
const CONST_X: u32 = 1;
const SIN: u32 = 2;
const POW: u32 = 3;
const ADD: u32 = 4;
const MUL: u32 = 5;

#[macro_export]
macro_rules! op {
    () => {
        op!(CONST, 0.)
    };
    ($opcode:expr) => {
        Op {
            opcode: $opcode,
            operand: 0.,
            _pad: Vec2::ZERO,
        }
    };
    ($opcode:expr, $operand:expr) => {
        Op {
            opcode: $opcode,
            operand: $operand,
            _pad: Vec2::ZERO,
        }
    };
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Op {
    pub opcode: u32,
    pub operand: f32,
    pub _pad: Vec2, // 8 bytes of padding
}

