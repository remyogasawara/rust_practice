fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    // let number = if condition { 5 } else { "six" };
    // this is an error because the true and false branches are differnet types 

    println!("The value of number is: {number}");

    // -------- loops ---------- 
    // infinite loop 
    // loop {
    //     println!("again!"); 
    // }
    
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; 
        }
    };
    println!("this is the result: {result}");

    // you can use a loop label to distingues what loop you are breaking using a single quote 
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");

    // while loops 
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // while loop vs for loop:
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // for loops 
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for loop range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
