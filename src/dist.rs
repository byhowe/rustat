use std::ops::Bound;
use std::ops::RangeBounds;

use crate::calc;
use crate::normal::Normal;

const STD_NORMAL: Normal = Normal::standard();

/// Calculates the area under a standard normal distribution curve within the
/// given interval.
pub fn std_normal<R: RangeBounds<f32>>(interval: R) -> f32
{
    let range = match (interval.start_bound(), interval.end_bound()) {
        (Bound::Included(s) | Bound::Excluded(s), Bound::Included(e) | Bound::Excluded(e)) => {
            *s..*e
        }
        _ => unimplemented!(),
    };
    calc::area(|x| STD_NORMAL.value(x), range)
}
