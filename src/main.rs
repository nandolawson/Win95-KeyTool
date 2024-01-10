/*Dependencies*/
use std::env;
use rand::Rng;

/*Code*/
fn main() {
	// Get arguments from terminal
	let args: Vec<String> = env::args()
		.collect();
	println!("[Win95 Key Tool]");
	// Check how many arguments are available
	if args
		.len() != 2 {
		eprintln!("Usage: {} <product key> / generate", args[0]);
		std::process::exit(1);
	}
	// Load argument into variable and check if it is called "generate"
	// Everything else will be checked if it is a valid product key
	let product_key: &str = args[1]
		.as_str();
	if product_key == "generate" {
		println!("Windows 95 key: {}", generate_product_key());
	}
	else if validate_product_key(product_key) {
		println!("Valid key: {}", product_key);
	}
	else {
		println!("Invalid key: {}", product_key);
	}
}
fn generate_product_key() -> String {
	let mut rng = rand::thread_rng();
	loop {
		// Generate first block of the product key
		let block_a: u32 = rng
			.gen_range(100..=998);
		// Check if it matches all rules
		if !(((3..=9).contains(&(block_a / 111))) && block_a % 111 == 0) {
			loop {
				// Generate second block of the product key
				let block_b: u32 = rng
					.gen_range(1000000..=8888888);
				// Check if it matches all rules
				if block_b
					.to_string()
					.chars()
					.filter_map(|c| c
						.to_digit(10)
					)
					.sum::<u32>() % 7 == 0 {
					// Merge both blocks and send it back to main function
					let product_key: String = format!("{}-{}", block_a, block_b);
					return product_key;
				}
			}
		}
	}
}
fn validate_product_key(product_key: &str) -> bool {
	// Get first block from product key
	let block_a: u32 = product_key[0..=2]
		.parse()
		.unwrap_or_default();
	// Get second block from product key
	let block_b: String = product_key[4..=10]
		.parse()
		.unwrap_or_default();
	// Multiple conditions to check
	if (product_key
			.len() != 11
		) ||
		((3..=9)
			.contains(&(block_a / 111)) &&
		block_a % 111 == 0
		) ||
		(product_key
			.chars()
			.nth(3) != Some('-')
		) ||
		(block_b
			.to_string()
			.chars()
			.filter_map(|c| c
				.to_digit(10)
			)
				.sum::<u32>() % 7 != 0) {
		// Check fails if any of the conditions is true
		return false;
	} else {
		// Check successful
		return true;
	}
}
