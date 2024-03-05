// Import the components defined in separate files
mod footer;
mod header;
mod navbar;
pub mod post;
pub mod posts;
mod row;
pub mod tag;

// expose modules
pub use footer::Footer;
pub use header::Header;
pub use navbar::Navbar;
pub use post::PostComponent;
pub use row::Row;
