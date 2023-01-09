use std::ops::Bound;
use std::ops::RangeBounds;

use crate::calc;
use crate::normal::Normal;

const STD_NORMAL: Normal = Normal::standard();

/// Calculates the area under a standard normal distribution curve within the
/// given interval.
pub fn std_normal<R: RangeBounds<f32>>(interval: &R) -> f32
{
    let range = match (interval.start_bound(), interval.end_bound()) {
        (Bound::Included(s) | Bound::Excluded(s), Bound::Included(e) | Bound::Excluded(e)) => {
            *s..*e
        }
        _ => unimplemented!(),
    };
    calc::area(|x| STD_NORMAL.value(x), range)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn std_normal_table_values()
    {
        let ranges = vec![-1.0..1.0, -2.0..2.0, -3.0..3.0];
        let values = ranges.iter().map(std_normal).collect::<Vec<_>>();
        panic!("{:?}", values);
    }
}
