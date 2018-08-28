

extern crate reqwest;
extern crate markov;

use markov::Chain;


fn main() {

	let mut chain = Chain::new();
	match chain.feed_file("../666_ibm.txt"){
		
		Err(e) => println!("{:?}", e),
		_ => ()
	};
	for i in chain.str_iter_for(5){
		println!("{}", i);
	}
}
