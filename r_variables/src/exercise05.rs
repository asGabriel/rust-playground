pub fn constants() {
    const PI: f64 = 3.14159;
    const DAYS_IN_WEEK: i32 = 7;

    calculate_days_in_month(&DAYS_IN_WEEK, Months::Fev);

    println!("Value of PI: {}", PI);
}

fn calculate_days_in_month(DAYS_IN_WEEK: &i32, month: Months) {
    println!("{} and {:?}", DAYS_IN_WEEK, month);
}

#[derive(Debug)]
enum Months {
    Jan,
    Fev,
    Mar,
    Apr,
    Jun,
    Jul,
    Aug,
    Set,
    Oct,
    Dec,
}
