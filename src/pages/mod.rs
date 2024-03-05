// Import the components defined in separate files
mod blog;
mod contact_me;
mod home;
mod projects;

// expose modules
pub use blog::Blog;
pub use contact_me::ContactMe;
pub use home::Home;
pub use projects::Projects;
