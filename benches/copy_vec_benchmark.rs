use criterion::{criterion_group, criterion_main, Criterion};
use std::ptr;

struct CopyVec {}

//三种不同的方法实现两个&[8]数据的连接，看看性能如果
impl CopyVec {
	fn copy() -> Vec<u8> {
		let key1: [u8; 256] = [0; 256];
		let key2: [u8; 512] = [0; 512];
		let mut new_key = Vec::with_capacity(256 + 512 + 3);
		unsafe {
			let mut p = new_key.as_mut_ptr();
			ptr::copy(key1.as_ptr(), p, key1.len());
			p = p.offset(key1.len() as isize);
			*p = ':' as u8;
			*(p.offset(1)) = '_' as u8;
			*(p.offset(2)) = '-' as u8;
			p = p.offset(3);
			ptr::copy(key2.as_ptr(), p, key2.len());
			new_key.set_len(new_key.capacity());
		}
		new_key
	}
	fn vec() -> Vec<u8> {
		let key1: [u8; 256] = [0; 256];
		let key2: [u8; 512] = [0; 512];
		let mut new_key = Vec::with_capacity(256 + 512 + 3);
		new_key.extend_from_slice(&key1);
		new_key.push(':' as u8);
		new_key.push('_' as u8);
		new_key.push('_' as u8);
		new_key.extend_from_slice(&key2);
		new_key
	}
	fn copy_over() -> Vec<u8> {
		let key1: [u8; 256] = [0; 256];
		let key2: [u8; 512] = [0; 512];
		let mut new_key = Vec::with_capacity(256 + 512 + 3);
		unsafe {
			let mut p = new_key.as_mut_ptr();
			ptr::copy_nonoverlapping(key1.as_ptr(), p, key1.len());
			p = p.offset(key1.len() as isize);
			*p = ':' as u8;
			*(p.offset(1)) = '_' as u8;
			*(p.offset(2)) = '-' as u8;
			p = p.offset(3);
			ptr::copy_nonoverlapping(key2.as_ptr(), p, key2.len());
			new_key.set_len(new_key.capacity());
		}
		new_key
	}
}

pub fn copy_vec_benchmark(c: &mut Criterion) {
	c.bench_function("copy", |b| b.iter(|| CopyVec::copy()));
	c.bench_function("vec", |b| b.iter(|| CopyVec::vec()));
	c.bench_function("copy_over", |b| b.iter(|| CopyVec::copy_over()));
}

criterion_group!(benches, copy_vec_benchmark);
criterion_main!(benches);
