1. **Prepare the build‑time environment**  
   1.1 Install the required packages on Ubuntu 22.04  
   1.2 Install Rust toolchain with `rustup` (stable, 1.90)  
   1.3 Install `podman` and `cargo-dioxus` for Dioxus support  

2. **Create a new Rust project**  
   2.1 Use `cargo new rust-manager --bin` to initialise the binary crate  
   2.2 Add required dependencies to `Cargo.toml` (dioxus, sqlite, bcrypt, actix‑web, etc.)  

3. **Implement the application logic**  
   3.1 Build an SQLite database helper that stores encrypted passwords (bcrypt)  
   3.2 Implement basic auth middleware (login, logout, session handling)  
   3.3 Create two Dioxus pages:  
       * Landing page with a link to `/login`  
       * Dashboard page (`/dashboard`) that welcomes the logged‑in user  
   3.4 Wire routes so that anonymous users see the landing page, and authenticated users see the dashboard  

4. **Create the container build file**  
   4.1 Write a `Containerfile` that builds the Rust binary and serves it with a lightweight web server (or directly run the binary)  

5. **Test the build locally**  
   5.1 Run `cargo build --release` to confirm compilation  
   5.2 Use `podman build .` to build the container image and verify it runs (`podman run -p 8080:8080 <image>`)

6. **Set up version control**  
   6.1 Initialise a Git repository in the project root (`git init`)  
   6.2 Add all source files and commit (`git add . && git commit -m "Initial rust-manager commit"`)  
   6.3 Create a remote repository on GitHub under user `dooleydiligent` and push (`git remote add origin ... && git push -u origin main`)  

7. **Document the project**  
   7.1 Write a brief README explaining how to build, run, and test the application.  
   7.2 Add any necessary notes for future development or deployment.