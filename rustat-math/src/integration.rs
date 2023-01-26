use std::ops::Range;

const WIDTH: f64 = 0.05;

/// Internal enum that is passed to function as a const parameter. The method is
/// decided based on the enum passed. The Rust compiler is smart enough to
/// remove dead code when compiling the function.
#[derive(Debug, PartialEq, Eq)]
enum Integration
{
    /// Divide the curve into n bars and calculate the sum of their areas.
    /// Height is based on the midpoint of each bar.
    Midpoint,
    /// Divide the curve into n trapezoids. Height of the left and the right
    /// side of the trapezoids are used.
    Trapezoid,
    Simpsons,
}

fn integration<const M: Integration, F: Fn(f64) -> f64>(
    f: F,
    interval: Range<f64>,
    width: f64,
) -> f64
{
    // calculate the actual n and width;
    let range = (interval.end - interval.start).abs();
    let n = (range / width).ceil();
    let width = range / n;

    match M {
        Integration::Midpoint => {
            (0..n as u32)
                .map(|i| f(interval.start + (i as f64 + 0.5) * width))
                .sum::<f64>()
                * width
        }
        Integration::Trapezoid => {
            // calculate the sum of the y values of the function f at the points. the
            // reason we count the points other than the start and the end twice can be
            // seen when looking at the formula for the trapezoid rule.
            let sum = f(interval.start)
                + 2.0
                    * (1..n as u32)
                        .map(|i| f(interval.start + i as f64 * width))
                        .sum::<f64>()
                + f(interval.end);
            (sum * width) / 2.0
        }
        Integration::Simpsons => todo!(),
    }
}

/// Approximate the area under the curve `f` by using the Midpoint Rule. Each
/// bar has a width that can be modified using the `width` argument. Smaller
/// the `width`, the better the approximation.
pub fn midpoint_w<F: Fn(f64) -> f64>(f: F, interval: Range<f64>, width: f64) -> f64
{
    integration::<{ Integration::Midpoint }, _>(f, interval, width)
}

/// Approximate the area under the curve `f` by using the Midpoint Rule.
/// Default width of each bar is set to 0.05.
pub fn midpoint<F: Fn(f64) -> f64>(f: F, interval: Range<f64>) -> f64
{
    midpoint_w(f, interval, WIDTH)
}

/// Approximate the area under the curve `f` by using the Trapezoid Rule. Each
/// segment has a width that can be modified using the `width` argument. Smaller
/// the `width`, the better the approximation.
pub fn trapezoid_w<F: Fn(f64) -> f64>(f: F, interval: Range<f64>, width: f64) -> f64
{
    integration::<{ Integration::Trapezoid }, _>(f, interval, width)
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

    // This is a generic test functin that uses the `Integration` enum to decide
    // which method to use when numerically integrating. So, this keeps the vectors
    // and the values the same for all methods.
    fn integration_test<const M: Integration>()
    {
        // The normal distribution is used as a test.
        let dist = Normal::standard();
        let pdf = |x| dist.pdf(x);
        // Using smaller width improves the approximation.
        let width = 0.0005;

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
        // Calculate the difference between the CDF values and the integration values.
        let diff = vectors
            .iter()
            .map(|(r, v)| (integration::<M, _>(pdf, r.clone(), width) - v).abs())
            .collect::<Vec<_>>();

        // Pass threshold is currently set to 1e-8. Any lower trips the assert. This
        // threshold should suffice for now, but take a look at the implementation of
        // the integration function.
        let pass = diff.iter().filter(|x| **x > 1e-8).next().is_none();
        assert!(
            pass,
            "{} values are not within acceptable range: {:#?}",
            match M {
                Integration::Midpoint => "MIDPOINT",
                Integration::Trapezoid => "TRAPEZOID",
                Integration::Simpsons => "SIMPSONS",
            },
            diff
        );
    }

    #[test]
    fn trapezoid()
    {
        integration_test::<{ Integration::Trapezoid }>()
    }

    #[test]
    fn midpoint()
    {
        integration_test::<{ Integration::Midpoint }>()
    }
}
