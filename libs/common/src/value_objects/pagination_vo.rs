//! Pagination-related value objects
//!
//! This module contains value objects for pagination and data slicing.

use serde::{Deserialize, Serialize};

/// Pagination parameters with safety bounds
///
/// Automatically enforces reasonable limits to prevent abuse.
/// Page numbers are 1-indexed.
///
/// # Example
///
/// ```rust
/// use common::value_objects::Pagination;
///
/// let page = Pagination::new(1, 20);
/// assert_eq!(page.offset(), 0);
/// assert_eq!(page.limit(), 20);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page number (1-indexed)
    pub page: u32,
    /// Items per page (capped at 100)
    pub limit: u32,
}

impl Pagination {
    /// Create pagination with safety bounds
    ///
    /// - Page 0 or negative: set to 1
    /// - Limit > 100: capped at 100
    /// - Limit < 1: set to 1
    pub fn new(page: u32, limit: u32) -> Self {
        Self {
            page: page.max(1),
            limit: limit.min(100).max(1),
        }
    }

    /// Default pagination (page 1, limit 20)
    pub fn default() -> Self {
        Self::new(1, 20)
    }

    /// Get offset for database queries
    ///
    /// Converts 1-indexed page to 0-indexed offset.
    pub fn offset(&self) -> u64 {
        ((self.page - 1) as u64) * (self.limit as u64)
    }

    /// Get limit
    pub fn limit(&self) -> u32 {
        self.limit
    }

    /// Get page number
    pub fn page(&self) -> u32 {
        self.page
    }

    /// Check if this is the first page
    pub fn is_first_page(&self) -> bool {
        self.page == 1
    }

    /// Calculate total pages needed for given total count
    pub fn total_pages(&self, total_count: u64) -> u64 {
        (total_count as f64 / self.limit as f64).ceil() as u64
    }

    /// Check if page is the last page for given total count
    pub fn is_last_page(&self, total_count: u64) -> bool {
        let total_pages = self.total_pages(total_count);
        self.page as u64 >= total_pages
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::default()
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortDirection {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl SortDirection {
    /// Get SQL representation
    pub fn as_sql(&self) -> &str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}

/// Sort specification
///
/// Type parameter `T` represents the field being sorted.
/// Usually an enum of sortable fields.
///
/// # Example
///
/// ```rust
/// use common::value_objects::{Sort, SortDirection};
///
/// enum UserFields {
///     Name,
///     CreatedAt,
/// }
///
/// impl ToString for UserFields {
///     fn to_string(&self) -> String {
///         match self {
///             UserFields::Name => "name".to_string(),
///             UserFields::CreatedAt => "created_at".to_string(),
///         }
///     }
/// }
///
/// let sort = Sort::asc(UserFields::Name);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sort<T: ToString + Clone> {
    /// Field to sort by
    pub field: T,
    /// Direction to sort
    pub direction: SortDirection,
}

impl<T: ToString + Clone> Sort<T> {
    /// Create ascending sort
    pub fn asc(field: T) -> Self {
        Self {
            field,
            direction: SortDirection::Asc,
        }
    }

    /// Create descending sort
    pub fn desc(field: T) -> Self {
        Self {
            field,
            direction: SortDirection::Desc,
        }
    }

    /// Get the field name
    pub fn field_name(&self) -> String {
        self.field.to_string()
    }

    /// Get SQL ORDER BY clause
    pub fn to_sql(&self) -> String {
        format!("{} {}", self.field_name(), self.direction.as_sql())
    }
}

/// Search parameters combining pagination, sort, and filters
///
/// Provides a comprehensive query specification.
///
/// # Example
///
/// ```rust
/// use common::value_objects::{SearchParams, Sort, SortDirection};
///
/// enum OrderFields {
///     Id,
///     CreatedAt,
/// }
///
/// impl ToString for OrderFields {
///     fn to_string(&self) -> String {
///         match self {
///             OrderFields::Id => "id".to_string(),
///             OrderFields::CreatedAt => "created_at".to_string(),
///         }
///     }
/// }
///
/// let search = SearchParams::new()
///     .sort(Sort::desc(OrderFields::CreatedAt))
///     .query("buyer:john");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchParams<T: ToString + Clone> {
    /// Pagination settings
    pub pagination: Pagination,
    /// Sort specifications (applied in order)
    pub sorts: Vec<Sort<T>>,
    /// Search query string
    pub query: Option<String>,
}

impl<T: ToString + Clone> SearchParams<T> {
    /// Create search params with default pagination
    pub fn new() -> Self {
        Self {
            pagination: Pagination::default(),
            sorts: Vec::new(),
            query: None,
        }
    }

    /// Set pagination
    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = pagination;
        self
    }

    /// Add a sort specification
    pub fn sort(mut self, sort: Sort<T>) -> Self {
        self.sorts.push(sort);
        self
    }

    /// Set search query
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Check if there's a search query
    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    /// Get SQL ORDER BY clause from sorts
    pub fn to_sql_order(&self) -> Option<String> {
        if self.sorts.is_empty() {
            None
        } else {
            let order_clause = self
                .sorts
                .iter()
                .map(|s| s.to_sql())
                .collect::<Vec<_>>()
                .join(", ");
            Some(order_clause)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_safety_bounds() {
        let page = Pagination::new(0, 200);
        assert_eq!(page.page, 1);
        assert_eq!(page.limit, 100);
    }

    #[test]
    fn test_pagination_offset() {
        let page = Pagination::new(2, 10);
        assert_eq!(page.offset(), 10);
    }

    #[test]
    fn test_pagination_total_pages() {
        let page = Pagination::new(1, 20);
        assert_eq!(page.total_pages(100), 5);
        assert_eq!(page.total_pages(99), 5);
        assert_eq!(page.total_pages(100), 5);
    }

    #[test]
    fn test_sort_direction_sql() {
        assert_eq!(SortDirection::Asc.as_sql(), "ASC");
        assert_eq!(SortDirection::Desc.as_sql(), "DESC");
    }

    #[test]
    fn test_search_params() {
        #[derive(Clone)]
        struct Field(String);

        impl ToString for Field {
            fn to_string(&self) -> String {
                self.0.clone()
            }
        }

        let search: SearchParams<Field> = SearchParams::new().query("test");
        assert!(search.has_query());
    }
}
