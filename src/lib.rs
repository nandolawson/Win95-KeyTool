use rand::Rng;

pub fn generate_product_key() -> String {
	let mut rng = rand::thread_rng();
	loop {
		let block_a: String = format!("{:03}", rng.gen_range(000..=998)); // Generate first block of the product key
		// Check if it matches all rules
		if validate_block_a(block_a.clone()) {
			loop {
				let block_b: String = format!("{:03}", rng.gen_range(0000000..=8888888)); // Generate second block of the product key
				// Check if it matches all rules
				if validate_block_b(block_b.clone()) {
					// Merge both blocks and send it back to main function
					let product_key: String = format!("{}-{}", block_a, block_b);
					return product_key;
				}
			}
		}
	}
}

pub fn validate_product_key(product_key: String) -> bool {
	match validate_format(product_key.clone()) {
		true => {
			let block_a: String = product_key[0..=2].to_string(); // Get first block from product key
			let block_b: String = product_key[4..=10].to_string(); // Get second block from product key
			return match (
				validate_block_a(block_a), // Check if first block is valid
				validate_block_b(block_b) // Check if second block is valid
			)
			{(true, true) => true, _ => false}
		}
		false => false
	}
}

fn validate_block_a(block_a: String) -> bool {
	return match (
		block_a.len() == 3,
		block_a.parse::<u32>().unwrap() / 111 < 3 || block_a.parse::<u32>().unwrap() / 111 > 9,
	) {(true, true) => true, _ => false}
}

fn validate_block_b(block_b: String) -> bool {
	return match (
		block_b.to_string().len() == 7,
		block_b.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() % 7 == 0,
	) {(true, true) => true,_ => false}
}

fn validate_format(product_key: String) -> bool {
	return match (
		product_key.len() == 11,
		product_key.chars().nth(3) == Some('-'),
	) {(true, true) => true, _ => false}
}
