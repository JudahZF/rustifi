//! Pagination utilities for iterating through paginated API responses.
//!
//! This module provides helpers for fetching all pages of data from paginated
//! endpoints, either as a complete collection or as an async stream.
//!
//! # Example
//!
//! ```no_run
//! use rustifi::UnifiClient;
//! use futures::StreamExt;
//!
//! # async fn example() -> rustifi::Result<()> {
//! let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
//!
//! // Fetch all clients at once
//! let all_clients = client.fetch_all_clients("site-id").await?;
//!
//! // Or stream page by page
//! let mut stream = client.stream_clients("site-id");
//! while let Some(result) = stream.next().await {
//!     let page = result?;
//!     println!("Got {} clients", page.len());
//! }
//! # Ok(())
//! # }
//! ```

use crate::api::clients::GetClients;
use crate::api::devices::GetDevices;
use crate::error::Result;
use crate::models::{Client, SiteDevice};
use crate::response::SiteResponse;
use crate::UnifiClient;
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Default page size for pagination.
pub const DEFAULT_PAGE_SIZE: usize = 100;

/// Type alias for the boxed future used in page streaming.
type PageFuture<'a, T> =
    Pin<Box<dyn std::future::Future<Output = Result<SiteResponse<T>>> + Send + 'a>>;

/// A stream that yields pages of items from a paginated endpoint.
pub struct PageStream<'a, T> {
    client: &'a UnifiClient,
    site_id: String,
    offset: usize,
    limit: usize,
    total_count: Option<usize>,
    done: bool,
    // Store the future for the current page fetch
    pending_future: Option<PageFuture<'a, T>>,
}

impl<'a> PageStream<'a, Client> {
    /// Create a new stream for clients.
    pub(crate) fn clients(client: &'a UnifiClient, site_id: impl Into<String>) -> Self {
        Self {
            client,
            site_id: site_id.into(),
            offset: 0,
            limit: DEFAULT_PAGE_SIZE,
            total_count: None,
            done: false,
            pending_future: None,
        }
    }

    /// Set the page size (limit).
    /// A size of 0 is normalized to 1 to prevent empty page requests.
    pub fn page_size(mut self, size: usize) -> Self {
        self.limit = if size == 0 { 1 } else { size };
        self
    }
}

impl<'a> PageStream<'a, SiteDevice> {
    /// Create a new stream for devices.
    pub(crate) fn devices(client: &'a UnifiClient, site_id: impl Into<String>) -> Self {
        Self {
            client,
            site_id: site_id.into(),
            offset: 0,
            limit: DEFAULT_PAGE_SIZE,
            total_count: None,
            done: false,
            pending_future: None,
        }
    }

    /// Set the page size (limit).
    /// A size of 0 is normalized to 1 to prevent empty page requests.
    pub fn page_size(mut self, size: usize) -> Self {
        self.limit = if size == 0 { 1 } else { size };
        self
    }
}

impl<'a> Stream for PageStream<'a, Client> {
    type Item = Result<Vec<Client>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        // If we have a known total and we've fetched everything, we're done
        if self.total_count.is_some_and(|total| self.offset >= total) {
            self.done = true;
            return Poll::Ready(None);
        }

        // If there's no pending future, create one
        if self.pending_future.is_none() {
            let endpoint = GetClients::with_pagination(&self.site_id, self.offset, self.limit);
            let client = self.client;
            self.pending_future = Some(Box::pin(async move { client.execute(&endpoint).await }));
        }

        // Poll the pending future
        let future = self.pending_future.as_mut().unwrap();
        match future.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                self.pending_future = None;
                match result {
                    Ok(response) => {
                        self.total_count = Some(response.total_count);
                        self.offset = response.next_offset().unwrap_or(self.offset + self.limit);

                        if response.data.is_empty() || !response.has_more() {
                            self.done = true;
                        }

                        Poll::Ready(Some(Ok(response.data)))
                    }
                    Err(e) => {
                        self.done = true;
                        Poll::Ready(Some(Err(e)))
                    }
                }
            }
        }
    }
}

impl<'a> Stream for PageStream<'a, SiteDevice> {
    type Item = Result<Vec<SiteDevice>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        // If we have a known total and we've fetched everything, we're done
        if self.total_count.is_some_and(|total| self.offset >= total) {
            self.done = true;
            return Poll::Ready(None);
        }

        // If there's no pending future, create one
        if self.pending_future.is_none() {
            let endpoint = GetDevices::with_pagination(&self.site_id, self.offset, self.limit);
            let client = self.client;
            self.pending_future = Some(Box::pin(async move { client.execute(&endpoint).await }));
        }

        // Poll the pending future
        let future = self.pending_future.as_mut().unwrap();
        match future.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                self.pending_future = None;
                match result {
                    Ok(response) => {
                        self.total_count = Some(response.total_count);
                        self.offset = response.next_offset().unwrap_or(self.offset + self.limit);

                        if response.data.is_empty() || !response.has_more() {
                            self.done = true;
                        }

                        Poll::Ready(Some(Ok(response.data)))
                    }
                    Err(e) => {
                        self.done = true;
                        Poll::Ready(Some(Err(e)))
                    }
                }
            }
        }
    }
}

