use crate::sampling::Sampler;

pub struct ShaderState {
    pub sampler: Box<dyn Sampler>,
}
