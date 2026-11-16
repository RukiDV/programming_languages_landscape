//! The Reading List logic, ported from TypeScript (see ../typescript/books.ts and ../SPEC.md).

use std::collections::BTreeMap;

use serde::Deserialize;

/// One book of the reading list.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub status: String,     // "want-to-read", "reading" or "done"
    pub rating: Option<u8>, // 1 to 5, None means "not rated"
    pub notes: String,
}

/// The label of every status. The key is the status, the value is the label.
pub fn status_labels() -> BTreeMap<String, String> {
    let mut labels = BTreeMap::new();
    labels.insert("want-to-read".to_string(), "Want to read".to_string());
    labels.insert("reading".to_string(), "Reading".to_string());
    labels.insert("done".to_string(), "Done".to_string());
    labels
}

/// `status` is "want-to-read", "reading", "done" or "all".
pub fn filter_books<'a>(books: &'a [Book], status: &str, query: &str) -> Vec<&'a Book> {
    let normalized_query = query.trim().to_lowercase();
    let mut result = Vec::new();
    for book in books {
        let status_matches = status == "all" || book.status == status;
        let title_matches = book.title.to_lowercase().contains(&normalized_query);
        if status_matches && title_matches {
            result.push(book);
        }
    }
    result
}

/// Returns an error message per invalid field (the keys "title" and "author").
/// An empty map means valid.
pub fn validate_book(title: &str, author: &str) -> BTreeMap<String, String> {
    let mut errors = BTreeMap::new();
    if title.trim().is_empty() {
        errors.insert("title".to_string(), "Title is required".to_string());
    }
    if author.trim().is_empty() {
        errors.insert("author".to_string(), "Author is required".to_string());
    }
    errors
}
