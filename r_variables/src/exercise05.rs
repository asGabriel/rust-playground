pub fn constants() {
    const PI: f64 = 3.14159;

    let r = 5.4;
    let cincunference_area = calculate_circunference_area(PI, r);
    println!(
        "Value of PI: {} and the Circunference area: {}",
        PI, cincunference_area
    );
}

fn calculate_circunference_area(pi: f64, r: f64) -> f64 {
    pi * r
}
