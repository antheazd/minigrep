use std::env; //args function returns an iterator of the command line arguments passed to minigrep

fn main() {
    let args: Vec<String> = env::args().collect(); //collects command line arguments in a vector

    let query = &args[1];
    let file_path = &args[2];

    println!("Query: {}, File path: {}", query, file_path);

    //print whole line arguments
    for i in &args{
        print!("{i} ");
    }
}
