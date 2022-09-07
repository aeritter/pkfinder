use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap, error::Error};

use regex::Regex;


pub fn determine_delimiter(file_path: &str) -> Result<String, Box<dyn Error>> {
    let reader = BufReader::new(File::open(file_path)?);
    let delim_finding_data: Vec<String> = reader.lines().take(5).map(|x| x.expect("Expected UTF-8 file encoding")).collect();

    let original_delimiter = match find_delimiter_method_1(&delim_finding_data) {
        Ok(delim) => delim,
        Err(_) => match find_delimiter_method_2(&delim_finding_data) {
            Ok(delim) => delim,
            Err(_) => panic!("Could not determine delimiter")
        },
    };
    return Ok(original_delimiter)
}

fn find_delimiter_method_1(test_data: &Vec<String>) -> Result<String, String>{
    let delimiter_types = vec![",", "\t"];
    for delim in delimiter_types {
        if test_data[0].contains(delim) {
            let delim_test_count: Vec<usize> = test_data.iter()
                .map(|x| x.matches(delim).count())
                .collect();
            if delim_test_count.iter().all(|x| x == &delim_test_count[0]) {
                return Ok(delim.to_string());
            }
        }
    }
    return Err("Could not find delimiter".to_string())
}

fn find_delimiter_method_2(test_data: &Vec<String>) -> Result<String, Box<dyn Error>>{
    // regex: Non-word, non-space, non-period value is grabbed, where it's followed by a quote or word character, followed by anything, then mirrored (palindrome)
    // if the value grabbed at the front is equivalent to the value grabbed at the back, add that value as a potential delimiter, then attempt
    // to use the value that appeared the most as the delimiter.
    let regex_string = Regex::new(r#"(?P<delim1>[^\w \.\\"])(?:"|\w).*?(?:"|\w)(?P<delim2>[^\w \.\\"])"#).unwrap();
    let mut delim_hashmap = HashMap::new();
    for row in test_data {
        for result in regex_string.captures_iter(&row) {
            if &result["delim1"] == &result["delim2"] {
                let delim1 = result["delim1"].to_string();
                let y = delim_hashmap.entry(delim1).or_insert(1);
                *y += 1;
            }
        }
    }
    let delim_guess = delim_hashmap
        .iter()
        .max_by(|x, y| x.1.cmp(&y.1))
        .map(|(k, _v)| k)
        .unwrap();
    return Ok(delim_guess.clone());
    // TODO: test the delim here and if it's wrong, test the one with the next-highest occurences or attempt to guess on a larger data set
    
}