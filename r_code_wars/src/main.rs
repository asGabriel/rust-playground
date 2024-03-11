mod exercise_01;
mod exercise_02;
mod exercise_03;
mod exercise04;

fn main() {
    println!("# Code Wars #");
    exercise_01::high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
    exercise_02::odd_or_even(vec![0, 1, 5]);
    exercise_03::xo("xxxm");
    exercise04::get_middle("testing");
}
