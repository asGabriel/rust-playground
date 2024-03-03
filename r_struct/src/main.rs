fn main() {
    let person1 = Person::new("Gabriel".to_string(), 28);

    person1.introduce_yourself();
}

struct Person {
    name: String,
    age: u32,
}

trait ShowInformation {
    fn introduce_yourself(&self);
    fn make_dinner(&self);
}

impl ShowInformation for Person {
    fn introduce_yourself(&self) {
        println!(
            "Hi, my name is {} and i'm {} years old",
            self.name, self.age
        );
    }

    fn make_dinner(&self) {
        todo!()
    }
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self {
            name: name,
            age: age,
        }
    }
}
