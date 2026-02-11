use std::io;
fn main() {
    let mut value = 0;
    // number of operations
    let n: usize = read();

    if n < 1 || n > 150 {
        //panic!("n must be between 1 and 100");
        return;
    }

    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if is_valid_input(input) == false {
            println!("Invalid input. ");
            break;
        }

        if input == "++X" || input == "X++" {
            value += 1;
        }
         if input == "--X" || input == "X--" {
            value -= 1;
        }
    }
    println!("{}", value);
}

// read a single value
fn read<T: std::str::FromStr>() -> T {
    let mut number_of_lines = String::new();
    io::stdin().read_line(&mut number_of_lines).unwrap();
    // number_of_lines.trim().parse().ok().unwrap()
    match number_of_lines.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse input"),
    }
}

// Checks whether a valid input is given
fn is_valid_input(input: &str) -> bool {
    input == "++X" || input == "X++" || input == "--X" || input == "X--"
}
