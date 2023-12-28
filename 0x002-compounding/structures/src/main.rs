fn main() {
    let person1 = Person {
        name: "Sualah Abdullai".to_string(),
        citizenship: "Pakistan".to_string(),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    println!(
        "Country: {} Name: {} tax: {}",
        person1.citizenship,
        person1.name,
        person1.compute_taxes()
    );

    let person2 = Person::new();
    println!(
        "Country: {} Name: {} tax: {}",
        person2.citizenship,
        person2.name,
        Person::compute_taxes(&person2)
    );

}

struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn new() -> Self {
        Person {
            citizenship: String::from("Ghana"),
            name: String::from("Hafsah"),
            age: 30,
            gender: 'F',
            salary: 40_000,
        }
    }
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}
