# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-02-03

### Added

- `push_front` and `push_back` methods for adding elements

## [0.2.0] - 2026-02-03

### Added

- `pop_front` and `pop_back` methods for removing elements

## [0.3.0] - 2026-02-03

### Added

- `is_empty` and `len` methods for inspecting the list
- `front` and `back` methods for accessing elements
- `front_mut` and `back_mut` methods for mutable access
- `contains` method to check if an element is present

## [0.4.0] - 2026-02-04

### Added

- `clear` method to remove all elements
- `reverse` method to reverse the order of elements
- `split` method to consume the list and return resulting split lists
- Private `remove_node` method to delete a specific node
- `retain` method to keep only elements that satisfy a predicate
- Implement `Drop` trait to automatically free the list and its elements
- Implement `Display` trait for printing the list
