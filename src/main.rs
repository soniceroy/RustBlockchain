fn main() {
    println!("Hello, world!");
}

struct Transaction {
	sender: usize,
	recipient: usize,
	amount: usize,
}


// each new block contains within itself the hash of the previous Block. 
// This is crucial because it’s what gives blockchains immutability: 
//   If an attacker corrupted an earlier Block in the chain then all subsequent
//   blocks will contain incorrect hashes.
struct Block {
	index: usize,
	timestamp: f64, // in unix time
	transactions: Vec<Transaction>,
	proof: usize,
	previous_hash: usize,
}


struct Blockchain {
	//not sure on types yet
	chain: Vec<Block>,
	current_transaction: Vec<Transaction>, 
}

impl Blockchain {
	fn new_block(&mut self) {

	}

	fn new_transaction(&mut self) {

	}

	fn last_block(& self) -> Option<&Block> {
		self.chain.first()
		/*
		if let Some(last) = self.chain.first() {
			Some(&last)
		}
		else {
			None
		}*/
	}
}

fn hash(block: Blockchain) {

}
