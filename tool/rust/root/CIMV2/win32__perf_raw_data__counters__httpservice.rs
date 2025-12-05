// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_HTTPService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_HTTPService {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CurrentUrisCached")]
    pub current_uris_cached: Option<u32>,

/// 
    #[serde(rename = "TotalFlushedUris")]
    pub total_flushed_uris: Option<u32>,

/// 
    #[serde(rename = "TotalUrisCached")]
    pub total_uris_cached: Option<u32>,

/// 
    #[serde(rename = "UriCacheFlushes")]
    pub uri_cache_flushes: Option<u32>,

/// 
    #[serde(rename = "UriCacheHits")]
    pub uri_cache_hits: Option<u32>,

/// 
    #[serde(rename = "UriCacheMisses")]
    pub uri_cache_misses: Option<u32>,
}

impl Win32_PerfRawData_Counters_HTTPService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            current_uris_cached: None,
            total_flushed_uris: None,
            total_uris_cached: None,
            uri_cache_flushes: None,
            uri_cache_hits: None,
            uri_cache_misses: None,
        }
    }


    /// Sets the value of CurrentUrisCached
    pub fn set_current_uris_cached(&mut self, value: u32) {
        self.current_uris_cached = Some(value);
    }

    /// Gets the value of CurrentUrisCached
    pub fn get_current_uris_cached(&self) -> Option<&u32> {
        self.current_uris_cached.as_ref()
    }

    /// Sets the value of TotalFlushedUris
    pub fn set_total_flushed_uris(&mut self, value: u32) {
        self.total_flushed_uris = Some(value);
    }

    /// Gets the value of TotalFlushedUris
    pub fn get_total_flushed_uris(&self) -> Option<&u32> {
        self.total_flushed_uris.as_ref()
    }

    /// Sets the value of TotalUrisCached
    pub fn set_total_uris_cached(&mut self, value: u32) {
        self.total_uris_cached = Some(value);
    }

    /// Gets the value of TotalUrisCached
    pub fn get_total_uris_cached(&self) -> Option<&u32> {
        self.total_uris_cached.as_ref()
    }

    /// Sets the value of UriCacheFlushes
    pub fn set_uri_cache_flushes(&mut self, value: u32) {
        self.uri_cache_flushes = Some(value);
    }

    /// Gets the value of UriCacheFlushes
    pub fn get_uri_cache_flushes(&self) -> Option<&u32> {
        self.uri_cache_flushes.as_ref()
    }

    /// Sets the value of UriCacheHits
    pub fn set_uri_cache_hits(&mut self, value: u32) {
        self.uri_cache_hits = Some(value);
    }

    /// Gets the value of UriCacheHits
    pub fn get_uri_cache_hits(&self) -> Option<&u32> {
        self.uri_cache_hits.as_ref()
    }

    /// Sets the value of UriCacheMisses
    pub fn set_uri_cache_misses(&mut self, value: u32) {
        self.uri_cache_misses = Some(value);
    }

    /// Gets the value of UriCacheMisses
    pub fn get_uri_cache_misses(&self) -> Option<&u32> {
        self.uri_cache_misses.as_ref()
    }
}

