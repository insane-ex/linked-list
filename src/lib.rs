#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

mod list;
mod node;
mod node_allocator;
mod traits;

#[cfg(test)]
mod tests;

pub use list::LinkedList;
