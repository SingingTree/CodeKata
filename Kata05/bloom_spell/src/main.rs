extern crate crypto;

use crypto::digest::Digest;

fn bloom_string<D : Digest>(input : &str, digest : &D) {
	digest.input_str(input);
	digest.result_str();
}

fn main() {
    println!("Hello, world!");
}
