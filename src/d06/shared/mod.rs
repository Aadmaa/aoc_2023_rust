use std::{fs::File, io::{BufRead, BufReader}};


#[derive(Debug, PartialEq, Clone)]
pub struct Race {
    pub duration: u64,
    pub record_distance: u64,
}

pub fn parse_file(reader: BufReader<File>) -> Result<Vec<Race>, String> {

    let mut result: Vec<Race> = vec![];

    let mut iter = reader.lines();

    let time_line = iter.next().expect("Time line missing").map_err(|e| e.to_string()).unwrap();
    let distance_line = iter.next().expect("Distance line missing").map_err(|e| e.to_string()).unwrap();

    let mut time_iter = time_line.split_ascii_whitespace();
    let mut distance_iter = distance_line.split_ascii_whitespace();

    let _ = time_iter.next();
    let _ = distance_iter.next();

    for (duration, record_distance) in time_iter.zip(distance_iter) {
        result.push( Race {
            duration: duration.parse::<u64>().unwrap(),
            record_distance: record_distance.parse::<u64>().unwrap(),
        } )
    }

    Ok(result)

}