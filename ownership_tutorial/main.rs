fn main() {
    // let s = String::from("hello");

    // takes_ownership(s.clone());

    // let x = 5;

    // makes_copy(x);

    // println!("{}", x);

    // println!("{}", s);

    // let s1 = gives_ownership();

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2);

    // let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    // References

    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);

    // let mut s = String::from("hello");

    // change(&mut s);

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);

    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);

    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//
// fn takes_ownership(some_string: String) {
// println!("{}", some_string);
// }
//
// fn makes_copy(some_integer: i32) {
// println!("{}", some_integer);
// }
//
// fn gives_ownership() -> String {
// let some_string = String::from("yours");
//
// return some_string;
// }
//
// fn takes_and_gives_back(a_string: String) -> String {
// a_string
// }
//
// fn calculate_length(s: String) -> (String, usize) {
// let length = s.len();
//
// return (s, length);
// }

// fn calculate_length(s: &String) -> usize {
//     return s.len();
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
