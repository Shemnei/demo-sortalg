#![allow(rustdoc::private_intra_doc_links)]
#![deny(
    // Documentation
	// TODO: rustdoc::broken_intra_doc_links,
	// TODO: rustdoc::missing_crate_level_docs,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,

    // Other
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	missing_debug_implementations,
	private_in_public,
	rust_2018_compatibility,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unstable_features,
	unused_import_braces,
	unused_qualifications,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]

pub mod quicksort;

pub trait Length {
	fn length(&self) -> usize;
}

impl<T> Length for [T] {
	fn length(&self) -> usize {
		self.len()
	}
}

impl<'a, T> Length for &'a [T] {
	fn length(&self) -> usize {
		self.len()
	}
}

impl<T, const N: usize> Length for [T; N] {
	fn length(&self) -> usize {
		N
	}
}

impl<'a, T, const N: usize> Length for &'a [T; N] {
	fn length(&self) -> usize {
		N
	}
}

pub trait Sort<C: ?Sized> {
	fn sort(&self, container: &mut C);
}
