use std::io;

fn main() {
    println!("Please provide free text: ");
    let mut input = String::new();
    
    // &mut is a reference and allows it to be changed while &input would be immutable
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read the line");

    // Let creates a new var with an old var's name.
    let input = input.trim();

    println!("{}!", input);

}
