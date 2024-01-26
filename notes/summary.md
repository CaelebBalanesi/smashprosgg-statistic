# Summary
A new matchmaking website came out recently for smash bros ultimate and its great. There really wasn't a good way to find games against good players besides going into discords and asking people if they wanna play. Smashpros.gg solved that problem but since they are in early state of development they don't have much when it comes to statistics about your games. They store raw data but they don't derive any interesting info from it and they don't show it to you. The goal for this project is to create the de facto standard statistics website for smashpros.gg.

# Methodology
The way I currently see this project working out is something similar to this flowchart.
```mermaid
flowchart TD
	user["User"] -->|Interacts| front["Angular Frontend"]
    front -->|Requests| back["Rust Backend"]
    back <-->|Queries| rsql["Rusqlite"]
    rsql <-->|Binds| db[(Database)]
    back -->|Requests| smash["Smashpros.gg API"]
```
The Rust backend will probably work similar to this.
```mermaid
flowchart TD
Main.rs -->|Inits| Api.rs
Api.rs -->|Pull Data| Connection.rs
Api.rs -->|Calculate| Statistics.rs
Connection.rs -->|Requests| smash["Smashpros.gg API"]
Connection.rs <-->|Cache| Database.rs
```
