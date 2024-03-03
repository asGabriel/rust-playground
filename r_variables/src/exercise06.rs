pub fn variable_typing() {
    let num1 = 10;
    let num2 = 18.5;

    let num1_as_f64 = num1 as f64;

    println!("num1 plus num2 = {}", num2 + num1_as_f64);

    // exercise 6.2
    let value = 10;
    let mut message = String::from("Foo");

    let value_as_string = value.to_string();
    message.push_str(&value_as_string);

    println!("The message is {}", message);
}
