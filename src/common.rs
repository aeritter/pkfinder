use std::{io::{BufReader, BufRead, Read}, fs::File, error::Error};


pub fn sort_and_convert_to_offset(column_numbers: &Vec<usize>) -> Vec<usize> {

    let mut column_numbers_sorted: Vec<usize> = column_numbers.clone();
    column_numbers_sorted.sort_unstable();
    let mut column_numbers_offset: Vec<usize> = vec![];
    let mut previous_number: usize = 0;
    for x in column_numbers_sorted {
        column_numbers_offset.push(x-&previous_number);
        previous_number = x+1;
    }
    return column_numbers_offset;

}


pub fn print_details(file_path: &str, delimiter: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(file_path)?);
    let headers: String = reader.by_ref().lines().take(1).map(|x| x.expect("Expected UTF-8 file encoding")).collect();
    // println!("Newline format: {:?}", self.newline);
    println!("Delimiter: {:?}", delimiter);
    println!("Column count: {:?}", headers.split(&delimiter).count());
    println!("Row count: {:?}", reader.lines().count());
    Ok(())
}


pub fn print_column_numbers(file_path: &str, delimiter: &str, skip_lines: usize) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(file_path)?).lines();
    let reader = reader.skip(skip_lines);
    let header: String = reader.take(1).map(|x| x.expect("Expected UTF-8 file encoding")).collect();
    println!();
    for (index, column) in header.split(delimiter).enumerate() {
        println!("{}: {}", index+1, column)
    }
    Ok(())
}
