use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn number_of_elements() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn main() {
    // write a function that gets the user input
    println!("Please input the number of elements");
    let number_of_elements = number_of_elements();

    println!("Please input the elements one by one");
    let mut numbers = Vec::new();

    for _ in 0..number_of_elements {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number = input.trim().parse().expect("Please type a number!");
        numbers.push(number);
    }

    let result = sum(&numbers);
    let average = result as f64 / numbers.len() as f64;

    println!("The sum is {} and the average is {}", result, average);
}
