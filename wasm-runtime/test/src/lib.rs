#![no_std]
#![feature(lang_items)]
#![cfg_attr(feature = "strict", deny(warnings))]

#![feature(alloc)]
extern crate alloc;
use alloc::vec::Vec;

#[macro_use]
extern crate runtime_support;
use runtime_support::{set_storage, storage, print, blake2_256, twox_128, twox_256, ed25519_verify};

fn test_blake2_256(input: Vec<u8>) -> Vec<u8> {
	blake2_256(&input).to_vec()
}

fn test_twox_256(input: Vec<u8>) -> Vec<u8> {
	twox_256(&input).to_vec()
}

fn test_twox_128(input: Vec<u8>) -> Vec<u8> {
	twox_128(&input).to_vec()
}

fn test_ed25519_verify(input: Vec<u8>) -> Vec<u8> {
	let sig = &input[0..64];
	let pubkey = &input[64..96];
	let msg = b"all ok!";
	[ed25519_verify(sig, &msg[..], pubkey) as u8].to_vec()
}

fn test_data_in(input: Vec<u8>) -> Vec<u8> {
	print(b"set_storage" as &[u8]);
	set_storage(b"input", &input);

	print(b"storage" as &[u8]);
	let foo = storage(b"foo");

	print(b"set_storage" as &[u8]);
	set_storage(b"baz", &foo);

	print(b"finished!" as &[u8]);
	b"all ok!".to_vec()
}

impl_stubs!(test_data_in, test_blake2_256, test_twox_256, test_twox_128, test_ed25519_verify);
