fn fizzbuzz(n: u32) -> String {
    if n & 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else {
        format!("{}", n)
    }
}

fn main() {
    let output = (1..20)
        .map(fizzbuzz)
        .fold("".to_string(), |accum, line| format!("{}\n{}", accum, line));
    println!("{}", output);
}

#[test]
fn test_fizzbuzz_returns_fizzbuzz() {
    let expected = "FizzBuzz".to_string();
    let actual = fizzbuzz(15);
    assert_eq!(expected, actual);
}
