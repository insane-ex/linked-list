#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

mod list;
mod node;
mod node_allocator;

pub use list::LinkedList;
