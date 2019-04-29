extern crate rand;

use rand::Rng;
use std::io;

fn main() 
{
	let mut balance :u32 = 100_000;
	let mut color = String::new();

	loop
	{	
		let number = rand::thread_rng().gen_range(0,15);
		let mut win = false;
		let mut multi = 2;
		let bet = String::new();
		loop
		{
			let mut bet = String::new();
			println!("Please enter your bet: ");
			io::stdin().read_line(&mut bet)
			.expect("Please enter a valid value!");

			let bet :u32 = match bet.trim().parse()
			{
				Ok(bet) => bet,
				Err(_) => {
					println!("Please enter a valid value!");
					continue;}
			};

			if bet > balance
			{
				println!("You cannot afford this bet.");
				continue;
			}
			balance -= bet;
			break;
		}

		loop
		{
			println!("Please enter the color you wish to bet on: ");
			io::stdin().read_line(&mut color)
			.expect("Please enter a valid value!");

			match color.trim().as_ref()
			{
				"r" => {
					if number >= 1 && number <= 7{
						win = true;
						break;
					}
				}

				"g" => {
					if number == 0{
						win = true;
						multi = 14;
						break;
					}
				}

				"b" => {
					if number >= 8 && number <= 14{
						win = true;
						break;
					}
				}

				_ => {
					println!("Please enter a valid color! (r, g, b)");
					continue;
				}
			}
		}

		match win
		{
			true => 
			{
				println!("You won!");
				balance += bet.trim().parse::<u32>().unwrap()*multi;
			}
			_ => 
			{
				println!("You lost!");
			}
		}
	}
}
