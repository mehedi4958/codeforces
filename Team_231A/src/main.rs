fn main() {
    let mut number_of_problems = 0;
    // number of inputs
    let n: usize = read();

    if n < 1 || n > 1000 {
        //panic!("n must be between 1 and 100");
        return;
    }
    for i in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if is_valid_input(input) == false {
            println!("Invalid input. ");
            break;
        }

        let count_0 = input.chars().filter(|&c| c == '0').count();
        if count_0 < 2 {
            number_of_problems += 1;
        }
    }
    println!("{}", number_of_problems);
}

// read a single value
fn read<T: std::str::FromStr>() -> T {
    let mut number_of_lines = String::new();
    std::io::stdin().read_line(&mut number_of_lines).unwrap();
    // number_of_lines.trim().parse().ok().unwrap()
    match number_of_lines.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse input"),
    }
}

// Checks whether a valid input is given
fn is_valid_input(input: &str) -> bool {
    input.len() >= 1
        && input.len() <= 5
        && input.chars().all(|c| c.is_whitespace() || c.is_digit(2))
}
