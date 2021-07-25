use thermite::*;

use crate::vm;

use vm::{rom::ROM, stack::Stack};

pub mod binary;
pub mod compare;
pub mod unary;

macro_rules! decl_wrappers {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(transparent)]
            pub struct $name(pub u8);

            impl $name {
                #[inline]
                pub fn new(index: usize) -> $name {
                    debug_assert!(index < 256);

                    $name(index as u8)
                }
            }

            impl From<$name> for usize {
                #[inline(always)]
                fn from(index: $name) -> usize {
                    index.0 as usize
                }
            }

            impl From<usize> for $name {
                #[inline]
                fn from(index: usize) -> $name {
                    $name::new(index)
                }
            }
        )*
    }
}

decl_wrappers!(FloatIndex, VectorIndex, TextureIndex, CurveIndex, ColorModelIndex);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    NoOp,
    ScalarBinary(binary::BinaryOp),
    ScalarUnary(unary::UnaryOp),
    ScalarCompare(compare::CompareMode),
    VectorBinary(binary::BinaryOp),
    VectorUnary(unary::UnaryOp),
    VectorCompare(compare::CompareMode),
    VectorSum,
    VectorProduct,
    VectorMin,
    VectorMax,
    VectorSplat,
    CopyScalar(u8),
    CopyVector(u8),
    Curve(CurveIndex),
}

impl Instruction {
    pub fn eval<S: Simd>(self, stack: &mut Stack<S>, rom: &ROM) {
        match self {
            Instruction::NoOp => {}

            Instruction::ScalarUnary(op) => stack.peek_one_mut(|x| *x = op.eval::<S>(*x)),
            Instruction::ScalarBinary(op) => stack.reduce(|[a, b]| op.eval::<S>(a, b)),

            Instruction::VectorUnary(op) => stack.map(|[x, y, z]| [op.eval::<S>(x), op.eval::<S>(y), op.eval::<S>(z)]),
            Instruction::VectorBinary(op) => {
                // map two 3-vectors into one 3-vector, where A is the top-most vector
                stack.map(|[xb, yb, zb, xa, ya, za]: [Vf32<S>; 6]| [op.eval::<S>(xa, xb), op.eval::<S>(ya, yb), op.eval::<S>(za, zb)]);
            }

            Instruction::ScalarCompare(mode) => stack.reduce(|[a, b]| mode.compare::<S>(a, b)),
            Instruction::VectorCompare(mode) => stack.map(|[xb, yb, zb, xa, ya, za]: [Vf32<S>; 6]| {
                [mode.compare::<S>(xa, xb), mode.compare::<S>(ya, yb), mode.compare::<S>(za, zb)]
            }),

            Instruction::VectorSum => stack.reduce(|[x, y, z]| x + y + z),
            Instruction::VectorProduct => stack.reduce(|[x, y, z]| x * y * z),
            Instruction::VectorMin => stack.reduce(|[x, y, z]| x.min(y).min(z)),
            Instruction::VectorMax => stack.reduce(|[x, y, z]| x.max(y).max(z)),
            Instruction::VectorSplat => stack.map(|[x]| [x, x, x]),

            Instruction::CopyScalar(count) => {
                let value = stack.peek_one(|x| *x);
                stack.push(count as usize, |head| head.fill(value));
            }
            Instruction::CopyVector(count) => {
                let xyz = stack.peek_n::<3>();

                stack.push(count as usize * 3, |head| {
                    let mut i = 0;
                    for l in head {
                        *l = xyz[i];

                        // cycle through x, y, z, more efficient than %/mod 3
                        i += 1;
                        if i == 3 {
                            i = 0;
                        }
                    }
                    // reference behavior
                    //s.chunks_exact_mut(3).for_each(|chunk| {
                    //    chunk.copy_from_slice(&xyz);
                    //})
                })
            }

            Instruction::Curve(idx) => stack.peek_one_mut(|x| *x = rom.get_curve(idx).eval::<S>(*x)),

            illegal_instruction => {
                #[inline(never)]
                #[cold]
                pub fn on_illegal_instruction(instruction: Instruction) -> ! {
                    panic!("Illegal ShaderVM instruction: {:?}", instruction)
                }

                on_illegal_instruction(illegal_instruction)
            }
        }
    }
}
