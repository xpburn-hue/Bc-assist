pub mod g1;
pub mod g7;
pub mod lookup;
pub mod table;

pub trait DragFunction {
    fn retardation(&self, velocity_fps: f64) -> f64;
}

#[cfg(test)]
mod tests;
