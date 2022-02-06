use std::io;
fn main() {
    let mut arr = vec![1,2,3,4,5,6,7];
    loop{
        println!("you have a array.\n {:?}", arr);
        println!("what are you want ?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error!");

        let index = match input.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => {
                println!("input type error!\n");
                continue;
            },
        };

        match arr.get(index as usize){
            Some(value) => println!("get element: {}\n", value),
            None => println!("no element.\n")
        }
    }
    
}
