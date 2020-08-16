use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::any::Any;
use std::borrow::Borrow;


fn main() {
    loops();
}


fn loops() {
    let mut counter = 0;
    let result = loop {
        counter+=1;
        if counter == 10 {
            break counter *2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
     while number != 0 {
         println!("{}!", number);
         number -= 1;
     }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30 , 40 , 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn functions(x: i32, y:i32){
    //This is function implementation
    println!("Happy hacking! :)");
    let z = {
      let a = 1;
        a+1;
    };
}

fn ifConditions() -> i32 {
    let number = 32;
    if number == 32 {
        println!("true");
    } else {
        println!("wrong!");
    }
    if number == 32 {number} else {0}
}

fn returnValues() -> i32 {
    5
}

fn numericOperations(){
    let sum = 5 + 10;
    let difference = 0.3 - 0.2;
    let product = 4 * 30;
    let division = 3/1;
    let remainder = 43 % 5;
}

fn dataTypes(){
    let guess: u32 = "42".parse().expect("Not a number");
    let i8: u8 = "255".parse().expect("Not a expected number."); // if it was 256 we would get an exception.
    let f: bool = false;
    let t = true;
    println!("guess: {}", guess);
    println!("guess: {}", i8);

    //The Tuple 😊😊😊
    let mut tup : (i32, f64, u8) = (500, 6.4, 1);
    let tup = ("a",true,1);
    let mut a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March"]; // April?May?June? are you fucking kidding me? corona losts my months.
    let a: [i32; 5] = [1,2,3,4,5];
    let a = [3; 5];
    let first = a[0];

}

fn immutableVariables(){
    let x = 5;
    println!("the value of x is {}", x);
    //x = 6;  ---->this variable is immutable. Therefore we cannot change.
    println!("the value of x is {}", x);
}

fn mutableVariables(){
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;  // this variable mutable therefore we can change it.
    println!("The value of x is {}", x);
    let x = "String"; // this variable mutable therefore we can change type its.
    println!("The value of x is {}", x);
}

fn guessGame(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess : ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        println!("You guessed: {}", guess);

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {println!("You win!"); break;}
        }
    }
}
