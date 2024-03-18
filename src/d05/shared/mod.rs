use std::{fs::File, io::{BufRead, BufReader}};

// use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Transform {
    pub dest_start: u64,
    pub source_start: u64,
    pub length: u64,
}

#[derive(Debug, PartialEq)]
pub struct TransformGroup {
    pub name: String,
    pub transforms: Vec<Transform>,
}

impl TransformGroup {
    fn new(name: &str) -> TransformGroup {
        TransformGroup {
            name: name.to_string(),
            transforms: vec![]
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rules {
    pub seeds: Vec<u64>,
    pub transform_groups: Vec<TransformGroup>,
}

impl Rules {
    fn new(seeds: Vec<u64>) -> Rules {
        Rules {
            seeds,
            transform_groups: vec![]
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct SeedRange {
    pub start: u64,
    pub length: u64,
}

#[derive(Debug, PartialEq)]
pub struct Rules2 {
    pub seed_ranges: Vec<SeedRange>,
    pub transform_groups: Vec<TransformGroup>,
}

impl Rules2 {
    fn new(
        initial_seed_ranges: Vec<SeedRange>, 
        transform_groups: Vec<TransformGroup>
    ) -> Rules2 {
        Rules2 {
            seed_ranges: initial_seed_ranges,
            transform_groups
        }
    }
}

// Convert Rules to Rules2, which uses seed ranges instead of seeds
pub fn rule_to_rules2(rules: Rules) -> Rules2 {
    let seed_pair_iter = rules.seeds.chunks(2);
    let mut seed_ranges: Vec<SeedRange> = vec![];
    for pair in seed_pair_iter {
        seed_ranges.push( SeedRange {
            start: pair[0],
            length: pair[1],
        });
    };
    Rules2::new(seed_ranges, rules.transform_groups)
}


pub fn parse_file(reader: BufReader<File>) -> Result<Rules, String> {

    let mut iter = reader.lines();

    let seed_line = iter.next().expect("Seed line missing").map_err(|e| e.to_string()).unwrap();
    let mut seed_iter = seed_line.split_ascii_whitespace();
    let _ = seed_iter.next();

    let mut seeds: Vec<u64> = vec![];

    // Collect seeds into a vector
    for val_string in seed_iter {
        seeds.push(val_string.parse::<u64>().unwrap());
    };

    // Instantiate our Rules structure
    let mut rules: Rules = Rules::new(seeds);

    for the_line in iter {

        let line = the_line.unwrap();

        // Skip blank line
        if line.trim().len() == 0 {
            continue;
        }

        // Measurement header line
        if line.find('-').is_some() {
            // Create a new map
            rules.transform_groups.push(
                TransformGroup::new(&line)
            );

            continue;
        }

        // We should have a Rule line like "49 53 8"
        let mut rule_iter = line.split_ascii_whitespace();

        let dest_start = rule_iter.next().unwrap().parse::<u64>().unwrap();
        let source_start = rule_iter.next().unwrap().parse::<u64>().unwrap();
        let length = rule_iter.next().unwrap().parse::<u64>().unwrap();

        let transform: Transform = Transform {
            dest_start,
            source_start,
            length
        };

        // Put push it into the transforms for the final transform group
        let last_transform_group: &mut TransformGroup = rules.transform_groups.last_mut()
            .expect("There should be a transform group");
        
        last_transform_group.transforms.push(transform);
       
    }

    // println!("RULES {:?}", rules);

    Ok(rules)
}

pub fn transform_one_seed_value(seed: u64, transform: &Transform) -> Result<u64, String> {
    // println!("seed {}, dest_start {}, source_start {}", seed, xform.dest_start, xform.source_start );
    let rez = (seed + transform.dest_start).checked_sub(transform.source_start);
    if let Some(checked_val) = rez {
        Ok(checked_val)
    } else {
        Err(format!(
            "FAILED! seed {}, dest_start {}, source_start {}",
            seed, transform.dest_start, transform.source_start
        ))
    }
}