// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_OfflineFiles_ClientSideCaching struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_OfflineFiles_ClientSideCaching {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ApplicationBytesReadFromCache")]
    pub application_bytes_read_from_cache: Option<u64>,

/// 
    #[serde(rename = "ApplicationBytesReadFromServer")]
    pub application_bytes_read_from_server: Option<u64>,

/// 
    #[serde(rename = "ApplicationBytesReadFromServerNotCached")]
    pub application_bytes_read_from_server_not_cached: Option<u64>,

/// 
    #[serde(rename = "PrefetchBytesReadFromCache")]
    pub prefetch_bytes_read_from_cache: Option<u64>,

/// 
    #[serde(rename = "PrefetchBytesReadFromServer")]
    pub prefetch_bytes_read_from_server: Option<u64>,

/// 
    #[serde(rename = "PrefetchOperationsQueued")]
    pub prefetch_operations_queued: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheBytesPublished")]
    pub smbbranch_cache_bytes_published: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheBytesReceived")]
    pub smbbranch_cache_bytes_received: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheBytesRequested")]
    pub smbbranch_cache_bytes_requested: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheBytesRequestedFromServer")]
    pub smbbranch_cache_bytes_requested_from_server: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheHashBytesReceived")]
    pub smbbranch_cache_hash_bytes_received: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheHashesReceived")]
    pub smbbranch_cache_hashes_received: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashesRequested")]
    pub smbbranch_cache_hashes_requested: Option<u32>,
}

impl Win32_PerfRawData_OfflineFiles_ClientSideCaching {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            application_bytes_read_from_cache: None,
            application_bytes_read_from_server: None,
            application_bytes_read_from_server_not_cached: None,
            prefetch_bytes_read_from_cache: None,
            prefetch_bytes_read_from_server: None,
            prefetch_operations_queued: None,
            smbbranch_cache_bytes_published: None,
            smbbranch_cache_bytes_received: None,
            smbbranch_cache_bytes_requested: None,
            smbbranch_cache_bytes_requested_from_server: None,
            smbbranch_cache_hash_bytes_received: None,
            smbbranch_cache_hashes_received: None,
            smbbranch_cache_hashes_requested: None,
        }
    }


    /// Sets the value of ApplicationBytesReadFromCache
    pub fn set_application_bytes_read_from_cache(&mut self, value: u64) {
        self.application_bytes_read_from_cache = Some(value);
    }

    /// Gets the value of ApplicationBytesReadFromCache
    pub fn get_application_bytes_read_from_cache(&self) -> Option<&u64> {
        self.application_bytes_read_from_cache.as_ref()
    }

    /// Sets the value of ApplicationBytesReadFromServer
    pub fn set_application_bytes_read_from_server(&mut self, value: u64) {
        self.application_bytes_read_from_server = Some(value);
    }

    /// Gets the value of ApplicationBytesReadFromServer
    pub fn get_application_bytes_read_from_server(&self) -> Option<&u64> {
        self.application_bytes_read_from_server.as_ref()
    }

    /// Sets the value of ApplicationBytesReadFromServerNotCached
    pub fn set_application_bytes_read_from_server_not_cached(&mut self, value: u64) {
        self.application_bytes_read_from_server_not_cached = Some(value);
    }

    /// Gets the value of ApplicationBytesReadFromServerNotCached
    pub fn get_application_bytes_read_from_server_not_cached(&self) -> Option<&u64> {
        self.application_bytes_read_from_server_not_cached.as_ref()
    }

    /// Sets the value of PrefetchBytesReadFromCache
    pub fn set_prefetch_bytes_read_from_cache(&mut self, value: u64) {
        self.prefetch_bytes_read_from_cache = Some(value);
    }

    /// Gets the value of PrefetchBytesReadFromCache
    pub fn get_prefetch_bytes_read_from_cache(&self) -> Option<&u64> {
        self.prefetch_bytes_read_from_cache.as_ref()
    }

    /// Sets the value of PrefetchBytesReadFromServer
    pub fn set_prefetch_bytes_read_from_server(&mut self, value: u64) {
        self.prefetch_bytes_read_from_server = Some(value);
    }

    /// Gets the value of PrefetchBytesReadFromServer
    pub fn get_prefetch_bytes_read_from_server(&self) -> Option<&u64> {
        self.prefetch_bytes_read_from_server.as_ref()
    }

    /// Sets the value of PrefetchOperationsQueued
    pub fn set_prefetch_operations_queued(&mut self, value: u32) {
        self.prefetch_operations_queued = Some(value);
    }

    /// Gets the value of PrefetchOperationsQueued
    pub fn get_prefetch_operations_queued(&self) -> Option<&u32> {
        self.prefetch_operations_queued.as_ref()
    }

    /// Sets the value of SMBBranchCacheBytesPublished
    pub fn set_smbbranch_cache_bytes_published(&mut self, value: u64) {
        self.smbbranch_cache_bytes_published = Some(value);
    }

    /// Gets the value of SMBBranchCacheBytesPublished
    pub fn get_smbbranch_cache_bytes_published(&self) -> Option<&u64> {
        self.smbbranch_cache_bytes_published.as_ref()
    }

    /// Sets the value of SMBBranchCacheBytesReceived
    pub fn set_smbbranch_cache_bytes_received(&mut self, value: u64) {
        self.smbbranch_cache_bytes_received = Some(value);
    }

    /// Gets the value of SMBBranchCacheBytesReceived
    pub fn get_smbbranch_cache_bytes_received(&self) -> Option<&u64> {
        self.smbbranch_cache_bytes_received.as_ref()
    }

    /// Sets the value of SMBBranchCacheBytesRequested
    pub fn set_smbbranch_cache_bytes_requested(&mut self, value: u64) {
        self.smbbranch_cache_bytes_requested = Some(value);
    }

    /// Gets the value of SMBBranchCacheBytesRequested
    pub fn get_smbbranch_cache_bytes_requested(&self) -> Option<&u64> {
        self.smbbranch_cache_bytes_requested.as_ref()
    }

    /// Sets the value of SMBBranchCacheBytesRequestedFromServer
    pub fn set_smbbranch_cache_bytes_requested_from_server(&mut self, value: u64) {
        self.smbbranch_cache_bytes_requested_from_server = Some(value);
    }

    /// Gets the value of SMBBranchCacheBytesRequestedFromServer
    pub fn get_smbbranch_cache_bytes_requested_from_server(&self) -> Option<&u64> {
        self.smbbranch_cache_bytes_requested_from_server.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashBytesReceived
    pub fn set_smbbranch_cache_hash_bytes_received(&mut self, value: u64) {
        self.smbbranch_cache_hash_bytes_received = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashBytesReceived
    pub fn get_smbbranch_cache_hash_bytes_received(&self) -> Option<&u64> {
        self.smbbranch_cache_hash_bytes_received.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashesReceived
    pub fn set_smbbranch_cache_hashes_received(&mut self, value: u32) {
        self.smbbranch_cache_hashes_received = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashesReceived
    pub fn get_smbbranch_cache_hashes_received(&self) -> Option<&u32> {
        self.smbbranch_cache_hashes_received.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashesRequested
    pub fn set_smbbranch_cache_hashes_requested(&mut self, value: u32) {
        self.smbbranch_cache_hashes_requested = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashesRequested
    pub fn get_smbbranch_cache_hashes_requested(&self) -> Option<&u32> {
        self.smbbranch_cache_hashes_requested.as_ref()
    }
}

