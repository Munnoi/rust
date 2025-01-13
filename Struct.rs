struct Person {
  name: String,
  age: u32,
  is_student: bool,
}

impl Person { // impl is used when defining methods.
  fn greet(&self) {
    println!("Hi, my name is {}.", self.name);
  }
}

fn main() {
  let person1 = Person {
    name: String::from("Munno"),
    age: 22,
    is_student: true,
  };
  let person2 = Person {
    name: String::from("Tony"),
    ..person1 // copying the rest from person1.
  };
  println!(
    "{} is {} years old. {} is a student: {}",
    person1.name, person1.age, person1.name, person1.is_student
  );
  println!(
    "{} is {} years old. {} is a student: {}",
    person2.name, person2.age, person2.name, person2.is_student
  );
  person1.greet();
  person2.greet();
}
