use wgpu::BufferAsyncError;

use super::*;
use std::sync::{Arc, Mutex};


// pub trait ComputeData: Clone + Sized + bytemuck::Pod + std::marker::Send + std::marker::Sync 
// {
//     fn from_bytes(b: &[u8]) -> Self;
// }


// impl ComputeData for u8 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for i8 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for u16 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for i16 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for u32 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		//let input = &mut b.clone();
// 		//let (int_bytes, rest) = b.split_at(std::mem::size_of::<Self>())[0];
// 	    //*input = rest;
// 	    Self::from_ne_bytes(b.split_at(std::mem::size_of::<Self>()).0.try_into().unwrap())
// 	}
// }

// impl ComputeData for i32 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for f32 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for u64 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for i64 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for f64 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for u128 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }

// impl ComputeData for i128 {
// 	fn from_bytes(b: &[u8]) -> Self {
// 		let input = &mut b.clone();
// 		let (int_bytes, rest) = input.split_at(std::mem::size_of::<Self>());
// 	    *input = rest;
// 	    Self::from_ne_bytes(int_bytes.try_into().unwrap())
// 	}
// }


