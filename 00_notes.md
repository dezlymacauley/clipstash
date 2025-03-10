### Initial structure
_______________________________________________________________________________

❯ tree --gitignore
.
├── 00_notes.md
├── Cargo.toml
├── migrations
│   └── 20210624130929_initial.sql
├── README.md
├── src
│   ├── bin
│   └── lib
│       ├── data
│       │   └── mod.rs
│       ├── domain
│       │   ├── clip
│       │   │   ├── field
│       │   │   │   └── mod.rs
│       │   │   └── mod.rs
│       │   └── mod.rs
│       ├── mod.rs
│       ├── service
│       │   └── mod.rs
│       └── web
│           └── mod.rs
├── static
│   ├── clipstash.css
│   ├── logo.svg
│   ├── tiny-date-picker.min.css
│   └── tiny-date-picker.min.js
└── templates
    ├── base.hbs
    ├── clip.hbs
    ├── clip_need_password.hbs
    ├── error_box.hbs
    ├── footer.hbs
    ├── header.hbs
    └── home.hbs

13 directories, 22 files

_______________________________________________________________________________
### Order of creation 

1. `src/lib/domain/clip` 


_______________________________________________________________________________
