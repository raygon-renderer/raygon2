#[macro_export]
macro_rules! impl_deepsizeof_pod {
    ($t:ident) => {
        impl deepsize::DeepSizeOf for $t
        where
            Self: Sized + 'static,
        {
            #[inline(always)]
            fn deep_size_of_children(&self, _: &mut deepsize::Context) -> usize {
                0
            }
        }
    };
}
