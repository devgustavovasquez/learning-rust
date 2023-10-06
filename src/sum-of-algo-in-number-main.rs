use std::io;

fn get_i32_vec_from_string(raw_input: String) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for number in raw_input.trim().split("") {
        if number == "" {
            continue;
        }
        
        numbers.push(number.parse::<i32>().unwrap());
    }
    numbers
}

fn som_numbers_in_vec (numbers: Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

fn main() {
    //clear the screen
    print!("\x1B[2J\x1B[1;1H");
    println!("Enter a number: ");
    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input).unwrap();
    let numbers: Vec<i32> = get_i32_vec_from_string(raw_input);
    let result: i32 = som_numbers_in_vec(numbers);
    println!("The sum of the numbers is: {}", result);
}
