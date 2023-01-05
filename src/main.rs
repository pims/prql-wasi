use prql_compiler::compile;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut prql = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut prql).expect("failed to read input");
    let sql = compile(&prql);
    match sql {
        Ok(s) => println!("{}", s),
        Err(error) => {
            println!("error compiling prql to sql: {:?}", error);
            process::exit(1);
        }
    };
}