use std::ops::Range;

const WIDTH: f64 = 0.05;

/// Approximate the area under the curve `f` by using the Trapezoid Rule. Each
/// segment has a width that can be modified using the `width` argument. Smaller
/// the `width`, the better the approximation.
pub fn trapezoid_w<F: Fn(f64) -> f64>(f: F, interval: Range<f64>, width: f64) -> f64
{
    // calculate the actual n and width;
    let range = (interval.end - interval.start).abs();
    let n = (range / width).ceil();
    let width = range / n;

    // calculate the sum of the y values of the function f at the points. the
    // reason we count the points other than the start and the end twice can be
    // seen when looking at the formula for the trapezoid rule.
    let sum = f(interval.start)
        + (1..n as u32)
            .map(|i| 2.0 * f(interval.start + i as f64 * width))
            .sum::<f64>()
        + f(interval.end);

    (sum * width) / 2.0
}

/// Approximate the area under the curve `f` by using the Trapezoid Rule.
/// Default width of each segment is set to 0.05.
pub fn trapezoid<F: Fn(f64) -> f64>(f: F, interval: Range<f64>) -> f64
{
    trapezoid_w(f, interval, WIDTH)
}

#[cfg(test)]
mod tests
{
    use rustat::normal::Normal;

    use super::*;

    #[test]
    fn trapezoid_tests()
    {
        // The normal distribution is used as a test.
        let dist = Normal::standard();
        let pdf = |x| dist.pdf(x);
        // Using smaller width improves the approximation.
        let width = 0.05;

        // The area under the standard normal distribution graph from x0 to x1. The area
        // values are obtained from scipy.stats.norm.cdf
        let vectors = vec![
            // (x0..x1, area)
            (-2.33..-2.12, 0.007099947088468542),
            (-3.83..1.60, 0.9451366366709532),
            (-1.99..0.31, 0.5984240540718074),
            (-1.51..2.89, 0.9325520787788957),
            (1.73..2.77, 0.039012322980829905),
        ];
        // Calculate the difference between the CDF values and the trapezoid values.
        let diff = vectors
            .iter()
            .map(|(r, v)| (trapezoid_w(pdf, r.clone(), width) - v).abs())
            .collect::<Vec<_>>();

        // Pass threshold is currently set to 1e-4. Any lower trips the assert. This
        // threshold should suffice for now, but take a look at the implementation of
        // the trapezoid function.
        let pass = diff.iter().filter(|x| **x > 1e-4).next().is_none();
        assert!(
            pass,
            "TRAPEZOID values are not within acceptable range: {:#?}",
            diff
        );
    }
}
