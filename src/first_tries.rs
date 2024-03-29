#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod tutorial{

    use std::io;
    use std::io::Read;
    use rand::Rng;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;

    use std::fs::File;

    pub fn error_handling_panic() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error)
            },
        };

        // shortcut, also calls panic!
        let f = File::open("hello.txt").unwrap();

        // shortcut, also calls panic!, with message
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    // shows how a Result works (erorrhandling)
    pub fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

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

    pub struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
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

        let user2 = build_user(String::from("harry@harry.com"), String::from("Harry"));
        println!("User: {} - email: {}", user2.username, user2.email);

        let user3 = User {
            email: String::from("different@emailaddress.com"),
            ..user2 // copies the remainder of fields from user2
        };
        println!("User: {} - email: {}", user3.username, user3.email);

        // you can also use a struct without names, for instance to bundle something returned from a function
        struct FunctionResult(i32, i32, i32);
        let result = FunctionResult(43,178,0);
        println!("Age:{} Length (cm):{}, # of Children:{}",result.0, result.1, result.2);

        // now we implement a method IN the struct (sort of like how a class is both a struct AND method prototype
        let rect1 = Rectangle { width: 30, height: 50 };
        println!("The area of the rectangle is {} square pixels.", rect1.area());
        let rect2 = Rectangle { width: 20, height: 20 };
        let rect3 = Rectangle { width: 40, height: 50 };
        println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
    }


    pub fn enums(){
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        impl Coin {
            fn value_in_cents(&self) -> u8 {
                match &self {
                    Coin::Penny => 1,
                    Coin::Nickel => 5,
                    Coin::Dime => 10,
                    Coin::Quarter => 25,
                }
            }
            fn description(&self) -> &str {
                match &self {
                    Coin::Penny => "Penny",
                    Coin::Nickel => "Nickel",
                    Coin::Dime => "Dime",
                    Coin::Quarter => "Quarter",
                }
            }
        }

        println!("{} = {} cents", Coin::Penny.description(), Coin::Penny.value_in_cents());
    }

    pub fn if_let(){
        let some_u8_value = Some(0u8);
        let three = Some(3);

        // without if-let
        match three {
            Some(3) => println!("three"),
            _ => (),
        }

        // the same, but with if-let
        if let Some(3) = three {
            println!("three");
        } else {
        }
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
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

    pub fn strings_and_hash_maps(){
        let s = "alex";
        let s1 = String::from(s);
        let s2 = "this is also possible to create a String".to_string();
        let an_utf8_string = String::from("Здравствуйте");

        // you can also "use" within a function
        use std::collections::HashMap;

        //this is a scores hashmap
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // you can also build the hashmap using vectors.
        let teams  = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("{:?}", score);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // the below will use "entry" to insert a key/value pair only when it does not exist yet
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    pub fn count_words(text:&str){

        let mut map = HashMap::new();

        let alt_text = text.replace(",", "").replace(".","");

        for word in alt_text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
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