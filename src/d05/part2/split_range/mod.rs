use std::cmp::{max, min};

use crate::d05::shared::{SeedRange, Transform};

/// Represents a range that has been split into affected and unaffected subranges
#[derive(Debug, PartialEq)]
pub struct SplitRange {
    pub affected: Option<SeedRange>,
    pub unaffected: Vec<SeedRange>,
}


/// Splits a single seed range into up to three ranges:
/// 0 or 1 range that is affected by the transform
/// 0, 1 or 2 ranges that are unaffected by the transform
pub fn split_input_range(range: &SeedRange, transform: &Transform) -> SplitRange {
    let mut unaffected: Vec<SeedRange> = vec![];
    let mut affected: Option<SeedRange> = None;

    let range_end = range.start + range.length - 1;
    let transform_source_end = transform.source_start + transform.length - 1;

    // Entirely unaffected?
    if 
        range.start > transform_source_end || range_end < transform.source_start  {
        return SplitRange {
            unaffected: vec![range.clone()],
            affected,
        };
    };

    // My range starts to the left of the tranform,
    if range.start < transform.source_start {
        unaffected.push(SeedRange {
            start: range.start,
            length: transform.source_start - range.start,
        });
    };

    // We will have an affected section
    let affected_left = max(range.start, transform.source_start);
    let affected_right = min(range_end, transform_source_end);

    affected = Some(SeedRange {
        start: affected_left,
        length: affected_right - affected_left + 1,
    });

    // Possible right-affected
    if affected_right < range_end {
        unaffected.push(SeedRange {
            start: affected_right + 1,
            length: range_end - affected_right,
        })
    };

    SplitRange {
        unaffected,
        affected,
    }
}

// region: Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_input_range() {

        // All
        let result = split_input_range(
            &SeedRange { start: 12, length: 2, },
            &Transform { source_start: 9, length: 10, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![],
                affected: Some(SeedRange { start: 12, length: 2 }),
            }
        );

        // None (seed range is to the right of the transform)
        let result = split_input_range(
            &SeedRange { start: 19, length: 5, },
            &Transform { source_start: 9, length: 10, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![SeedRange { start: 19, length: 5, }],
                affected: None
            }
        );

        // None (seed range is to the left of the transform)
        let result = split_input_range(
            &SeedRange { start: 2, length: 7, },
            &Transform { source_start: 9, length: 10, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![SeedRange { start: 2, length: 7, }],
                affected: None
            }
        );

        // 1 char is inside the range, from left
        let result = split_input_range(
            &SeedRange { start: 2, length: 8, },
            &Transform { source_start: 9, length: 10, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![SeedRange { start: 2, length: 7, }],
                affected: Some(SeedRange { start: 9, length: 1}),
            }
        );

        // 1 char is inside the range, from right
        let result = split_input_range(
            &SeedRange { start: 18, length: 8, },
            &Transform { source_start: 9, length: 10, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![SeedRange { start: 19, length: 7, }],
                affected: Some(SeedRange { start: 18, length: 1}),
            }
        );

        // Inside from right, and extends past the bounds of the transform
        let result = split_input_range(
            &SeedRange { start: 10, length: 10, },
            &Transform { source_start: 8, length: 5, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![SeedRange { start: 13, length: 7, }],
                affected: Some(SeedRange { start: 10, length: 3}),
            }
        );
        

        // Extends past the bounds of the transform on both sides
        let result = split_input_range(
            &SeedRange { start: 1, length: 20, },
            &Transform { source_start: 8, length: 5, dest_start: 50, },
        );

        assert_eq!(
            result,
            SplitRange {
                unaffected: vec![SeedRange { start: 1, length: 7}, SeedRange {start: 13, length: 8}],
                affected: Some(SeedRange { start: 8, length: 5, } ),
            }
        );

    }
}

