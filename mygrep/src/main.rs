use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mygrep")]
struct MyGrep {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: String,
}

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(mygrep: MyGrep) {
    match std::fs::read_to_string(mygrep.path) {
        Ok(content) => grep(content, mygrep.pattern),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    let mygrep = MyGrep::from_arg();
    run(mygrep);
}
