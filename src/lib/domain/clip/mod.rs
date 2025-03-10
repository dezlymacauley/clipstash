// SECTION: Listing the sub-directories that are in `src/lib/domain/clip`

// 1. `field` is a sub-directory `src/lib/domain/clip/field`
// 2. `mod` tells Rust that this is a module. 
// A module is just a directory that contains Rust files.
// 3. `pub` means public, which means that any Rust file that is inside
// src/lib/domain/clip/
// can use code from the directory src/lib/domain/clip/field 

// E.g. If you had a file like: `src/lib/domain/clip/example_file.rs`
// and you wanted to use code from: 
// `src/lib/domain/clip/field/awesome_code.rs` in `example_file.rs`

// Then inside `example_file.rs`, you would add this line:
// `use super::field::awesome_code`

// `super` is how to import awesome_code.rs in using a relative path.
// If you wanted to import awesome_code.rs with a relative path then you
// would have to do this:
// use crate::lib::domain::clip::field::awesome_code;

pub mod field;

//_____________________________________________________________________________
// SECTION: Creating a struct that matches the schema of the SQLite database



// pub struct Clip {
//
// }

//_____________________________________________________________________________

// Import the Deserialize, and Serialize traits from the serde crate
// use serde::{ Deserialize, Serialize };

// Import `Error` from this error
// use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum ClipError {
//     #[error("invalid password: {0}")]
//     InvalidPassword(String),
//
//     #[error("invalid title: {0}")]
//     InvalidTitle(String),
//     
//     #[error("empty content: {0}")]
//     EmptyContent,
//
//     #[error("invalid date: {0}")]
//     InvalidDate(String),
//     
//     #[error("date parse error: {0}")]
//     DateParse(#[from] chrono::ParseError),
//
//     #[error("id parse error: {0}")]
//     Id(#[from] uuid::Error),
//
//     #[error("hits parse error: {0}")]
//     Hits(#[from] std::num::TryFromIntError),
// }
//
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Clip {
//
//     // Each clip will have a unique id
//     pub clip_id: field::ClipId,
//     
//     // The URL used to locate the clip
//     pub shortcode: field::ShortCode,
//
//     // The actual clip itself
//     pub content: field::Content,
//
//     // The title of the clip
//     pub title: field::Title,
//
//     // The date that it was posted
//     pub posted: field::Posted,
//     
//     // When it expires (if it does) 
//     pub expires: field::Expires,
//
//     // Password protection
//     pub password: field::Password,
//     
//     // The number of times that the clip is viewed
//     pub hits: field::Hits,
//
// }


// NOTE: The Clip struct matches the structure of the database

/*

    CREATE TABLE IF NOT EXISTS clips
    (
        clip_id   TEXT PRIMARY KEY NOT NULL,
        shortcode TEXT UNIQUE NOT NULL,
        content   TEXT NOT NULL,
        title     TEXT,
        posted    DATETIME NOT NULL,
        expires   DATETIME,
        password  TEXT,
        hits      BIGINT NOT NULL
    );

*/
