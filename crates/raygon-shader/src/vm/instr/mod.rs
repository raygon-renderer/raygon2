use thermite::*;

use crate::vm;

use vm::stack::Stack;

pub mod binary;
pub mod compare;
pub mod unary;

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
}

impl Instruction {
    pub fn eval<S: Simd>(self, stack: &mut Stack<S>) {
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
            _ => unimplemented!(),
        }
    }
}
