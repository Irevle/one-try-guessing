use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn get_input(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Expected: Reading failed!");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        }
    }
}

fn main() {
    'main_loop: loop {
        println!(
            "It's your generic random number guessing game, but every time you fail to guess, the number changes! HO HO HO!"
        );

        let low_num = get_input("Input your lowest number: ");
        let high_num = get_input("Input your highest number: ");

        // checks if start number is higher than end number
        if low_num > high_num {
            println!("Lowest number can't be higher than the higher number")
        }

        let mut rng = rand::rng();

        if high_num - low_num >= 100 {
            println!("Good luck soldier.");
        }
        if high_num - low_num >= 50 {
            println!("Well isn't that interesting...");
        } else {
            println!(
                "Alright you've inputted your starting ({}) and ending ({}) numbers! Now guess!",
                low_num, high_num
            );
        }
        println!("\n---- Remember. You only have one shot at this. ----\n");
        let mut count: i32 = 0;

        loop {
            let n: i32 = rng.random_range(low_num..=high_num);
            let guess = get_input("Enter number >> ");

            if guess > high_num {
                println!(
                    "There goes an attempt. Your limit is {}... Now the answer is randomized again.",
                    high_num
                );
                count += 1;
            } else if guess < low_num {
                println!(
                    "An attempt wasted. Lowest number is {}... Now the answer is randomized again.",
                    low_num
                );
                count += 1;
            } else if guess != n {
                println!("Wrong. It was {}!", n);
                count += 1;
            } else {
                println!(
                    "Well hey, you did it! It's {}! It took you {} attempts!",
                    n, count
                );

                if count == 10 {
                    println!("=== You have guessed wrong 10 times. ===")
                } else if count == 25 {
                    println!("=== 25 tries. Feeling like giving up yet? ===")
                } else if count == 50 {
                    println!("=== That's the 50th. Persistent, aren't you? ===")
                } else if count == 100 {
                    println!("=== It's your 100th attempt. Such determination... ===")
                } else if count == 200 {
                    println!("=== Well, I'll be here until you leave or succeed. ===")
                }

                sleep(Duration::from_secs(2));

                'again_loop: loop {
                    print!("Another game, perhaps? (y/n): ");
                    let mut play_again_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut play_again_input)
                        .expect("Reading failed!");
                    let play_again = play_again_input.trim().to_lowercase();

                    match play_again.as_str() {
                        "y" => {
                            for _ in 0..20 {
                                println!(" ");
                            }
                            continue 'main_loop;
                        }
                        "n" => {
                            println!("Closing the game in 3 seconds...");
                            for i in (1..=3).rev() {
                                print!("{}... ", i);
                                io::stdout().flush().unwrap();
                                sleep(Duration::from_secs(1));
                            }
                            println!("Poof!");
                            break 'main_loop;
                        }
                        _ => {
                            println!("Invalid input. Please enter 'y' or 'n'!");
                            continue 'again_loop;
                        }
                    }
                }
            }
        }
    }
}
