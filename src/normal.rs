use std::f32::consts::TAU;

#[derive(Debug)]
pub struct Normal
{
    mu: f32,
    sigma: f32,
}

impl Normal
{
    /// Standard normal distribution with mu=0 and sigma=1.
    pub const fn standard() -> Self
    {
        Self {
            mu: 0.0,
            sigma: 1.0,
        }
    }

    pub const fn value(&self, x: f32) -> f32
    {
        (-(x - self.mu).powi(2) / (2.0 * self.sigma.powi(2))).exp() / (self.sigma * TAU.sqrt())
    }
}
