pub mod myserde;

fn main() {
    println!("Initializing rust-example");
    myserde::serialize_test();
    myserde::desserialize_test();
}
