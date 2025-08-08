use crate::parser::Document;

use super::context::is_post;

pub fn determine_template_name(document: &Document) -> String {
    if let Some(layout) = document.front_matter.extra.get("layout").and_then(|v| v.as_str()) {
        format!("{layout}.html")
    } else if is_post(document) {
        "post.html".to_string()
    } else {
        "page.html".to_string()
    }
}


