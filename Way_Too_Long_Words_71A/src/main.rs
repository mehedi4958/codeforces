//use regex::Regex;

fn main() {
    // cannot use regex
    //let re = Regex::new(r"^[a-z]{1,100}$").unwrap();
    let mut number_of_inputs = String::new();

    std::io::stdin()
        .read_line(&mut number_of_inputs)
        .expect("Failed to read line");
    let number_of_inputs: u32 = match number_of_inputs.trim().parse() {
        Ok(num) => {
            if num < 1 || num > 100 {
                //println!("Invalid input. Please enter a number between 1 and 100.");
                return;
            }
            num
        }
        Err(_) => {
            //println!("Invalid input. Please enter a number between 1 and 100.");
            return;
        }
    };

    for i in 0..number_of_inputs {
        //println!("Please enter a word with more than 10 letters in lowercase: "); // does not need this for the submission, but it is useful for testing
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if is_valid_input(input) == false {
            //println!("Invalid input. Please enter a word with more than 10 letters in lowercase.");
            break;
        }

        if input.len() <= 10 {
            println!("{}", input);
        }

        if input.len() > 10 {
            let f_char = input.chars().next().unwrap();
            let l_char = input.chars().last().unwrap();
            let middle_count = input.len() - 2;
            println!("{}{}{}", f_char, middle_count, l_char);
        }
    }
}

// Checks whether a valid input is given
fn is_valid_input(input: &str) -> bool {
    input.len() >= 1 && input.len() <= 100 && input.chars().all(|c| c.is_ascii_lowercase())
}
