use std::ops::Range;

pub fn is_range_within(a: &Range<usize>, b: &Range<usize>) -> bool {
    b.start <= a.start && a.end <= b.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            is_range_within(&Range { start: 6, end: 7 }, &Range { start: 5, end: 10 }),
            true
        );
    }
}
