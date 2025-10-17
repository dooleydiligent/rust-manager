#rustmanager

The project is to be a rust-based html server "minimum viable product" deployed as a container.  The website will support basic authentication.  A logged in user will see a "Welcome to rust-manager" dashboard page.  An anonymous user will only see a landing page with a link to the login page.  User credentials are stored in an sqlite database with the password encrypted.

build platform: podman 3.4.4 (Only build a Containerfile)

The following `((Cargo.toml))` will be used, although the presence of a dependency in this file does not obligate the project to use it.
```
[package]
name = "rust-manager"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.6"
hyper = "1.7.0"
dioxus = { version = "0.6.2", features = ["web"] }
axum_session = { version = "0.17", features = ["key-store"] } # Use the appropriate version
axum_session_sqlx = { version = "0.6.0", features = ["sqlite"]}
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = "1.18.1"
argon2 = "0.5"
rand = "0.8"
```
Note: Use rand to generate a session key at runtime

All code will be committed to a monolith repository at https://github.com/dooleydiligent/rust-manager. Instructions will include git initialization of the repository. The username for github is dooleydiligent.

This is NOT a proof of concept project. The project will be code-complete and all requested features will be securely implemented (hence the term minimum viable product).

Assume the user will follow along using a newly installed ubuntu 22.04 server computer, but do not assume the user has installed any additional software.  You must explicitly state the commands to use to generate the project using cargo, update the Cargo.toml file, or other tool-based actions.

Therefore, the outline must follow a logical progression such as the following suggestion:
- Install buildtime environment (`apt install podman`, `rustup`, etc.)
- Generate Cargo.toml and rust source code;
- Generate Containerfile;
- Test build;
- Initialize and commit to git;

Generate the project now
