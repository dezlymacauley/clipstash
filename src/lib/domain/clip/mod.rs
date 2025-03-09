pub mod field;

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

// NOTE: Each one of these clips is going to be accessed within the field
// module that is why there each on has `field::` (path access operator)

// The Deserialize and Serialize traits are part of the `serde` crate
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
