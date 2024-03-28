const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    println!("Constant variable: {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    // when shadowing, you can also change the type of a variable 
    let spaces = "   ";
    let spaces = spaces.len();

    // whereas this will cause an error 
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}", z);
    println!("Cat {heart_eyed_cat}");

    // tuples 
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // arrays -- fixed number of elements 
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    // program that gets array element at the index you entetr 
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // a vector is allowed tchange in size 

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    // let y = __ is a statemnt 
    // {let x = 3; x+1} is an expression 
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");


    // call the function 
    another_function(5); 
    let x = five() ;

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// functions with return values: we are returning an i32
// you can return early using "return" but most functions implicity return the last expression 
fn five() -> i32 {
    5
}

// this does not cause an error 
fn plus_one(x: i32) -> i32 {
    x + 1
}
// but this does becuase of the semicolon  
// we say we are returning an i32, but x+1; is a statement which does not return a balue
fn plus_one(x: i32) -> i32 {
    x + 1;
}