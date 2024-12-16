use std::io::stdin;

fn main() {
    loop {
        println!("Do you want to know how to keep a gullible person busy for hours? Y/N");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim().to_uppercase() == "Y" || input.trim().to_uppercase() == "YES" {
            continue;
        } else if input.trim().to_uppercase() == "N" || input.trim().to_uppercase() == "NO" {
            break;
        } else {
            println!("{} is not a valid yes/no response.", input.trim());
        }
    }
}
