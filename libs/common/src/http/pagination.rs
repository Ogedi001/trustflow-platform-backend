use serde::{Deserialize, Serialize};

/// Pagination metadata for list endpoints
///
/// Follows the JSON:API style pagination pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page number (1-indexed)
    pub page: u64,

    /// Number of items per page
    pub per_page: u64,

    /// Total number of items across all pages
    pub total: u64,

    /// Total number of pages
    pub total_pages: u64,

    /// URL for the next page (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,

    /// URL for the previous page (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
}

impl Pagination {
    pub fn new(page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as u64;
        Self {
            page,
            per_page,
            total,
            total_pages: total_pages,
            next: None,
            prev: None,
        }
    }

    /// Set next page URL
    pub fn with_next(mut self, url: impl Into<String>) -> Self {
        self.next = Some(url.into());
        self
    }

    /// Set previous page URL
    pub fn with_prev(mut self, url: impl Into<String>) -> Self {
        self.prev = Some(url.into());
        self
    }
}

/// Helper for creating paginated responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The actual data items
    pub items: Vec<T>,

    /// Pagination metadata
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(items: Vec<T>, pagination: Pagination) -> Self {
        Self { items, pagination }
    }
}
