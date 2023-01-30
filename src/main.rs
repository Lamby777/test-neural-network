// Call lib.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
	test_neural_network::main(
		std::env::args().collect::<Vec<String>>()
	)
}
