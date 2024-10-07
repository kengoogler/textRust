pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let num = args[1].parse::<i32>().unwrap();
    println!("{:?}", num);
    let num2 = args[2].parse::<i32>().unwrap();
    println!("{:?}", num2);
    let num3 = args[3].parse::<i32>().unwrap();
    println!("{:?}", num3);
}
