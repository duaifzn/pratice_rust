use std::io;
fn main() {
    loop {
        let mut name = String::new();
        println!("type name: ");
        io::stdin()
            .read_line(&mut name)
            .expect("input error");
        let name = name.trim();
        println!("{}", wellcome(&name));
    }
    
}

fn wellcome(name: &str) -> String{
    let a: String =  match name.chars().nth(0).unwrap() {
        'R' => name.to_string() + " plays banjo1",
        'r' => name.to_string() + " plays banjo2",
         _  => name.to_string() + " plays banjo3",
    };
    return a
}
