use std::ops::Range;

const WIDTH: f32 = 0.05;

/// Calculate the area under the curve `f` by using midpoints. Each bar has a
/// width that can be modified using the `width` argument. Smaller the `width`,
/// the better the outcome.
pub fn area_precision<F: Fn(f32) -> f32>(f: F, interval: Range<f32>, width: f32) -> f32
{
    let range = (interval.end - interval.start).abs();
    let n = (range / width).ceil();
    let width = range / n;
    let mut sum = 0.0;
    for i in 0..n as u32 {
        let x = interval.start + width * (i as f32 + 0.5);
        sum += f(x) * width;
    }
    sum
}

/// Calculate the area under the curve `f` by using midpoints. Default width of
/// each bar is set to 0.05.
pub fn area<F: Fn(f32) -> f32>(f: F, interval: Range<f32>) -> f32
{
    area_precision(f, interval, WIDTH)
}
