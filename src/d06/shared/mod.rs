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

pub fn parse_file_b(reader: BufReader<File>) -> Result<Race, String> {

    let mut iter = reader.lines();

    let time_line = iter.next().expect("Time line missing").map_err(|e| e.to_string()).unwrap();
    let distance_line = iter.next().expect("Distance line missing").map_err(|e| e.to_string()).unwrap();

    let mut time_iter = time_line.split_ascii_whitespace();
    let mut distance_iter = distance_line.split_ascii_whitespace();

    let _ = time_iter.next();
    let _ = distance_iter.next();

    let mut duration = "".to_string();
    duration.extend(time_iter);

    let mut record_distance = "".to_string();
    record_distance.extend(distance_iter);


    let result = Race {
        duration: duration.parse::<u64>().unwrap(),
        record_distance: record_distance.parse::<u64>().unwrap(),
    };

    Ok(result)

}


/// You can win when you press the button for B seconds, and B(D - B) > the record
/// So that gives a quadratic equation with a lower and upper bound. You have to be
/// Above the lower bound and below the upper bound
pub fn count_ways_to_win (race: &Race) -> Result<u64, String> {

    let duration = race.duration as f64;
    let record = race.record_distance as f64;

    let radical = (duration*duration - (4.0 * record)).sqrt();
    let result_upper = (duration + radical) / 2.0;
    let result_lower = (duration - radical) / 2.0;

    // It is a TIE with the existing record if result_upper is exactly an integer, so we'd need to subtract one to be faster
    let upper = if result_upper == result_upper.floor() {
        (result_upper - 1.0) as u64
    } else {
        result_upper.floor() as u64
    };

    let lower = if result_lower == result_lower.ceil() {
        (result_lower + 1.0) as u64
    } else {
        result_lower.ceil() as u64
    };

    // Victories are from lower to upper inclusive

    Ok(upper - lower + 1)
}

