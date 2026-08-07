use std::io;

fn main() {
    println!("Welcome to the calculator in CLI!");

    let mut again = true;

    while again {
        let mut number1 = String::new();
        let mut number2 = String::new();
        let mut operator = String::new();

        println!("Please enter your first desired number: ");

        io::stdin()
            .read_line(&mut number1)
            .expect("Please enter a valid number!");

        println!("Enter operator (+, -, *, /): ");

        io::stdin()
            .read_line(&mut operator)
            .expect("Enter valid operator!");

        println!("Good, now the second number: ");

        io::stdin()
            .read_line(&mut number2)
            .expect("Please enter a valid number!");

        let num1: f32 = number1.trim().parse().expect("Invalid number");
        let num2: f32 = number2.trim().parse().expect("Invalid number");
        let operator = operator.trim();

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("That's not a valid operator!!!");
                continue;
            }
        };

        println!("Result: {result}");

        let mut response = String::new();
        println!("Do you wanna calculate again?(y/n)");

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read");

        response = response.trim().to_lowercase();

        if response == "y" {
            println!("-----Calculating again-----");
            again = true;
        } else {
            again = false;
            println!("-----Okay, thanks for using me!-----");
        }
    }
}