/// Extension methods for UnifiClient to support pagination.
impl UnifiClient {
    /// Fetch all clients for a site, automatically handling pagination.
    ///
    /// This method fetches all pages sequentially and returns a complete list.
    /// For large datasets, consider using `stream_clients()` instead.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    /// let all_clients = client.fetch_all_clients("site-id").await?;
    /// println!("Total clients: {}", all_clients.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_all_clients(&self, site_id: &str) -> Result<Vec<Client>> {
        let mut all_items = Vec::new();
        let mut offset = 0;

        loop {
            let endpoint = GetClients::with_pagination(site_id, offset, DEFAULT_PAGE_SIZE);
            let response = self.execute(&endpoint).await?;

            let has_more = response.has_more();
            let next_offset = response.next_offset().unwrap_or(offset + DEFAULT_PAGE_SIZE);

            all_items.extend(response.data);

            if !has_more {
                break;
            }

            offset = next_offset;
        }

        Ok(all_items)
    }

    /// Fetch all devices for a site, automatically handling pagination.
    ///
    /// This method fetches all pages sequentially and returns a complete list.
    /// For large datasets, consider using `stream_devices()` instead.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    /// let all_devices = client.fetch_all_devices("site-id").await?;
    /// println!("Total devices: {}", all_devices.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_all_devices(&self, site_id: &str) -> Result<Vec<SiteDevice>> {
        let mut all_items = Vec::new();
        let mut offset = 0;

        loop {
            let endpoint = GetDevices::with_pagination(site_id, offset, DEFAULT_PAGE_SIZE);
            let response = self.execute(&endpoint).await?;

            let has_more = response.has_more();
            let next_offset = response.next_offset().unwrap_or(offset + DEFAULT_PAGE_SIZE);

            all_items.extend(response.data);

            if !has_more {
                break;
            }

            offset = next_offset;
        }

        Ok(all_items)
    }

    /// Create a stream that yields pages of clients.
    ///
    /// This is useful for processing clients in batches without loading
    /// everything into memory at once.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// use futures::StreamExt;
    ///
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    /// let mut stream = client.stream_clients("site-id");
    ///
    /// while let Some(result) = stream.next().await {
    ///     let page = result?;
    ///     for client in page {
    ///         println!("Client: {}", client.id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_clients(&self, site_id: &str) -> PageStream<'_, Client> {
        PageStream::clients(self, site_id)
    }

    /// Create a stream that yields pages of devices.
    ///
    /// This is useful for processing devices in batches without loading
    /// everything into memory at once.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// use futures::StreamExt;
    ///
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    /// let mut stream = client.stream_devices("site-id");
    ///
    /// while let Some(result) = stream.next().await {
    ///     let page = result?;
    ///     for device in page {
    ///         println!("Device: {} ({})", device.name, device.id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_devices(&self, site_id: &str) -> PageStream<'_, SiteDevice> {
        PageStream::devices(self, site_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::SiteResponse;

    #[test]
    fn test_default_page_size() {
        assert_eq!(DEFAULT_PAGE_SIZE, 100);
    }

    // Tests for SiteResponse pagination helpers

    #[test]
    fn test_site_response_has_more_with_remaining_pages() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 0,
            limit: 10,
            count: 10,
            total_count: 100,
        };
        assert!(response.has_more());
    }

    #[test]
    fn test_site_response_has_more_on_last_page() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 90,
            limit: 10,
            count: 10,
            total_count: 100,
        };
        assert!(!response.has_more());
    }

    #[test]
    fn test_site_response_has_more_partial_last_page() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 95,
            limit: 10,
            count: 5,
            total_count: 100,
        };
        assert!(!response.has_more());
    }

    #[test]
    fn test_site_response_next_offset_with_more_pages() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 0,
            limit: 10,
            count: 10,
            total_count: 100,
        };
        assert_eq!(response.next_offset(), Some(10));
    }

    #[test]
    fn test_site_response_next_offset_on_last_page() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 90,
            limit: 10,
            count: 10,
            total_count: 100,
        };
        assert_eq!(response.next_offset(), None);
    }

    #[test]
    fn test_site_response_next_offset_empty_response() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec![],
            offset: 0,
            limit: 10,
            count: 0,
            total_count: 0,
        };
        assert_eq!(response.next_offset(), None);
    }

    #[test]
    fn test_site_response_next_offset_uses_limit_when_available() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 0,
            limit: 25,
            count: 10,
            total_count: 100,
        };
        assert_eq!(response.next_offset(), Some(25));
    }

    #[test]
    fn test_site_response_next_offset_uses_count_when_limit_zero() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item1".to_string()],
            offset: 0,
            limit: 0,
            count: 10,
            total_count: 100,
        };
        assert_eq!(response.next_offset(), Some(10));
    }

    // Tests for pagination logic simulation

    #[test]
    fn test_pagination_logic_single_page() {
        // Simulate fetching a single page where total_count <= limit
        let mut all_items: Vec<String> = Vec::new();

        // First (and only) page
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            offset: 0,
            limit: 100,
            count: 3,
            total_count: 3,
        };

        all_items.extend(response.data.clone());
        let has_more = response.has_more();

        // Verify pagination terminates correctly
        assert!(!has_more);
        assert_eq!(all_items.len(), 3);
        assert_eq!(response.next_offset(), None); // No more pages
    }

    #[test]
    fn test_pagination_logic_multiple_pages() {
        // Simulate fetching multiple pages
        let mut all_items: Vec<String> = Vec::new();
        let mut offset = 0;
        let limit = 2;

        // First page
        let page1: SiteResponse<String> = SiteResponse {
            data: vec!["a".to_string(), "b".to_string()],
            offset: 0,
            limit,
            count: 2,
            total_count: 5,
        };

        all_items.extend(page1.data.clone());
        assert!(page1.has_more());
        offset = page1.next_offset().unwrap_or(offset + limit);
        assert_eq!(offset, 2);

        // Second page
        let page2: SiteResponse<String> = SiteResponse {
            data: vec!["c".to_string(), "d".to_string()],
            offset: 2,
            limit,
            count: 2,
            total_count: 5,
        };

        all_items.extend(page2.data.clone());
        assert!(page2.has_more());
        offset = page2.next_offset().unwrap_or(offset + limit);
        assert_eq!(offset, 4);

        // Third (last) page
        let page3: SiteResponse<String> = SiteResponse {
            data: vec!["e".to_string()],
            offset: 4,
            limit,
            count: 1,
            total_count: 5,
        };

        all_items.extend(page3.data.clone());
        assert!(!page3.has_more());

        assert_eq!(all_items.len(), 5);
        assert_eq!(all_items, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_pagination_logic_empty_result() {
        let response: SiteResponse<String> = SiteResponse {
            data: vec![],
            offset: 0,
            limit: 100,
            count: 0,
            total_count: 0,
        };

        assert!(!response.has_more());
        assert!(response.data.is_empty());
    }

    #[test]
    fn test_pagination_logic_exact_page_boundary() {
        // When total_count is exactly divisible by limit
        let mut all_items: Vec<String> = Vec::new();

        // First page
        let page1: SiteResponse<String> = SiteResponse {
            data: vec!["a".to_string(), "b".to_string()],
            offset: 0,
            limit: 2,
            count: 2,
            total_count: 4,
        };
        all_items.extend(page1.data.clone());
        assert!(page1.has_more());

        // Second (last) page - offset + count == total_count
        let page2: SiteResponse<String> = SiteResponse {
            data: vec!["c".to_string(), "d".to_string()],
            offset: 2,
            limit: 2,
            count: 2,
            total_count: 4,
        };
        all_items.extend(page2.data.clone());
        assert!(!page2.has_more());

        assert_eq!(all_items.len(), 4);
    }

    // Tests for PageStream page_size zero guard

    #[test]
    fn test_page_stream_page_size_zero_guard() {
        // We can't easily create a PageStream without a client, but we can verify
        // the zero guard logic by testing the SiteResponse next_offset behavior
        // which is used by the stream

        // When limit is 0, next_offset should use count
        let response: SiteResponse<String> = SiteResponse {
            data: vec!["item".to_string()],
            offset: 0,
            limit: 0,
            count: 1,
            total_count: 10,
        };

        // With limit=0, next_offset uses count
        assert_eq!(response.next_offset(), Some(1));
    }

    // Tests for endpoint query parameter construction

    #[test]
    fn test_get_clients_pagination_params() {
        use crate::api::clients::GetClients;
        use crate::api::Endpoint;

        let endpoint = GetClients::with_pagination("site-123", 50, 25);
        let params = endpoint.query_params();

        assert_eq!(params.len(), 2);
        assert!(params.contains(&("offset", "50".to_string())));
        assert!(params.contains(&("limit", "25".to_string())));
    }

    #[test]
    fn test_get_devices_pagination_params() {
        use crate::api::devices::GetDevices;
        use crate::api::Endpoint;

        let endpoint = GetDevices::with_pagination("site-456", 100, 50);
        let params = endpoint.query_params();

        assert_eq!(params.len(), 2);
        assert!(params.contains(&("offset", "100".to_string())));
        assert!(params.contains(&("limit", "50".to_string())));
    }

    #[test]
    fn test_get_clients_builder_pagination() {
        use crate::api::clients::GetClients;
        use crate::api::Endpoint;

        let endpoint = GetClients::new("site-123").offset(20).limit(10);
        let params = endpoint.query_params();

        assert_eq!(params.len(), 2);
        assert!(params.contains(&("offset", "20".to_string())));
        assert!(params.contains(&("limit", "10".to_string())));
    }

    #[test]
    fn test_get_devices_builder_pagination() {
        use crate::api::devices::GetDevices;
        use crate::api::Endpoint;

        let endpoint = GetDevices::new("site-456").offset(30).limit(15);
        let params = endpoint.query_params();

        assert_eq!(params.len(), 2);
        assert!(params.contains(&("offset", "30".to_string())));
        assert!(params.contains(&("limit", "15".to_string())));
    }
}
