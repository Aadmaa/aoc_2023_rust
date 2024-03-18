use std::fs::File;
use std::io::BufReader;

use super::shared::{parse_file, transform_one_seed_value, Rules, TransformGroup};


/// Seed transformation problems: individual seeds
/// 
/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d5 -t a -f "./data/d05/sample.txt"
/// cargo run -- -p d5 -t a -f "./data/d05/main.txt"
/// ````
pub fn d5(reader: BufReader<File>) -> Result<u64, String> {

    let rules = parse_file(reader).map_err(|e| e.to_string())?;

    let new_seed_val = transform_seeds(&rules).map_err(|e| e.to_string())?;

    let min_val: u64 = new_seed_val.iter().min().expect("The vector is empty!").clone();

    println!("RESULT: {}", min_val);

    Ok(min_val)
}


fn transform_seeds(rules: &Rules) ->  Result<Vec<u64>, String> {
    let mut mapped_seeds: Vec<u64> = vec![];
    for &seed in &rules.seeds {
        let new_seed_val = transform_seed_all_groups(seed, rules).map_err(|e| e.to_string())?;
        mapped_seeds.push(new_seed_val);
    }

    Ok(mapped_seeds)
}

fn transform_seed_all_groups(seed: u64, rules: &Rules) -> Result<u64, String> {
    let mut xform_seed = seed;

    for group in &rules.transform_groups {
        xform_seed = transform_seed_one_group(xform_seed, group).map_err(|e| e.to_string())?;
    }

    Ok(xform_seed)
}

fn transform_seed_one_group(seed: u64, group: &TransformGroup) -> Result<u64, String>  {
    let xform_option = group.transforms.iter().find(|v| seed >= v.source_start && seed <= (v.source_start + v.length) );

    match xform_option {
        Some(xform) => transform_one_seed_value(seed, xform),
        None => Ok(seed)
    }
}