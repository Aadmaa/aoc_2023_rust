use std::cmp::max;

use regex::Regex;

// Get the error to the outside if there is one
pub fn invert_line_value(v: &Option<Result<String, impl ToString >>) -> Result<Option<String>, String> {
    match v {
        None => Ok(None),
        Some(Err(e)) => Err(e.to_string()),
        Some(Ok(inner_value)) => Ok(Some(inner_value.to_string())),
    }
}

pub fn search_slice_with_regex(input: &Option<String>, start: usize, end: usize, pattern: &str) -> bool {
    if let Some(ref str) = input {
        if end <= str.len() && start <= end {
            let slice = &str[start..end+1];
            let re = Regex::new(pattern).unwrap(); 
            let result = re.is_match(slice);

            if result {
                return true
            }
        }
    }

    false
}


/* 
pub fn get_vals_from_slice_with_regex<'a>(input: &'a Option<String>, start: usize, end: usize, pattern: &str) -> Vec<&'a str> {

    let mut result: Vec<&str> = vec![];

    if let Some(ref str) = input {
        if end <= str.len() && start <= end {
            let slice = &str[start..end+1];
            let re = Regex::new(pattern).unwrap(); 

            // Search the slice with the regex pattern
            let match_iter = re.find_iter(slice);

            match_iter.for_each(|mat| {
                let st = mat.as_str();
                result.push(st);
            });

        }
    }

    result
}
*/

pub fn get_numbers_touching_index(line: &Option<String>, index: usize) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    if let Some(ref str) = line {
        let re = Regex::new(r"[0-9]+").unwrap();
        let matches = re.find_iter(str);

        for mat in matches {
            let mat_start = max(mat.start().saturating_sub(1), 0);
            let mat_end = mat.end();
            if mat_start <= index && mat_end >= index {
                let num: u32 = mat.as_str().parse().unwrap();
                result.push(num);
            }
        }
    }

    result
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_numbers_touching_index() {
        // println!("TEST: test_get_numbers_touching_index");
        // 467..114..
        // ...*......
        // ..35..633.
        let list = get_numbers_touching_index(&Some("467..114..".to_string()), 2);
        assert_eq!(list, vec![467]);

        let list2 = get_numbers_touching_index(&Some("46".to_string()), 2);
        assert_eq!(list2, vec![46]);

        let list3 = get_numbers_touching_index(&Some("467..114..".to_string()), 0);
        assert_eq!(list3, vec![467]);

        let list3 = get_numbers_touching_index(&Some("467..114".to_string()), 8);
        assert_eq!(list3, vec![114]);

        let list4 = get_numbers_touching_index(&Some("467..114".to_string()), 7);
        assert_eq!(list4, vec![114]);

        let list5 = get_numbers_touching_index(&Some("467..114".to_string()), 4);
        assert_eq!(list5, vec![114]);

        let list6 = get_numbers_touching_index(&Some("467..114".to_string()), 3);
        assert_eq!(list6, vec![467]);

       //  let list7 = get_numbers_touching_index(&Some("467...114".to_string()), 4);
       // assert_eq!(list7, vec![]);

        // println!("COMPLETED TEST: test_get_numbers_touching_index");
    }
}