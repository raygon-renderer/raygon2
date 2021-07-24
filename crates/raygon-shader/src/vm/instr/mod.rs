use thermite::*;

use crate::vm;

use vm::stack::Stack;

pub mod binary;

pub enum Instruction {
    NoOp,
    ScalarBinary(binary::BinaryOp),
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
            //Instruction::ScalarBinary(op) => stack.reduce_n(|[a, b]| op.eval::<S>(a, b)),
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
