fn main() {
    let mut index = 1;
    while index < 50 {
        if index % 15 == 0 {
            println!("FizzBuzz");
        } else if index % 3 == 0 {
            println!("Fizz");
        } else if index % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", index);
        }
        index = index + 1;
    }
}
