use std::f64::consts::SQRT_2;
use std::f64::consts::TAU;
use std::ops::Bound;
use std::ops::RangeBounds;

use crate::erf::erf;

#[derive(Debug)]
pub struct Normal
{
    mu: f64,
    sigma: f64,
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

    pub fn cdf(&self, x: f64) -> f64
    {
        0.5 * (1.0 + erf((x - self.mu) / (self.sigma * SQRT_2)))
    }

    pub fn p<R: RangeBounds<f64>>(&self, interval: R) -> f64
    {
        let e = match interval.end_bound() {
            Bound::Excluded(x) | Bound::Included(x) => Some(*x),
            _ => None,
        };
        let s = match interval.start_bound() {
            Bound::Excluded(x) | Bound::Included(x) => Some(*x),
            _ => None,
        };
        let upper = if let Some(x) = e { self.cdf(x) } else { 1.0 };
        let lower = if let Some(x) = s { self.cdf(x) } else { 0.0 };

        upper - lower
    }

    pub fn pdf(&self, x: f64) -> f64
    {
        (-(x - self.mu).powi(2) / (2.0 * self.sigma.powi(2))).exp() / (self.sigma * TAU.sqrt())
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
