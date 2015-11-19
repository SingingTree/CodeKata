extern crate crypto;
extern crate bit_vec;
use std::iter::repeat;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::md5::Md5;

use bit_vec::BitVec;

const BIT_VEC_SIZE : usize = 5000;

fn digest_string<D : Digest>(input : &str, digest : &mut D) -> Vec<u8> {
	digest.input_str(input);
	let output_bytes = digest.output_bytes();
	let mut buf: Vec<u8> = repeat(0).take(output_bytes).collect();
	digest.result(&mut buf);
	digest.reset();
	buf
}

fn bloom_hash(hash : &[u8], bit_vec : &mut BitVec) {
	if hash.len() < 4 {
		panic!("Need at least 4 bytes to bloom");
	}
	// Cut the hash into 4 chunks, then use them to index into the bit vec to bloom
	let chunk_size = hash.len() / 4;
	let mut hash_iter = hash.iter();

	for _ in 0..3 {
		let mut bit_vec_index : usize = 0;
		for _ in 0..chunk_size {
			bit_vec_index += (*hash_iter.next().unwrap()) as usize;
		}
		bit_vec_index = bit_vec_index % BIT_VEC_SIZE;
		bit_vec.set(bit_vec_index, true);
	}
}

fn main() {
	let mut md5_digest = Md5::new();
	let mut sha1_digest = Sha1::new();
	let mut bit_vec = BitVec::from_elem(BIT_VEC_SIZE, false);

	let md5_hash = digest_string("t", &mut md5_digest);

	println!("{:?}", md5_hash);
	bloom_hash(&md5_hash, &mut bit_vec);

	let sha1_hash = digest_string("t", &mut sha1_digest);
	println!("{:?}", sha1_hash);
}
