use std::ops::Bound;
use std::ops::RangeBounds;

use crate::calc;
use crate::normal::Normal;

/// Calculates the area under a normal distribution curve within the given
/// interval.
pub fn cdf_normal<R: RangeBounds<f32>>(normal: Normal, interval: R) -> f32
{
    let range = match (interval.start_bound(), interval.end_bound()) {
        (Bound::Included(s) | Bound::Excluded(s), Bound::Included(e) | Bound::Excluded(e)) => {
            *s..*e
        }
        (Bound::Unbounded, Bound::Excluded(e) | Bound::Included(e)) => -15.0..*e,
        (Bound::Excluded(s) | Bound::Included(s), Bound::Unbounded) => *s..15.0,
        (Bound::Unbounded, Bound::Unbounded) => -15.0..15.0,
    };
    calc::area(|x| normal.pdf(x), range)
}

/// Calculates the area under a standard normal distribution curve within the
/// given interval.
pub fn cdf_std_normal<R: RangeBounds<f32>>(interval: R) -> f32
{
    cdf_normal(Normal::standard(), interval)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn std_normal_table_values()
    {
        let values = vec![
            (cdf_std_normal(0.0..1.0), 0.34134),
            (cdf_std_normal(0.0..2.0), 0.47725),
            (cdf_std_normal(0.0..3.0), 0.49865),
            (cdf_std_normal(-1.0..2.0), 0.34134 + 0.47725),
            (cdf_std_normal(..0.0), 0.5),
        ];
        let diff = values
            .iter()
            .map(|(v, t)| (v - t).abs())
            .collect::<Vec<_>>();
        let acceptable = diff.iter().filter(|x| **x > 0.001).next().is_none();
        assert!(acceptable, "Values are not withing acceptable range")
    }
}
