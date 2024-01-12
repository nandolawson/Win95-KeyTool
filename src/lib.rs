use rand::Rng;

pub fn generate_product_key() -> String {
	let mut rng = rand::thread_rng();
	loop {
		// Generate first block of the product key
		let block_a: u32 = rng
			.gen_range(100..=998);

		// Check if it matches all rules
		if validate_block_a(block_a) {
			loop {

				// Generate second block of the product key
				let block_b: u32 = rng
					.gen_range(1111111..=8888888);
				// Check if it matches all rules
				if validate_block_b(block_b) {
					// Merge both blocks and send it back to main function
					let product_key: String = format!("{}-{}", block_a, block_b);
					return product_key;
				}
			}
		}
	}
}

pub fn validate_product_key(product_key: String) -> bool {
	// Get first block from product key
	let block_a: u32 = product_key[0..=2]
		.parse()
		.unwrap_or_default();
	// Get second block from product key
	let block_b: u32 = product_key[4..=10]
		.parse()
		.unwrap_or_default();

	// Multiple conditions to check
	match (
		validate_format(product_key),
		validate_block_a(block_a),
		validate_block_b(block_b)
	) {
		(true, true, true) => return true,
		_ => return false,
	}
}

fn validate_block_a(block_a: u32) -> bool {
	match (
		block_a.to_string().len() == 3,
		block_a % 111 != 0
		|| block_a / 111 < 3
		|| block_a / 111 > 9,
	) {
		(true, true) => return true,
		_ => return false,
	}
}

fn validate_block_b(block_b: u32) -> bool {
	if block_b // Are there exactly 7 digits?
		.to_string()
		.len() == 7
	&& block_b // Is the cross sum of block_b divisible by 7?
		.to_string()
		.chars()
		.filter_map(|c| c
			.to_digit(10)
		)
		.sum::<u32>() % 7 == 0
	{return true;} else {return false;}
}

fn validate_format(product_key: String) -> bool {
	if product_key // Are there exactly 11 digits?
		.len() == 11
	&& product_key // Is the 4th character a hyphen?
		.chars()
		.nth(3) == Some('-')
	{return true;} else {return false;}
}