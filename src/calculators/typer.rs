use std::fmt::{Display, self};

use crate::file_object::FileObject;

pub struct DatasetTyper {
    pub types_list: Vec<GuessSummary>
}

impl DatasetTyper {
    pub fn new(file_object: &FileObject, trim_whitespace: bool) -> Self {
        let mut typer_list: Vec<GuessSummary> = file_object.headers.iter().map(|x| GuessSummary::new(x.to_string())).collect();
        for row in &file_object.data {
            for (colnum, field) in row.split(&file_object.internal_delimiter).enumerate() {
                typer_list[colnum].add(field, trim_whitespace)
            }
        }
        Self {
            types_list: typer_list
        }
    }
    
    pub fn print_guesses(&self) {
        for (colnum, answer) in self.types_list.iter().enumerate() {
            println!("{}. {}: {}", colnum+1, answer.name, answer.guess())
        }
    }

    pub fn print_detailed_guesses(&self) {
        for (colnum, answer) in self.types_list.iter().enumerate() {
            println!("{}. {}: {}", colnum+1, answer.name, answer)
        }
    }
}


#[derive(Clone, Default, PartialEq, Debug)]
pub struct GuessSummary {
    name: String,
    datapoints: usize,
    null: usize,
    int: usize,
    float: usize,
    numeric: (usize, usize, bool), // (total digits, max digits after decimal, contains scientific notation (E+))
    char: usize,
    varchar: usize,
}

impl GuessSummary {
    pub fn new(name: String) -> Self {
        let datapoints: usize = 0;
        let null: usize = 0;
        let int: usize = 0;
        let float: usize = 0;
        let numeric: (usize, usize, bool) = (0, 0, false);
        let char: usize = 0;
        let varchar: usize = 0;

        GuessSummary {
            name,
            datapoints,
            null,
            int,
            float,
            numeric,
            char,
            varchar,
        }
    }

    pub fn add(&mut self, data: &str, trim_whitespace: bool) -> () {
        self.datapoints += 1;

        let working_data = if trim_whitespace { data.trim() } else { data };

        if working_data.len() == 0 {
            self.null += 1;
        } else if working_data.parse::<char>().is_ok() {
            self.char += 1;
        }
        if working_data.len() > self.varchar {
            self.varchar = working_data.len();
        }


        let numeric = working_data.parse::<f64>().is_ok();
        if numeric {
            self.float += 1;
                    
            if working_data.parse::<i32>().is_ok() {
                self.int += 1;
            }

            let split_numeric: Vec<&str>;
            let converted_str = working_data.parse::<f64>().unwrap().to_string();
            if working_data.contains("E+") {
                split_numeric = converted_str.split(".").collect();
                self.numeric.2 = true
            } else {
                split_numeric = working_data.split(".").collect();
            }
            if split_numeric.len() == 2 {
                let decinum = split_numeric[1].len();
                if decinum > self.numeric.1 {
                    self.numeric.1 = decinum
                }
            }
            let precision = split_numeric[0].len() + self.numeric.1;
            if precision > self.numeric.0 {
                self.numeric.0 = precision
            }
        }
    }

    pub fn guess(&self) -> String {
        if self.null == self.datapoints {
            return "null".to_string()
        }
        if self.int == self.datapoints-self.null {
            return "int".to_string()
        }
        if self.float == self.datapoints-self.null {
            if self.numeric.0 > 0 {
                if self.numeric.2 { println!("// Fields in the following column contain scientific notation, file may have been opened and saved from Excel.") };
                return format!("numeric({}, {})", self.numeric.0, self.numeric.1)
            }
            return "float".to_string()
        }
        if self.char == self.datapoints-self.null {
            return "char".to_string()
        }
        return format!("varchar({})", self.varchar)
    }
}

impl Display for GuessSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let warning = if self.numeric.2 { " // Scientific notation found." } else {""};
        write!(f, "Stats - (null: {}, int: {}, float: {}, numeric: ({}, {}), char: {}, max length: {}){}", self.null, self.int, self.float, self.numeric.0, self.numeric.1, self.char, self.varchar, warning)
    }
}


#[cfg(test)]
mod test_type_guess {
    use super::*;

    #[test]
    fn check_null() {
        let mut test = GuessSummary::new("null".to_string());
        test.add(&"", false);
        assert_eq!(test, GuessSummary {name: "null".to_string(), datapoints: 1, null: 1, int: 0, float: 0, numeric:(0, 0, false), char: 0, varchar: 0 });
    }

    #[test]
    fn check_int() {
        let mut test = GuessSummary::new("int".to_string());
        test.add(&"25", false);
        assert_eq!(test, GuessSummary {name: "int".to_string(), datapoints: 1, null: 0, int: 1, float: 1, numeric:(2, 0, false), char: 0, varchar: 2 });
    }
    
    #[test]
    fn check_float() {
        let mut test = GuessSummary::new("float".to_string());
        test.add(&"0.3258", false);
        assert_eq!(test, GuessSummary {name: "float".to_string(), datapoints: 1, null: 0, int: 0, float: 1, numeric:(5, 4, false), char: 0, varchar: 6 });
    }

    #[test]
    fn check_char() {
        let mut test = GuessSummary::new("char".to_string());
        test.add(&"C", false);
        assert_eq!(test, GuessSummary {name: "char".to_string(), datapoints: 1, null: 0, int: 0, float: 0, numeric:(0, 0, false), char: 1, varchar: 1 });
    }

    #[test]
    fn check_varchar() {
        let mut test = GuessSummary::new("varchar".to_string());
        test.add(&"test32\n", false);
        assert_eq!(test, GuessSummary {name: "varchar".to_string(), datapoints: 1, null: 0, int: 0, float: 0, numeric:(0, 0, false), char: 0, varchar: 7 });
    }
    
}
