
// Import the components defined in separate files
mod home;
mod blog;
mod projects;
mod contact_me;


// expose modules
pub use home::Home;
pub use blog::Blog;
pub use projects::Projects;
pub use contact_me::ContactMe;
