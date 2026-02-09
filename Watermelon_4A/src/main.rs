fn main() {
    loop {
        //println!("Please enter a number: "); // does not need this for the submission, but it is useful for testing
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let number: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Check if number is between 1 and 100
        if number < 1 || number > 100 {
            println!("Please enter a number between 1 and 100.");
            continue;
        }

        // if it is 2, then cannot be divided to two person with using two even numbers.
        if number == 2 {
            println!("No");
            break;
        }

        if number % 2 == 0 {
            println!("Yes");
            break;
        } else {
            println!("No");
            break;
        }
    }
}
