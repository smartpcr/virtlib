// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETMemoryCache40_NETMemoryCache40 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETMemoryCache40_NETMemoryCache40 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CacheEntries")]
    pub cache_entries: Option<u32>,

/// 
    #[serde(rename = "CacheHitRatio")]
    pub cache_hit_ratio: Option<u32>,

/// 
    #[serde(rename = "CacheHitRatio_Base")]
    pub cache_hit_ratio__base: Option<u32>,

/// 
    #[serde(rename = "CacheHits")]
    pub cache_hits: Option<u32>,

/// 
    #[serde(rename = "CacheMisses")]
    pub cache_misses: Option<u32>,

/// 
    #[serde(rename = "CacheTrims")]
    pub cache_trims: Option<u32>,

/// 
    #[serde(rename = "CacheTurnoverRate")]
    pub cache_turnover_rate: Option<u32>,
}

impl Win32_PerfRawData_NETMemoryCache40_NETMemoryCache40 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            cache_entries: None,
            cache_hit_ratio: None,
            cache_hit_ratio__base: None,
            cache_hits: None,
            cache_misses: None,
            cache_trims: None,
            cache_turnover_rate: None,
        }
    }


    /// Sets the value of CacheEntries
    pub fn set_cache_entries(&mut self, value: u32) {
        self.cache_entries = Some(value);
    }

    /// Gets the value of CacheEntries
    pub fn get_cache_entries(&self) -> Option<&u32> {
        self.cache_entries.as_ref()
    }

    /// Sets the value of CacheHitRatio
    pub fn set_cache_hit_ratio(&mut self, value: u32) {
        self.cache_hit_ratio = Some(value);
    }

    /// Gets the value of CacheHitRatio
    pub fn get_cache_hit_ratio(&self) -> Option<&u32> {
        self.cache_hit_ratio.as_ref()
    }

    /// Sets the value of CacheHitRatio_Base
    pub fn set_cache_hit_ratio__base(&mut self, value: u32) {
        self.cache_hit_ratio__base = Some(value);
    }

    /// Gets the value of CacheHitRatio_Base
    pub fn get_cache_hit_ratio__base(&self) -> Option<&u32> {
        self.cache_hit_ratio__base.as_ref()
    }

    /// Sets the value of CacheHits
    pub fn set_cache_hits(&mut self, value: u32) {
        self.cache_hits = Some(value);
    }

    /// Gets the value of CacheHits
    pub fn get_cache_hits(&self) -> Option<&u32> {
        self.cache_hits.as_ref()
    }

    /// Sets the value of CacheMisses
    pub fn set_cache_misses(&mut self, value: u32) {
        self.cache_misses = Some(value);
    }

    /// Gets the value of CacheMisses
    pub fn get_cache_misses(&self) -> Option<&u32> {
        self.cache_misses.as_ref()
    }

    /// Sets the value of CacheTrims
    pub fn set_cache_trims(&mut self, value: u32) {
        self.cache_trims = Some(value);
    }

    /// Gets the value of CacheTrims
    pub fn get_cache_trims(&self) -> Option<&u32> {
        self.cache_trims.as_ref()
    }

    /// Sets the value of CacheTurnoverRate
    pub fn set_cache_turnover_rate(&mut self, value: u32) {
        self.cache_turnover_rate = Some(value);
    }

    /// Gets the value of CacheTurnoverRate
    pub fn get_cache_turnover_rate(&self) -> Option<&u32> {
        self.cache_turnover_rate.as_ref()
    }
}

