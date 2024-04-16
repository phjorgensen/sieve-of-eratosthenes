fn main() {
    let primes_under = get_user_int_input("Get primes under: ");

    let mut numbers = vec![true; primes_under + 1];

    let sqrt_of_max = (primes_under as f32).sqrt().ceil() as usize;
    for i in 2..sqrt_of_max {
        if numbers[i] == false {
            continue;
        }

        numbers = remove_multiples(numbers, i, primes_under);
    }

    print_result(numbers, primes_under);
}

fn remove_multiples(mut items: Vec<bool>, index: usize, end: usize) -> Vec<bool> {
    for j in (index + index..end).step_by(index) {
        items[j] = false;
    }

    return items;
}

fn print_result(items: Vec<bool>, max_int: usize) {
    for i in 2..max_int {
        if items[i] == true {
            println!("{}", i);
        }
    }
}

fn get_user_int_input(prompt: &str) -> usize {
    let max_int = get_user_input(prompt);

    return max_int
        .trim()
        .parse()
        .expect("Input was not a whole number.");
}

fn get_user_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};

    let mut user_input = String::new();

    print!("{}", prompt);

    let _ = stdout().flush();

    stdin()
        .read_line(&mut user_input)
        .expect("Did not enter a number");

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }

    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    return user_input;
}
