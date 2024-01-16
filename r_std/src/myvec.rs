pub fn testing_vec_methods() {
    let mut myvec = Vec::new();

    myvec.push(1);
    println!("{:?}", myvec);
    myvec.pop();
    println!("{:?}", myvec);
}