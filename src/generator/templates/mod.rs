//! Template rendering submodules

pub mod context;
pub mod paths;
pub mod select;
pub mod render_page;
pub mod render_index;

pub use render_page::{generate_page, generate_pages};
pub use render_index::generate_index;


