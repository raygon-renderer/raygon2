use thermite::backends::avx2::AVX2;
use thermite::*;

use raygon_shader::vm;
use vm::{instr::Instruction, rom::ROM, stack::Stack};

#[no_mangle]
#[target_feature(enable = "avx2,fma")]
#[inline(never)]
pub unsafe fn eval_instruction(isntr: Instruction, stack: &mut Stack<AVX2>, rom: &ROM) {
    isntr.eval(stack, rom);
}

fn main() {}

// cargo rustc --example asm --release -- -C target-cpu=native -C opt-level=3 --emit asm
