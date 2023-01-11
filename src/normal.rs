use std::f64::consts::SQRT_2;
use std::f64::consts::TAU;
use std::fmt::Display;
use std::ops::Bound;
use std::ops::RangeBounds;

use crate::erf::erf;

#[derive(Debug)]
pub struct Normal
{
    mu: f64,
    sigma: f64,
}

impl Display for Normal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "N(μ={}, σ={})", self.mu(), self.sigma())
    }
}

impl Normal
{
    /// Normal distribution with μ and σ.
    ///
    /// - `mu` is the location of the normal distribution graph along the
    ///   x-axis.
    /// - `sigma` is the standard deviation of the normal distribution graph.
    ///   Standard deviation cannot be negative. Absolute value of the sigma is
    ///   used.
    pub const fn new(mu: f64, sigma: f64) -> Self
    {
        Self { mu, sigma }
    }

    /// Standard normal distribution with μ=0 and σ=1.
    pub const fn standard() -> Self
    {
        Self::new(0.0, 1.0)
    }

    pub fn mu(&self) -> f64
    {
        self.mu
    }

    pub fn sigma(&self) -> f64
    {
        self.sigma.abs()
    }

    /// Cumulative distribution function of the normal distribution.
    ///
    /// Returns the area under the normal distribution graph from -∞ to `x`.
    pub fn cdf(&self, x: f64) -> f64
    {
        if x.is_infinite() & x.is_sign_negative() {
            0.0
        } else if x.is_infinite() & x.is_sign_positive() {
            1.0
        } else if x.is_finite() {
            0.5 * (1.0 + erf((x - self.mu) / (self.sigma() * SQRT_2)))
        } else {
            f64::NAN
        }
    }

    /// The area under the graph of the normal distribution within the given
    /// range.
    pub fn p<R: RangeBounds<f64>>(&self, interval: R) -> f64
    {
        let e = match interval.end_bound() {
            Bound::Excluded(x) | Bound::Included(x) => *x,
            _ => f64::INFINITY,
        };
        let s = match interval.start_bound() {
            Bound::Excluded(x) | Bound::Included(x) => *x,
            _ => f64::NEG_INFINITY,
        };

        self.cdf(e) - self.cdf(s)
    }

    /// Probability density function of the normal distribution.
    ///
    /// The value of the graph at `x`.
    pub fn pdf(&self, x: f64) -> f64
    {
        (-(x - self.mu).powi(2) / (2.0 * self.sigma().powi(2))).exp() / (self.sigma() * TAU.sqrt())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn cdf_tests()
    {
        let dist = Normal::standard();
        let vectors = vec![
            (-0.228756, 0.409525),
            (-0.491818, 0.311434),
            (-1.266972, 0.102572),
            (1.389189, 0.917621),
            (0.659599, 0.745236),
            (1.022284, 0.846685),
            (-1.320967, 0.093246),
            (0.477800, 0.683593),
            (-1.282607, 0.099804),
            (-1.159063, 0.123205),
        ];

        let diff = vectors
            .iter()
            .map(|(x, y)| y - dist.cdf(*x))
            .collect::<Vec<f64>>();
        let pass = diff.iter().filter(|x| **x > 1e-6).next().is_none();
        assert!(
            pass,
            "CDF values are not within acceptable range: {:#?}",
            diff
        );
    }

    #[test]
    fn std_normal_table_values()
    {
        let dist = Normal::standard();
        let vectors = vec![
            (dist.p(0.0..1.0), 0.34134),
            (dist.p(0.0..2.0), 0.47725),
            (dist.p(0.0..3.0), 0.49865),
            (dist.p(-1.0..2.0), 0.34134 + 0.47725),
            (dist.p(..0.0), 0.5),
        ];
        let diff = vectors
            .iter()
            .map(|(v, t)| (v - t).abs())
            .collect::<Vec<_>>();
        let pass = diff.iter().filter(|x| **x > 1e-3).next().is_none();
        assert!(
            pass,
            "STDNORM P values are not withing acceptable range: {:#?}",
            vectors
        )
    }
}
