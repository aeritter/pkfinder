use std::fmt::{Display, self};

use crate::file_object::FileObject;

pub fn get_types(file_object: &FileObject, details: bool) {
    let mut typer_list = vec![Typer::new(); file_object.total_columns];
    for row in &file_object.data {
        for (colnum, field) in row.split(&file_object.internal_delimiter).enumerate() {
            typer_list[colnum].add(colnum, field)
        }
    }
    if details {
        for (colnum, answer) in typer_list.iter().enumerate() {
            println!("{}. {}: {}", colnum+1, file_object.headers[colnum], answer.summarize_stats())
        }
    } else {
        for (colnum, answer) in typer_list.iter().enumerate() {
            println!("{}. {}: {}", colnum+1, file_object.headers[colnum], answer.guess())
        }
    }
}


#[derive(Clone, Default)]
pub struct Typer<'a> {
    pub stats: Vec<GuessType<'a>>
}

impl<'a> Typer<'a> {
    pub fn new() -> Self {
        Self {
            stats: vec![]
        }
    }

    pub fn add(&mut self, rownum: usize, data: &'a str) {
        let g = GuessType::new(rownum, data);
        self.stats.push(g);
    }

    pub fn guess(&self) -> String {
        let data = self.summarize_stats();
        let num_datapoints = self.stats.len();
        if data.null == num_datapoints {
            return "null".to_string()
        }
        if data.int == num_datapoints-data.null {
            return "int".to_string()
        }
        if data.float == num_datapoints-data.null {
            if data.numeric.1 > 0 {
                return format!("numeric({}, {})", data.numeric.0, data.numeric.1)
            }
            return "float".to_string()
        }
        if data.char == num_datapoints-data.null {
            return "char".to_string()
        }
        return format!("varchar({})", data.varchar)
    }

    pub fn summarize_stats(&self) -> GuessSummary {
        GuessSummary::new(&self.stats)
    }
}


#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct GuessType<'a> {
    rownum: usize,
    data: &'a str,
    null: bool,
    int: bool,
    float: bool,
    numeric: Option<(usize, usize)>,
    char: bool,
    varchar: usize,
}
impl<'a> GuessType<'a> {
    pub fn new(rownum: usize, data: &'a str) -> Self {
        if data.is_empty() {
            return Self {
                rownum,
                null: true,
                int: false,
                float: false,
                numeric: None,
                char: false,
                varchar: 0,
                data,
            }
        }

        let int = data.parse::<i32>().is_ok();
        let float = data.parse::<f64>().is_ok();
        let char = data.parse::<char>().is_ok();
        let varchar = data.len();
        let mut numeric: Option<(usize, usize)> = None;

        if float {
            let total_digits = data.chars().filter(|x| x.is_numeric()).count();
            let decimal_precision = data.split(".").nth(1).unwrap_or("").chars().filter(|x| x.is_numeric()).count();
            numeric = Some((total_digits, decimal_precision));
        }
        
        GuessType {
            rownum,
            null: false,
            int,
            float,
            numeric,
            char,
            varchar,
            data,
        }

    }

}

pub struct GuessSummary {
    null: usize,
    int: usize,
    float: usize,
    numeric: (usize, usize),
    char: usize,
    varchar: usize,
}

impl GuessSummary {
    pub fn new(data: &Vec<GuessType>) -> Self {
        let mut null: usize = 0;
        let mut int: usize = 0;
        let mut float: usize = 0;
        let mut numeric: (usize, usize) = (0, 0);
        let mut char: usize = 0;
        let mut varchar: usize = 0;

        for datapoint in data {
            if datapoint.null == true {
                null += 1;
            }
            if datapoint.int == true {
                int += 1;
            }
            if datapoint.float == true {
                float += 1;
            }
            match datapoint.numeric {
                Some(num) => {
                    if numeric.0 < num.0 {
                        numeric.0 = num.0
                    }
                    if numeric.1 < num.1 {
                        numeric.1 = num.1
                    }
                }
                None => (),
            };
            if datapoint.char == true {
                char += 1;
            }
            if datapoint.varchar > varchar {
                varchar = datapoint.varchar;
            }
        }
        GuessSummary {
            null,
            int,
            float,
            numeric,
            char,
            varchar,
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
        assert_eq!(GuessType::new(0, &""), GuessType {rownum: 0, null:true, int:false, float:false, numeric:None, char:false, varchar:0, data: &"" });
    }

    #[test]
    fn check_int() {
        assert_eq!(GuessType::new(0, &"25"), GuessType {rownum: 0, null: false, int: true, float: true, numeric:Some((2, 0)), char: false, varchar: 2, data: &"25"});
    }
    
    #[test]
    fn check_float() {
        assert_eq!(GuessType::new(0, &"0.3258"), GuessType {rownum: 0, null: false, int: false, float: true, numeric:Some((5, 4)), char: false, varchar: 6, data: &"0.3258"});
    }

    #[test]
    fn check_char() {
        assert_eq!(GuessType::new(0, &"C"), GuessType {rownum: 0, null: false, int: false, float: false, numeric:None, char: true, varchar: 1, data: &"C"});
    }

    #[test]
    fn check_varchar() {
        assert_eq!(GuessType::new(0, &"test32\n"), GuessType {rownum: 0, null: false, int: false, float: false, numeric:None, char: false, varchar: 7, data: &"test32\n"});
    }
    
}
