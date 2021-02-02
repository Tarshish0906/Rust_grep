use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mygrep")]
struct MyGrep {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: String,
    #[structopt(short = "-n", long)]
    line_number: bool,
}

fn grep(mygrep: MyGrep, content: String) {
    let mut line_number = 1;
    for line in content.lines() {
        if line.contains(mygrep.pattern.as_str()) {
            if mygrep.line_number {
                println!("{}: {}", line_number, line);
            } else {
                println!("{}", line);
            }
        }
        line_number += 1;
    }
}

fn run(mygrep: MyGrep) {
    match std::fs::read_to_string(&mygrep.path) {
        Ok(content) => grep(mygrep, content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    let mygrep = MyGrep::from_args();
    run(mygrep);
}
