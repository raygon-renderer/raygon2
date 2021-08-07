use thermite::*;

macro_rules! decl_builtin_callbacks {
    ($($name:ident($($arg_name:ident: $arg:ty),*)),*) => {
        #[repr(C)]
        pub struct BuiltinCallbacks<S: Simd> {
            $(
                pub $name: unsafe extern "C" fn ($($arg_name: *mut $arg),* )
            ),*
        }
    }
}

decl_builtin_callbacks! {
    vector_sin(input: Vf32<S>, out: Vf32<S>),
    vector_cos(input: Vf32<S>, out: Vf32<S>),
    vector_tan(input: Vf32<S>, out: Vf32<S>),

    vector_random(out: Vf32<S>),

    read_texture(id: u32, u: Vf32<S>, v: Vf32<S>, out_r: Vf32<S>, out_g: Vf32<S>, out_b: Vf32<S>, out_a: Vf32<S>)
}
