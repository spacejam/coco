//! Epoch-based memory reclamation
//!
//! TODO: Explain how EBR is used, with examples (maybe a simple treiber stack, with drop impl that
//! pins every X steps)
//!
//! TODO: A treiber stack that uses AtomicPtr instead of Atomic

// TODO: Debug for atomics, ptrs and pin
// TODO: swap method on atomics

#[macro_use(defer)]
extern crate scopeguard;

mod atomic;
mod garbage;
mod thread;
mod tagged_atomic;

pub use atomic::{Atomic, Ptr};
pub use garbage::Garbage;
pub use thread::{Pin, pin, defer_free};
pub use tagged_atomic::{TaggedAtomic, TaggedPtr};

// TODO: unit tests
