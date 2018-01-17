use runtime_support::{Vec, size_of, transmute, uninitialized, slice};
use joiner::Joiner;
use endiansensitive::EndianSensitive;

/// Trait that allows zero-copy read/write of value-references to/from slices in LE format.
pub trait Slicable: Sized {
	fn from_slice(value: &[u8]) -> Option<Self> {
		Self::set_as_slice(|out| if value.len() == out.len() {
			out.copy_from_slice(&value);
			true
		} else {
			false
		})
	}
	fn to_vec(&self) -> Vec<u8> {
		self.as_slice_then(|s| s.to_vec())
	}
	fn set_as_slice<F: FnOnce(&mut[u8]) -> bool>(set_slice: F) -> Option<Self>;
	fn as_slice_then<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		f(&self.to_vec())
	}
	fn size_of(_value: &[u8]) -> Option<usize>;
}

pub trait NonTrivialSlicable: Slicable {}

impl<T: EndianSensitive> Slicable for T {
	fn set_as_slice<F: FnOnce(&mut[u8]) -> bool>(fill_slice: F) -> Option<Self> {
		let size = size_of::<T>();
		let mut result: T = unsafe { uninitialized() };
		let result_slice = unsafe {
			slice::from_raw_parts_mut(transmute::<*mut T, *mut u8>(&mut result), size)
		};
		if fill_slice(result_slice) {
			Some(result.from_le())
		} else {
			None
		}
	}
	fn as_slice_then<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		let size = size_of::<Self>();
		self.as_le_then(|le| {
			let value_slice = unsafe {
				slice::from_raw_parts(transmute::<*const Self, *const u8>(le), size)
			};
			f(value_slice)
		})
	}
	fn size_of(_value: &[u8]) -> Option<usize> {
		Some(size_of::<Self>())
	}
}

impl Slicable for Vec<u8> {
	fn from_slice(value: &[u8]) -> Option<Self> {
		Some(value[4..].to_vec())
	}
	fn set_as_slice<F: FnOnce(&mut[u8]) -> bool>(_fill_slice: F) -> Option<Self> {
		unimplemented!();
	}
	fn to_vec(&self) -> Vec<u8> {
		let mut r: Vec<u8> = Vec::new().join(&(self.len() as u32));
		r.extend_from_slice(&self);
		r
	}
	fn size_of(data: &[u8]) -> Option<usize> {
		u32::from_slice(&data[0..4]).map(|i| (i + 4) as usize)
	}
}