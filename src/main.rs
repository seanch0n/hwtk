use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut fin = String::from("");
    // first arg is the name of the program so skip it
    for arg in args.iter().skip(1) {
        for (i, c) in arg.chars().enumerate() {
            if i % 2 == 0 {
                let a = c.to_string().to_uppercase();
                fin = fin + &a;
            } else {
                fin = fin + &c.to_string();
            }
        }
        fin = fin + &String::from(" ");
    }
    println!("{:?}", fin);

}
