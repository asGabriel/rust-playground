pub fn variable_mutability() {
    let mut number = 5;
    let mut text = "Any text";


    println!("Before changing > Number: {}, Text: {}", number, text);
    
    number = 10;
    text = "Another text";

    println!("After changing > Number: {}, Text: {}", number, text);
}