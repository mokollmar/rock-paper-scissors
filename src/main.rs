use rand::{thread_rng, Rng};
use std::io;

enum UserOutcome {
    Won,
    Loose,
    Match,
}

fn main() {
    let mut user_score: u32 = 0;
    let mut machine_score: u32 = 0;

    welcome_message();

    run_tests();

    let options: [&str; 3] = ["Rock", "Paper", "Scissor"];

    // Start Game:
    for _ in [0..100] {
        let random: u8 = thread_rng().gen_range(0..2);
        let mut user_choice = String::new();

        println!("What's your choice?");
        println!("( Rock=0 / Paper=1 / Scissor=2 )");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Sorry couldn't read this line!");

        let user_choice: u8 = match user_choice.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("=== Please input only Numbers! ===");
                continue;
            }
        };

        countdown();

        let _outcome: UserOutcome = evaluate_outcome(user_choice, random);
        if matches!(_outcome, UserOutcome::Won) {
            user_score += 1
        } else if matches!(_outcome, UserOutcome::Loose) {
            machine_score += 1
        }

        println!("I played: {}", options[random as usize]);
        println!("=== My Score is: {machine_score} and your Score is: {user_score} ===")
    }
}

fn evaluate_outcome(user_choice: u8, random: u8) -> UserOutcome {
    // 0 = Rock / Paper= 1 / Scissor = 2

    if user_choice != random {
        if (user_choice == 0 && random == 2)
            || (user_choice == 1 && random == 0)
            || (user_choice == 2 && random == 1)
        {
            special_message("Nice you just won!");
            UserOutcome::Won
        } else {
            special_message("You loosed!! I WON HAHA");
            UserOutcome::Loose
        }
    } else {
        special_message("LOL we both have the same! Noboy won!");
        UserOutcome::Match
    }
}

fn special_message(message: &str) {
    println!("=== {message} ===")
}

fn welcome_message() {
    print!(
        "
    \n\n
    **********************************************************************
    *****                                                            *****
    *****                                                            *****
    *****         Welcome to the ROCK PAPER SCISSOR Game             *****      
    *****                                                            *****
    *****                                                            *****
    **********************************************************************
    \n\n
    "
    );
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{number} !!")
    }
}

// & => create a reference to a variable without taking ownership
// String can be modified => Heap
// str has no pre-defined size => Is normally (standalone) not used
// to borrow a string use: &str
// &str => borrowed reference to a string, is a string slice => No Ownership
// &str => mostly used to pass strings
// &str => component (pointer, length) in Stack / Data (UTF-8) in Heap

// ---

fn run_tests() {
    let x = [_test_1, _test_2, _test_3, _test_4, _test_5, _test_6, _test_7, _test_8, _test_9, _test_10];

    for func in x {
        func();
    }
}

fn _test_1() {
    let s1 = String::from("Hello"); // non-literal
    let s2 = String::from("Hello"); // non-literal
    let x = 5; // literal

    func_string(s1);
    // print!("{}", s1); => ERROR, because s1 is moved to s1' (as a argument), so s1 is invalid

    func_string(s2.clone());
    print!("{}", s2);

    func_int(x);
    print!("{}", x);

    fn func_string(some_string: String) {
        print!("{}", some_string);
    }

    fn func_int(some_int: u32) {
        print!("{}", some_int);
    }
}

fn _test_2() {
    // testing references
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn _test_3() {
    // testing mutable references
    let mut s1 = String::from("Hello");

    modify_string(&mut s1);

    println!("The modified string is: '{}'", s1);

    fn modify_string(s: &mut String) {
        s.push_str(", World!");
    }
}

fn _test_4() {
    // testing mutable references
    let mut s1 = String::from("Hello");

    let _r1 = &mut s1;
    // let _r2 = &mut s1; //=> ERROR

    println!("{}", _r1);
}

fn _test_5() {
    // testing mutable references
    let mut s1 = String::from("Hello");

    func_one(&mut s1);
    func_two(&mut s1);

    fn func_one(_s: &mut String) {
        println!("{}", _s);
    }
    fn func_two(_s: &mut String) {
        println!("{}", _s);
    }
}

fn _test_6() {
    // let reference_to_nothing = dangle();

    // fn dangle() -> &String { // => Error
    //     let s = String::from("Hello");
    //     &s
    // }
}

fn _test_7() {
    let my_list = ["Hello", "Jo", "Hi"];

    for (index, element) in my_list.iter().enumerate() {
        println!("The {index}). element is {element}")
    }
}

fn _test_8() {
    let my_string = String::from("Hello there!");

    let word = get_first_word(&my_string);
    println!("My word is: {word}");

    fn get_first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (index, &element) in bytes.iter().enumerate() {
            if element == b' ' {
                // Byte string synatx
                return index;
            }
        }

        s.len()
    }

    // The Problem here is that my_word and the first word index are not in sync
    // => if my_word changes, the word index isn't correct anymore.
}

fn _test_9() {
    let my_string = String::from("Hello there!");

    let word = get_first_word(&my_string);
    println!("My word is: {word}");

    fn get_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (index, &element) in bytes.iter().enumerate() {
            if element == b' ' {
                return &s[0..index];
            }
        }

        &s[..]
    }

    // The Problem here is that my_word and the first word index are not in sync
    // => if my_word changes, the word index isn't correct anymore.
}

fn _test_10() {
    struct UserData {
        active: bool,
        username: String,
        sign_in_count: u32,
    }

    let mut user_data = UserData {
        active: true,
        username: String::from("MRZ"),
        sign_in_count: 9,
    };

    user_data.username = String::from("Hello");

    fn build_user(username: String, sign_in_count: u32) -> UserData {
        return UserData {
            active: true,
            sign_in_count,
            username,
        };
    }
}

fn _test_11() {
    // thest methods in an Object (struct)
    // struct is basically a class in java

    struct Programmer {
        id: u16,
        email: String,
    }
    impl Programmer {
        fn _same_programmer(&self, other: Programmer) -> bool {
            self.email == other.email
        }
        fn email(&self) -> &String {
            &self.email
        }
        fn dummy() -> Self {
            Self {
                id: 000,
                email: String::from("dummy@gmail")
            }
        }
    }

    let programmer_1 = Programmer {
        id: 111,
        email: String::from("mrz@gmail"),
    };

    let programmer_2 = Programmer::dummy();

    if programmer_1._same_programmer(programmer_2) {
        println!("programmer_1 and programmer_2 are the same");
    }
}

fn test_12() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}