use std::{mem,slice};

///A type which is representable as bytes
pub trait As{
	///Represent a pointer to a type as an array of bytes
	fn as_bytes(&self) -> &[u8];

	///Represent a mutable pointer to a type as a mutable array of bytes
	unsafe fn as_bytes_mut(&mut self) -> &mut [u8];
}

///A type which can be constructed from bytes unsafely
pub trait From{
	///Construct a type from an array of bytes
	//This is only safe if all byte values represents a state in the type (When each of the (2^number_of_bits) values are mapped to a state in the type). Insert a manual check when this is not the case.
	unsafe fn from_bytes(bytes: &[u8]) -> Self;
}

impl<T> As for T
	where T: Sized + Copy
{
	#[inline(always)]
	fn as_bytes(&self) -> &[u8]{
		unsafe{slice::from_raw_parts(
			self as *const T as *const u8,
			mem::size_of::<T>()
		)}
	}

	#[inline(always)]
	unsafe fn as_bytes_mut(&mut self) -> &mut [u8]{
		slice::from_raw_parts_mut(
			self as *mut T as *mut u8,
			mem::size_of::<T>()
		)
	}
}

impl<T> From for T
	where T: Sized + Copy
{
	#[inline(always)]
	unsafe fn from_bytes(bytes: &[u8]) -> Self{
		debug_assert!(bytes.len() >= mem::size_of::<T>());
		*(bytes.as_ptr() as *const T)
	}
}

#[test]
fn test(){
	let list: [u8;4] = [0x00,0x01,0x02,0x03];
	assert_eq!((list[0],list[1],list[2],list[3]),(0x00,0x01,0x02,0x03));

	let bytes = list.as_bytes();
	assert_eq!((bytes[0],bytes[1],bytes[2],bytes[3]),(0x00,0x01,0x02,0x03));

	let list: [u8;4] = unsafe{From::from_bytes(bytes)};
	assert_eq!((list[0],list[1],list[2],list[3]),(0x00,0x01,0x02,0x03));
}
