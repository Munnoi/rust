fn main() {
  let employee = ("Molly", 32, "Marketing");
  // let name = employee.0;
  // let age = employee.1;
  // let dept = employee.2;

  let (name, age, dept) = employee;

  println!("Name: {name}, Age: {age}, department: {dept}");
}
