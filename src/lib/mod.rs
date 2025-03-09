// NOTE: This is a list of directories 
// that are inside the dirctory `src/lib/`

// This is like a table of contents for the library of the project
// 1. `mod` means module, which is simply a directory that contains Rust code
// 2. `pub mod` means that the directories listed 
// here are public rather than private (which is the default in Rust).

pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// Sometimes this mod.rs file is called, `lib.rs`

//_____________________________________________________________________________
