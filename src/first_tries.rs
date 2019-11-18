#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod tutorial{

    use std::io;
    use std::cmp::Ordering;
    use rand::Rng;

    pub fn guessing_game() {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }

    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    pub fn structs(){

        let mut user1 = User {
            email: String::from("incorrect@emailaddress.com"),
            username: String::from("avk"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("info@alvanklaveren.com");

        println!("User: {} - email: {}", user1.username, user1.email);
    }

    pub fn string_slice(){
        let s = "hello you!";

        // NOTE: Interesting is that when we call the variable first_word (just like the function)
        //       it will work the first time, but when we reassign the variable first_word later
        //       on it will fail miserably... apparently there is a naming conflict between functions
        //       and variables.
        let word = first_word(s);
        println!("{}", word);

        let s = String::from("hello you!");
        let word = first_word(&s);
        println!("{}", word);

        // the below will fail because first_word is an immutable borrow, therefore does not
        // allow us to clear the original mutable string 's', as it would also modify
        // the immutable 'first_word'. (also because it is a borrow, defining first_word as
        // mut &str will not help.
        // s.clear();
        //println!("{}", first_word);
    }

    pub fn first_word(s: &str) -> &str {
        let white_space = b' '; // ' ' is a character, so b' ' is the byte value of ' '
        let bytes = s.as_bytes();

        // enumerate creates a two-valued tuple containing an index and the char at that index
        for (i, &item) in bytes.iter().enumerate() {
            if item == white_space {
                return &s[0..i]; // this is called a String Slice (&s[..i] also works)
            }
        }

        &s[..] // or just &s
    }

    pub fn borrowing_test() {
        let s1 = String::from("hello");
        let s2 = s1; // we MOVE the ownership from s1 to s2. s1 becomes invalid
        // the below does not work, because s2 is now the owner, and s1 has therefore been deallocated
        //println!("s1 : {} and s2: {}", s1, s2);

        // in the following example, the value of s1 is borrowed by s2
        let mut s1 = String::from("hello");
        let s2 = &s1; // & means borrow
        println!("s1 : {} and s2: {}", s1, s2);

        // this is not allowed, because s2 is not the owner, only borrowing the value
        // s2.push_str(", world");
        s1.push_str(", world");
        println!("s1 : {}", s1);
        // the below line will fail, because the borrowed value from s1 in s2 has changed and has therefore become invalid
        //println!("s2 : {}", s2);

        let s = String::from("hello");
        // the below does not work, because s is immutable
        //s.push_str(", world!");

        // mutable String object (this different from the primitive data type str).
        // re-declaring a variable is possible, as we do below reusing the s variable
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s); // prints "hello, world!"

        // cloning. The String object has some additional features to get past this
        let mut s1 = String::from("hello");
        let mut s2 = s1.clone();
        s1.push_str(", world 1!");
        s2.push_str(", world 2!");
        println!("s1 : {} and s2: {}", s1, s2);

        // there is an exception for primitive variables like integers
        // because the time it takes and because of the known size it has, it does not move ownership
        // this is possible, because primitive values have the 'Copy' trait
        // (and tuples as long as they only hold values that also have the Copy trait).
        let a = 10;
        let mut b = a;
        // now both a and b exist.
        println!("a: {} and b: {}", a, b);
        // to prove b exists as an individual value, we add 1 to b.
        b = b + 1;
        println!("a: {} and b: {}", a, b);

        // the same when assigning a value to an argument in a function.
        // So with strings, when we do not explicitly BORROW the variable (using &, called reference)
        // then the variable MOVES
        let s = String::from("Hello");
        takes_ownership(s);
        // the below line fails, because the ownership moved
        //println!("{}", s);
        let s = String::from("Hello there");
        borrows(&s);
        // The below works, because the value was borrowed
        println!("s is still valid: {}", s);

        // and for primitives, or any other type that has the Copy trait, it just works
        let x = 5;
        makes_copy(5);
        println!("x is still valid: {}", x);

        // another thing is how to borrow mutable variables. This is done using the &mut declaration
        let mut s = String::from("hello");
        mutable_argument(&mut s);
        println!("s: {}", s); // prints "hello, world!"
    }

    pub fn mutable_argument(s:&mut String){
        s.push_str(", world!");
    }


    pub fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    }

    pub fn borrows(some_string: &String) { // some_string comes into scope
        println!("{}", some_string);
    }

    pub fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    pub fn arrays(){
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
        println!("first and second: {} {}", first, second);

        let b = [1;3];
        for element in b.iter() {
            println!("element: {}", element);
        }
    }

    pub fn tuples(){
        // structuring tuples
        let tup : (i32, f64, u8) = (500, 6.4, 1);
        println!("The second value in tup is: {}", tup.1);

        // destructuring tuples (to individual variables)
        let (x, y, z) = tup;
        println!("The value of y is: {}", y);
    }

    pub fn min_max(){
        //println!("f64 (min/max) = {} / {}", std::f64::MIN, std::f64::MAX);
        println!("i32 (min/max) = {} / {}", std::i32::MIN, std::i32::MAX);
        println!("u32 (min/max) = {} / {}", std::u32::MIN, std::u32::MAX);
        println!("i64 (min/max) = {} / {}", std::i64::MIN, std::i64::MAX);
        println!("u64 (min/max) = {} / {}", std::u64::MIN, std::u64::MAX);
        println!("i128 (min/max) = {} / {}", std::i128::MIN, std::i128::MAX);
        println!("u128 (min/max) = {} / {}", std::u128::MIN, std::u128::MAX);
        println!("isize (min/max) = {} / {}", std::isize::MIN, std::isize::MAX);
        println!("usize (min/max) = {} / {}", std::usize::MIN, std::usize::MAX);

    }

}