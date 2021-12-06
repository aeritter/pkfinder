
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
