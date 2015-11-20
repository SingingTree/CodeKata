extern crate crypto;
extern crate bit_vec;
use std::iter::repeat;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use bit_vec::BitVec;

const BIT_VEC_SIZE : usize = 5000000;

fn read_file(file_name : &str) -> String {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
            Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
            Error::description(&why)),
        Ok(_) => s,
    }
}

fn bloom_lines<D : Digest>(words : &String, digest : &mut D, bit_vec : &mut BitVec) {
	let word_lines = words.lines();
	for word in word_lines {
		let hashed_word = digest_string(word, digest);
		bloom_hash(&hashed_word, bit_vec);
	}
}

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

fn check_hash(hash : &[u8], bit_vec : &BitVec) -> bool {
	if hash.len() < 4 {
		panic!("Need at least 4 bytes to bloom");
	}

	// Cut the hash into 4 chunks, then use them to index into the bit vec to bloom
	let chunk_size = hash.len() / 4;
	let mut hash_iter = hash.iter();

	let mut word_seen = true;

	for _ in 0..3 {
		let mut bit_vec_index : usize = 0;
		for _ in 0..chunk_size {
			bit_vec_index += (*hash_iter.next().unwrap()) as usize;
		}
		bit_vec_index = bit_vec_index % BIT_VEC_SIZE;
		if bit_vec[bit_vec_index] == false {
			word_seen = false;
		}
	}

	word_seen
}

fn check_spelling<D : Digest>(word : &String, digest : &mut D, bit_vec : &BitVec) -> bool {
	let hashed_word = digest_string(word, digest);
	check_hash(&hashed_word, &bit_vec)
}

fn main() {
	let mut sha1_digest = Sha1::new();
	let mut bit_vec = BitVec::from_elem(BIT_VEC_SIZE, false);

	let words = read_file("wordlist.txt");
	bloom_lines(&words, &mut sha1_digest, &mut bit_vec);

	println!("{} is word?: {}", "dog", check_spelling(&"dog".to_owned(), &mut sha1_digest, &bit_vec));
	println!("{} is word?: {}", "dogs", check_spelling(&"dogs".to_owned(), &mut sha1_digest, &bit_vec));
	println!("{} is word?: {}", "dogz", check_spelling(&"dogz".to_owned(), &mut sha1_digest, &bit_vec));
	println!("{} is word?: {}", "zzzz", check_spelling(&"zzzz".to_owned(), &mut sha1_digest, &bit_vec));
}
