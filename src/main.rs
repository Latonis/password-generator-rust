extern crate rand;
use std::env;
use std::process;
use rand::Rng;

fn main() {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
                eprintln!("Usage: generator <length of password>");
		process::exit(1);
        }

        let pass_len = &args[1].parse::<usize>().unwrap();
	let mut rng = rand::thread_rng();
	const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789()-!@_#$%^+=ABCDEFGHIJKLMNOPQRSTUVWXYZ";

	let password: String = (0..*pass_len).map(|_| { let index = rng.gen_range(0, CHARSET.len()); CHARSET[index] as char}).collect();
	println!("Password: {}", password);
}

