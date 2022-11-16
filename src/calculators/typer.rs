use std::fmt::{Display, self};

use crate::file_object::FileObject;

pub fn get_types(file_object: &FileObject, details: bool, trim_whitespace: bool) {
    let mut typer_list = vec![Typer::new(); file_object.total_columns];
    for row in &file_object.data {
        for (colnum, field) in row.split(&file_object.internal_delimiter).enumerate() {
            typer_list[colnum].add(field, trim_whitespace)
        }
    }
    if details {
        for (colnum, answer) in typer_list.iter().enumerate() {
            println!("{}. {}: {}", colnum+1, file_object.headers[colnum], answer.stats)
        }
    } else {
        for (colnum, answer) in typer_list.iter().enumerate() {
            println!("{}. {}: {}", colnum+1, file_object.headers[colnum], answer.guess())
        }
    }
}

#[derive(Clone, Default)]
pub struct Typer {
    pub stats: GuessSummary
}

impl Typer {
    pub fn new() -> Self {
        Self {
            stats: GuessSummary::new()
        }
    }

    pub fn add(&mut self, data: &str, trim_whitespace: bool) {
        self.stats.add_data(data, trim_whitespace);
    }

    pub fn guess(&self) -> String {
        if self.stats.null == self.stats.datapoints {
            return "null".to_string()
        }
        if self.stats.int == self.stats.datapoints-self.stats.null {
            return "int".to_string()
        }
        if self.stats.float == self.stats.datapoints-self.stats.null {
            if self.stats.numeric.0 > 0 {
                return format!("numeric({}, {})", self.stats.numeric.0, self.stats.numeric.1)
            }
            return "float".to_string()
        }
        if self.stats.char == self.stats.datapoints-self.stats.null {
            return "char".to_string()
        }
        return format!("varchar({})", self.stats.varchar)
    }
}



#[derive(Clone, Default, PartialEq, Debug)]
pub struct GuessSummary {
    datapoints: usize,
    null: usize,
    int: usize,
    float: usize,
    numeric: (usize, usize),
    char: usize,
    varchar: usize,
}

impl GuessSummary {
    pub fn new() -> Self {
        let datapoints: usize = 0;
        let null: usize = 0;
        let int: usize = 0;
        let float: usize = 0;
        let numeric: (usize, usize) = (0, 0);
        let char: usize = 0;
        let varchar: usize = 0;

        GuessSummary {
            datapoints,
            null,
            int,
            float,
            numeric,
            char,
            varchar,
        }
    }


    pub fn add_data(&mut self, data: &str, trim_whitespace: bool) -> () {
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

            let split_numeric: Vec<&str> = working_data.split(".").collect();
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

}

impl Display for GuessSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stats - (null: {}, int: {}, float: {}, numeric: ({}, {}), char: {}, max length: {})", self.null, self.int, self.float, self.numeric.0, self.numeric.1, self.char, self.varchar)
    }
}


#[cfg(test)]
mod test_type_guess {
    use super::*;

    #[test]
    fn check_null() {
        let mut test = Typer::new();
        test.add(&"", 0);
        assert_eq!(test.stats, GuessSummary {datapoints: 1, null: 1, int: 0, float: 0, numeric:(0, 0), char: 0, varchar: 0 });
    }

    #[test]
    fn check_int() {
        let mut test = Typer::new();
        test.add(&"25", 0);
        assert_eq!(test.stats, GuessSummary {datapoints: 1, null: 0, int: 1, float: 1, numeric:(2, 0), char: 0, varchar: 2 });
    }
    
    #[test]
    fn check_float() {
        let mut test = Typer::new();
        test.add(&"0.3258", 0);
        assert_eq!(test.stats, GuessSummary {datapoints: 1, null: 0, int: 0, float: 1, numeric:(5, 4), char: 0, varchar: 6 });
    }

    #[test]
    fn check_char() {
        let mut test = Typer::new();
        test.add(&"C", 0);
        assert_eq!(test.stats, GuessSummary {datapoints: 1, null: 0, int: 0, float: 0, numeric:(0, 0), char: 1, varchar: 1 });
    }

    #[test]
    fn check_varchar() {
        let mut test = Typer::new();
        test.add(&"test32\n", 0);
        assert_eq!(test.stats, GuessSummary {datapoints: 1, null: 0, int: 0, float: 0, numeric:(0, 0), char: 0, varchar: 7 });
    }
    
}
