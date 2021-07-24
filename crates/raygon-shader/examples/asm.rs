use thermite::backends::avx2::AVX2;
use thermite::*;

use raygon_shader::vm;
use vm::{instr::Instruction, stack::Stack};

#[no_mangle]
#[target_feature(enable = "avx2,fma")]
#[inline(never)]
pub unsafe fn eval_instruction(isntr: Instruction, stack: &mut Stack<AVX2>) {
    isntr.eval(stack);
}

fn main() {}

// cargo rustc --example asm --release -- -C target-cpu=native -C opt-level=3 --emit asm
