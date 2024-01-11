/*Dependencies*/
use std::env;
use win95_keytool::generate_product_key;
use win95_keytool::validate_product_key;

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
