// Copyright 2017 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Timestamp manager: just handles the current timestamp.

use storable::Storable;

/// Representation of a time.
pub type Timestamp = u64;

/// Get the current time.
pub fn get() -> Timestamp {
	Storable::lookup_default(b"tim:val")
}

/// Set the current time.
pub fn set(now: Timestamp) {
	now.store(b"tim:val")
}

#[cfg(test)]
mod tests {
	use joiner::Joiner;
	use keyedvec::KeyedVec;
	use runtime_support::{with_externalities, twox_128};
	use runtime::timestamp;
	use testing::TestExternalities;

	#[test]
	fn timestamp_works() {
		let mut t = TestExternalities { storage: map![
			twox_128(b"tim:val").to_vec() => vec![].join(&42u64)
		], };

		with_externalities(&mut t, || {
			assert_eq!(timestamp::get(), 42);
			timestamp::set(69);
			assert_eq!(timestamp::get(), 69);
		});
	}
}
