use std::ops::Range;

const WIDTH: f32 = 0.05;

/// Calculate the area under the curve `f` by using midpoints.
pub fn area<F: Fn(f32) -> f32>(f: F, interval: Range<f32>) -> f32
{
    let range = (interval.end - interval.start).abs();
    let n = (range / WIDTH).ceil();
    let width = range / n;
    let mut sum = 0.0;
    for i in 0..n as u32 {
        let x = interval.start + width * (i as f32 + 0.5);
        sum += f(x) * width;
    }
    sum
}
