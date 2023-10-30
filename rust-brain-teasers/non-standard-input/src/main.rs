use std::io::stdin;

fn main() {
    println!("What is 3 + 2 ? Type your answer and press enter.");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Unable read standard input.");

    if input.trim() == "5" {
        // if input == "5" { // this is not correct.
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}

// A good rule of thumb is to never trust input;
// however, there are a few things you can do to minimize your
// problems when input is necessary.

//when working with strings, use trim to remove whitespace.

// when comparing strings, use to_lowercase or to_uppercase to ensuer that you're comparing string  in the same case.
// These functions take care of Unicode case-folding.
//
// When parsing complicated strings, use regular expressions to extract parts of a strings.

