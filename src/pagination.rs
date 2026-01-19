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

    #[test]
    fn test_default_page_size() {
        assert_eq!(DEFAULT_PAGE_SIZE, 100);
    }
}
