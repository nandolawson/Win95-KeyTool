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
	if validate_format(product_key.clone()) == true {
		// Get first block from product key
		let block_a: u32 = product_key[0..=2]
			.parse()
			.unwrap_or_default();
		// Get second block from product key
		let block_b: u32 = product_key[4..=10]
			.parse()
			.unwrap_or_default();
		// Multiple conditions to check
		return match (
			validate_block_a(block_a),
			validate_block_b(block_b)
		) {
			(true, true) => true, _ => false,}
	} else {return false;}
}

fn validate_block_a(block_a: u32) -> bool {
	return match (
		block_a.to_string().len() == 3,
		block_a % 111 != 0
		|| block_a / 111 < 3
		|| block_a / 111 > 9,
	) {(true, true) => true, _ => false,}
}

fn validate_block_b(block_b: u32) -> bool {
	return match (
		block_b.to_string().len() == 7,
		block_b
			.to_string()
			.chars()
			.filter_map(|c| c.to_digit(10))
			.sum::<u32>() % 7 == 0,
	) {(true, true) => true,_ => false,}
}

fn validate_format(product_key: String) -> bool {
	return match (
		product_key.len() == 11,
		product_key
			.chars()
			.nth(3) == Some('-'),
	) {(true, true) => true, _ => false,};
}