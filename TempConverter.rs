#[allow(unused_variables)]
fn main() {
 let celcius: i32 = 100;
 let farenheit: i32 = 10;
//  let temperature: i32 = c_to_f(celcius);
 let temperature: i32 = f_to_c(farenheit);
 println!("{}", temperature);
}

#[allow(dead_code)]
fn c_to_f(c: i32) -> i32 {
    let temp: i32 = (c * 9/5) + 32;
    temp
}
#[allow(dead_code)]
fn f_to_c(f: i32) -> i32 {
    let temp: i32 =  (f - 32) * 5/9;
    temp
}
