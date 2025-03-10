//_____________________________________________________________________________
// SECTION: Listing the sub-directories that are in `src/lib/domain/`

pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// ABOUT: How modules and file imports work in Rust
/*

    NOTE: The sructure
    -------------------
    src/
    ├── lib/
    │   ├── data/
    │   │   ├── weapons_collected.rs
    │   │   ├── mod.rs
    │   ├── player_stats.rs
    │   ├── mod.rs


    NOTE: Modules

    `data` is a sub-directory located at `src/lib/data`

    `mod` tells Rust that the `data` directory is a module. 
    A module is just a directory that contains some Rust files,
    or a single .rs file.
    
    To avoid confusion, Rust will not allow you to have a file that has
    the same name as a sub directory.
    
    E.g. Rust will not compile if you try to do this:

    src/
    ├── lib/
    │   ├── data/
            | -- weapons_collected/
    │   │   ├── weapons_collected.rs
    │   │   ├── mod.rs
    │   ├── player_stats.rs
    │   ├── mod.rs

    In Rust spaces between words in a module name 
    must be separted by underscores. Hyphens are not allowed.

    `pub mod` means public module.
    This means that any Rust file that at the location: `src/lib`,
    can use the code from any file that is located in `src/lib/data` 

    ----------------------------------------------------------------
    NOTE: Imports

    E.g. If you had a file called `player_stats.rs`,
    that was located at `src/lib/player_stats.rs`

    You want to use code from a file called `weapons_collected.rs`,
    that is located at `src/lib/data/weapons_collected.rs`

    // Then inside `player_stats.rs`, you would add this line:
    // `use super::data::weapons_collected`
    // `super` is how to import weapons_collected.rs in using a relative path.
    // `super` in this case means "start from src/lib/"

    // If you wanted to import weapons_collected.rs 
    // with a relative path then you would have to do this:
    // use crate::lib::data::weapons_collected;
    // `crate` always means start from `src/`

*/
