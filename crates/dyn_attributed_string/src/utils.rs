use std::ops::Range;

pub fn is_range_within(a: &Range<usize>, b: &Range<usize>) -> bool {
    b.start <= a.start && a.end <= b.end
}
