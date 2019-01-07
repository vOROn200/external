/*!
Remote pointers.

# Rationale

Why is this abstraction useful, why not just use Rust's raw pointers?

These pointers point to memory in other processes; they are not valid within the process they are used from.
The pointers may not be of the same size, eg. 64bit process with a pointer to a 32bit process.

A decision was made to not support lifetimes, these are effectively raw pointers.
Reading and writing across processes at least prevent reading from invalid memory although the result may not be as expected if it happens to be reused for a different datastructure.

# Pointer types

There is both a raw pointer type and a typed pointer type.

Typed pointers allow the type system to assist you in preventing mistakes when interacting with this memory.

# Operations

All the pointer types implement these interfaces:

* Conversion between the underlying unsigned integer type and back to pointer type.

* Conversion between the raw and typed pointer types.

* Difference between two pointers of the same type; for raw pointers the difference is in bytes and for typed pointers the difference is in the number of elements between them.

* Adding and subtracting an unsigned integer offset resulting in the same pointer with specified offset. For typed pointers the addition is in number of elements.

* Display and Debug formatting.
 */

mod ptr64;
mod ptr32;

pub use self::ptr64::{RawPtr64, TypePtr64};
pub use self::ptr32::{RawPtr32, TypePtr32};

#[cfg(target_pointer_width = "64")]
pub type RawPtr = RawPtr64;
#[cfg(target_pointer_width = "64")]
pub type TypePtr<T> = TypePtr64<T>;

#[cfg(target_pointer_width = "32")]
pub type RawPtr = RawPtr32;
#[cfg(target_pointer_width = "32")]
pub type TypePtr<T> = TypePtr32<T>;

mod pod;
pub use self::pod::Pod;

/// Interact with pointers on the native target.
pub trait NativePtr: Sized {
	/// Converts the pointer to a `usize` value.
	fn into_usize(self) -> usize;
	/// Creates a pointer from a `usize` value.
	fn from_usize(address: usize) -> Self;
}

#[cfg(target_pointer_width = "64")]
impl NativePtr for RawPtr64 {
	fn into_usize(self) -> usize {
		self.into_u64() as usize
	}
	fn from_usize(address: usize) -> RawPtr64 {
		RawPtr64::from(address as u64)
	}
}
#[cfg(target_pointer_width = "64")]
impl<T: ?Sized> NativePtr for TypePtr64<T> {
	fn into_usize(self) -> usize {
		self.into_u64() as usize
	}
	fn from_usize(address: usize) -> TypePtr64<T> {
		TypePtr64::from(address as u64)
	}
}

impl NativePtr for RawPtr32 {
	fn into_usize(self) -> usize {
		self.into_u32() as usize
	}
	fn from_usize(address: usize) -> RawPtr32 {
		RawPtr32::from(address as u32)
	}
}
impl<T: ?Sized> NativePtr for TypePtr32<T> {
	fn into_usize(self) -> usize {
		self.into_u32() as usize
	}
	fn from_usize(address: usize) -> TypePtr32<T> {
		TypePtr32::from(address as u32)
	}
}

impl From<RawPtr32> for RawPtr64 {
	fn from(ptr: RawPtr32) -> RawPtr64 {
		RawPtr64::from(ptr.into_u32() as u64)
	}
}
impl<T: ?Sized> From<TypePtr32<T>> for TypePtr64<T> {
	fn from(ptr: TypePtr32<T>) -> TypePtr64<T> {
		TypePtr64::from(ptr.into_u32() as u64)
	}
}
