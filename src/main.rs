use std::time;
use std::time::UNIX_EPOCH;

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
struct Block <'a> {
	index: usize,
	timestamp: time::Duration, // in unix time
	transactions: &'a Vec<Transaction>,
	proof: usize,
	previous_hash: usize,
}

impl <'a>Block<'a> {
	fn new(index: usize, transactions: &'a Vec<Transaction>, proof: usize, previous_hash: usize) -> Block<'a> {
		let timestamp = match time::SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(t) => t,
			e => panic!("something wrong with time: {:?}", e)
		};
		Block {
			index,
			timestamp,
			transactions,
			proof,
			previous_hash,
		}
	}
}


struct Blockchain<'a> {
	//not sure on types yet
	chain: Vec<Block<'a>>,
	current_transactions: &'a mut Vec<Transaction>, 
}

impl <'a>Blockchain<'a> {
	fn new(current_transactions: &'a mut Vec<Transaction>) -> Blockchain<'a> {
		Blockchain {
			chain: Vec::new(),
			current_transactions,
		}
	}
	fn new_block(&mut self) {

		let new_block = Block::new(0, self.current_transactions, 100, 0);
		self.chain.push(new_block);
	}

	fn new_transaction(&mut self, sender: usize, recipient: usize, amount: usize) -> usize {
		self.current_transactions.push(
			Transaction {
				sender,
				recipient,
				amount,
			});

		self.last_block().index

	}

	fn last_block(&mut self) -> &Block {
		if let Some(last) = self.chain.first() {
			&last
		}
		else {
			self.new_block();
			self.last_block()
		}
	}
}

fn hash(block: Blockchain) {

}
