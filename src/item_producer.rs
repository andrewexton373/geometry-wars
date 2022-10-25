use crate::recipe::Recipe;

pub trait ItemProducer {
    fn new() -> Self;
    fn recipes(&self) -> Vec<Recipe>;
    fn currently_processing(&self) -> Option<Recipe>;
    fn remaining_processing_percent(&self) -> Option<f32>;
    fn remaining_processing_time(&self) -> Option<f32>;
}
