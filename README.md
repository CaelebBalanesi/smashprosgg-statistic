# Smashpros.gg Stats Viewer

## Goals
- Easy to use interface.
- Gain subsurface level insights about the users Super Smash Bros Ultimate statistics.
- Be the main site for viewing [smashpros.gg](https://smashpros.gg/) stats.

## Tech Stack
Using ngx-charts to visualize different statistics.
- [Angular](https://angular.io/) - PWA framework to create website, mobile app, and desktop app from one codebase.
- [Ngx-Charts](https://swimlane.github.io/ngx-charts) - Angular library to generate visuals of data.
- [Axum](https://github.com/tokio-rs/axum) - The API framework for rust powering the backend.
- [Reqwest](https://github.com/seanmonstar/reqwest) - To pull data from the smashpros.gg API.
- [Sqlite](https://www.sqlite.org/index.html) - A lightweight database.
- [Rusqlite](https://github.com/rusqlite/rusqlite) - A way to bind Rust to the database.

## 