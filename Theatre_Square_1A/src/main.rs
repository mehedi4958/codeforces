use std::io;
fn main() {
    // let  number_of_flagstone_by_the_length: u32 = 0;
    // let  number_of_flagstone_by_the_width: u32 = 0;
    // let  total_flagstone: u32 = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if is_valid_input(input) == false {
        println!("Invalid input.");
        return;
    }

    // if input.split_whitespace().count() != 3 {
    //     println!("Invalid input. Please provide exactly three values.");
    //     return;
    // }

    let numbers: Result<Vec<usize>, _> = input.split_whitespace().map(|s| s.parse()).collect();
    let numbers = numbers.expect("Failed to parse numbers");

    let length_of_the_square: usize = numbers[0];
    let width_of_the_square: usize = numbers[1];

    if length_of_the_square < 1 || width_of_the_square < 1 {
        //panic!("length and width must be >= 1");
        return;
    }

    let side_of_the_flagstone: usize = numbers[2];
    if side_of_the_flagstone <= 0 || side_of_the_flagstone > 10_usize.pow(9) {
        //panic!("size must be <= 10^9");
        return;
    }

    let mut number_of_flagstone_by_the_length = length_of_the_square / side_of_the_flagstone;
    if length_of_the_square % side_of_the_flagstone != 0 {
        number_of_flagstone_by_the_length += 1;
    }

    let mut number_of_flagstone_by_the_width = width_of_the_square / side_of_the_flagstone;
    if width_of_the_square % side_of_the_flagstone != 0 {
        number_of_flagstone_by_the_width += 1;
    }

    let total_flagstone = number_of_flagstone_by_the_length * number_of_flagstone_by_the_width;

    println!("{}", total_flagstone);
}

// read a single value
// fn read<T: std::str::FromStr>() -> T {
//     let mut number_of_lines = String::new();
//     io::stdin().read_line(&mut number_of_lines).unwrap();
//     // number_of_lines.trim().parse().ok().unwrap()
//     match number_of_lines.trim().parse() {
//         Ok(num) => num,
//         Err(_) => panic!("Failed to parse input"),
//     }
// }

// Checks whether a valid input is given
fn is_valid_input(input: &str) -> bool {
    input.len() >= 1
        && !more_than_two_spaces(input)
        && !input.contains("  ")
        && input.chars().all(|c| c.is_whitespace() || c.is_digit(10))
}

fn more_than_two_spaces(input: &str) -> bool {
    let mut space_count = 0;
    for c in input.chars() {
        if c.is_whitespace() {
            space_count += 1;
            if space_count > 2 {
                return true;
            }
        }
    }
    false
}
