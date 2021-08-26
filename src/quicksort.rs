use std::cmp::Ordering;
use std::ops::IndexMut;

use crate::{Length, Sort};

struct QuickSort;

impl QuickSort {
	fn quicksort<C, T>(&self, container: &mut C, lo: usize, hi: usize)
	where
		C: ?Sized + Length + IndexMut<usize, Output = T>,
		T: PartialOrd<T>,
	{
		if lo < hi {
			let p = self.partition(container, lo, hi);
			self.quicksort(container, lo, p);
			self.quicksort(container, p + 1, hi);
		}
	}

	fn partition<C, T>(&self, container: &mut C, lo: usize, hi: usize) -> usize
	where
		C: ?Sized + Length + IndexMut<usize, Output = T>,
		T: PartialOrd<T>,
	{
		let mi = (lo + hi) / 2;

		if container[mi] < container[lo] {
			let pa: *mut T = &mut container[lo];
			let pb: *mut T = &mut container[mi];

			unsafe {
				::std::ptr::swap(pa, pb);
			}
		}

		if container[hi] < container[lo] {
			let pa: *mut T = &mut container[lo];
			let pb: *mut T = &mut container[hi];

			unsafe {
				::std::ptr::swap(pa, pb);
			}
		}

		if container[mi] < container[hi] {
			let pa: *mut T = &mut container[mi];
			let pb: *mut T = &mut container[hi];

			unsafe {
				::std::ptr::swap(pa, pb);
			}
		}

		let pvt_idx = hi;

		let mut i: isize = lo as isize - 1;
		let mut j: isize = hi as isize + 1;

		loop {
			{
				let pivot = &container[pvt_idx];

				loop {
					i += 1;
					if let Some(Ordering::Less) =
						PartialOrd::partial_cmp(&container[i as usize], pivot)
					{
						// NOP
					} else {
						break;
					}
				}

				loop {
					j -= 1;
					if let Some(Ordering::Greater) =
						PartialOrd::partial_cmp(&container[j as usize], pivot)
					{
						// NOP
					} else {
						break;
					}
				}
			}

			if i >= j {
				return j as usize;
			}

			let pi: *mut T = &mut container[i as usize];
			let pj: *mut T = &mut container[j as usize];

			unsafe {
				::std::ptr::swap(pi, pj);
			}
		}
	}
}

impl<C, T> Sort<C> for QuickSort
where
	C: ?Sized + Length + IndexMut<usize, Output = T>,
	T: PartialOrd<T>,
{
	fn sort(&self, container: &mut C) {
		let len = container.length();
		self.quicksort(container, 0, len - 1);
	}
}

#[cfg(test)]
mod tests {
	use rand::prelude::*;

	use super::*;

	macro_rules! time {
		( $name:literal : $tt:expr ) => {
			let start = ::std::time::Instant::now();
			$tt;
			let elapsed = start.elapsed();
			println!(concat!($name, " {:?}"), elapsed);
		};
	}

	#[test]
	fn test_sort() {
		let mut container = [0u8; 128_000];
		thread_rng().fill_bytes(&mut container);

		// RUST normal sort
		let mut c_rs_s = container.clone();
		time!("std::sort" :  { c_rs_s.sort() });

		// RUST unstable sort
		let mut c_rs_u = container.clone();
		time!("std::sort_unstable" :  { c_rs_u.sort_unstable() });

		// QuickSort implementation
		let qs = QuickSort;
		let mut c_qs = container.clone();
		time!("quicksort" :  { qs.sort(&mut c_qs) });

		assert_eq!(c_qs, c_rs_s);
	}
}
