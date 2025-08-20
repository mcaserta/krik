//! Template rendering submodules

pub mod context;
pub mod paths;
pub mod render_index;
pub mod render_page;
pub mod select;

pub use render_index::generate_index;
pub use render_page::{generate_page, generate_pages};
