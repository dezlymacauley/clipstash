// SECTION: Listing the sub-directories that are in `src/lib/domain/clip`

pub mod field;

//_____________________________________________________________________________
// SECTION: Creating a struct that matches the schema of the SQLite database

use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {

    // Each clip will have a unique id
    // pub clip_id: field::ClipId,
    
    // The URL used to locate the clip
    // pub shortcode: field::ShortCode,

    // The actual clip itself
    // pub content: field::Content,

    // The title of the clip
    // pub title: field::Title,

    // The date that it was posted
    // pub posted: field::Posted,
    
    // When it expires (if it does) 
    // pub expires: field::Expires,

    // Password protection
    // pub password: field::Password,
    
    // The number of times that the clip is viewed
    // pub hits: field::Hits,

}

//_____________________________________________________________________________
/*
    // NOTE: Deserialize an Serialize
    
    ------------------------------------------------------------
    1. Deserialize 
    - JSON to Rust 
    - Client ===>>> Server

    Takes a data format like JSON,
    and converts it into a Rust object (like a struct).
    Takes a data format like JSON, and converts it to a Rust
    object like a struct

    This process is usually done when you receive data 
    from an external source (e.g., an API response) 
    and want to work with it in your Rust application.

    ------------------------------------------------------------
    2. Serialize
    - Rust to JSON
    - Client <<<=== Server

    Takes a Rust object (like a struct) 
    and converts it into a data format like JSON.

    Takes a Rust object, and converts it 
    to a data format like JSON

    This is typically done when you need to send 
    data to an external system (e.g., when making an API request).
    ------------------------------------------------------------

*/

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
