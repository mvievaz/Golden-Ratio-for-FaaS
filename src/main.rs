use std::env;

fn fibonacci(x: i32) -> f32 {
    if x == 0 {
        return 0.0;
    } else if x == 1 {
        return 1.0;
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error on argument, using 20");
            20
        }
    };
    let golden_ratio = fibonacci(num + 1) / fibonacci(num);
    println!("Golden ratio for {num} is: {golden_ratio}");
}
