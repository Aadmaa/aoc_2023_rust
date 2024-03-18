
use std::io::BufReader;
use std::fs::File;

use crate::d05::shared::rule_to_rules2;

use self::split_range::split_input_range;

use super::shared::{parse_file, Rules2, SeedRange, transform_one_seed_value};

mod split_range;

/// Seed transformation problems: ranges of seeds
/// 
/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d5 -t a -f "./data/d05/sample.txt"
/// cargo run -- -p d5 -t a -f "./data/d05/main.txt"
/// ````
pub fn d5_b(reader: BufReader<File>) -> Result<u64, String> {
    let rules = parse_file(reader).map_err(|e| e.to_string())?;

    let rules2 = rule_to_rules2(rules);

    let new_seed_val = transform_all_ranges_all_groups(&rules2).map_err(|e| e.to_string())?;

    let min_val: u64 = new_seed_val.iter().min_by_key(|x| x.start).expect("The vector is empty!").start;

    println!("RESULT: {}", min_val);

    Ok(min_val)
}

fn transform_all_ranges_all_groups(rules: &Rules2) -> Result<Vec<SeedRange>, String> {

    // println!("Starting rules: {:?}", rules);

    // Note that an input range can be split up and become 1, 2 or 3 input vectors
    // by a given transform group, depending on how the inputs overlap the range

    let mut untransformed: Vec<SeedRange> = rules.seed_ranges.clone();
    let mut transformed: Vec<SeedRange> = vec![];

    // println!("Initial: {:?}", untransformed);

    // Each group of transforms
    for group in &rules.transform_groups {

        // Each transform within the group
        for transform in &group.transforms {

            let mut untransformed_next: Vec<SeedRange> = vec![];

            for untransformed_range in &untransformed {
                // Split into up to three groups - two unaffected, one affected range
                let split_untransformed = split_input_range(&untransformed_range, transform);
                // Build the state of the untranformed group for the next transform
                untransformed_next.extend(split_untransformed.unaffected);

                if let Some(affected) = split_untransformed.affected {
                    // Get the new start value for the transformed range
                    let new_start = transform_one_seed_value(affected.start, transform)?;
                    transformed.push( SeedRange {
                        start: new_start,
                        length: affected.length
                    } )
                }
            }
            
            // Rebind untransformed to the next version
            untransformed = untransformed_next;

        }

        // Before we go to the next group, merge everything into Untransformed"
        untransformed.extend(transformed);
        transformed  = vec![];

        // println!("After {}: {:?}", group.name, untransformed);
    }

    // println!("Final: {:?}", untransformed);

    Ok(untransformed)
}
