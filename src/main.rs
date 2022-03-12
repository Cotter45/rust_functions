fn main() {
    // function calls 
    print_labeled_measurement(5, 'h');
    plus_one(5);


    // if conditionals and ternaries
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ternary, must be bool, must return same type
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}, after shadowing and ternary", number);

    // LOOPS

    // you can label loops to access them in inner loops and break
    // or continue if conditions are met. label with 'label_name:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // - semicolon ends the block and assigns value to result

    println!("The result is {}", result);

    // while conditions
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("While loop: the value is: {}", a[index]);

        index += 1;
    }

    // for loops 
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("For loop: the value is: {}", element);
    }

    // for loops numbers in range and in reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    println!("plus_one returns: {}", x + 1);
    x + 1
}