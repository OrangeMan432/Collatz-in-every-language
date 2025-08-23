use std::io;
use std::time::SystemTime;

fn main() {
    println!("Input a starting number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line from stdin!");
    let mut num: i128 = input.trim().parse().expect("Error converting input to integer!");

    let mut count = 0;
    let mut peak_num = 0;

    let start = SystemTime::now();

    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = num * 3 + 1;
        }
        if num >= peak_num {
            peak_num = num;
        }
        count += 1
    }

    println!("Completed in {} seconds.", start.elapsed().expect("Error getting elapsed time!").as_secs_f64());
    println!("Completed after {} steps.", count);
    println!("Peak number was {}.", peak_num);
}
