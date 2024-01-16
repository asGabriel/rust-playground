
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    age: u32
}

#[derive(Deserialize, Serialize, Debug)]
struct Dog {
    name: String,
    year_born: u32,
    owner: Person    
}

pub fn serialize_test() {
    println!("serializing");

    let my_dog = Dog {
        name: "Miley".to_string(),
        year_born: 2014,
        owner: Person {
            name: "Gabriel".to_string(),
            age: 20
        } 
    };

    let serr_dog: Result<String, serde_json::Error> = to_string_pretty(&my_dog);
    if serr_dog.is_ok() {
        println!("{}", serr_dog.unwrap());
    } else {
        println!("Dog could not be serialized");
    }


}

pub fn desserialize_test() {
    println!("desserializing");
    let json_string = r#"
    {
        "name": "Miley",
        "year_born": 2014,
        "owner": {
          "name": "Gabriel",
          "age": 20
        }
      }
    "#;

    let dess_json: Result<Dog, serde_json::Error> = from_str::<Dog>(json_string);
    if dess_json.is_ok() {
        println!("{:?}", dess_json.unwrap());
    } else {
        println!("Dog could not be desserialized");
    }
}