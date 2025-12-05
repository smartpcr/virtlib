// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCachePrimaryCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCachePrimaryCache {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCacheCache,

/// 
    #[serde(rename = "CurrentActiveCacheSize")]
    pub current_active_cache_size: Option<u64>,
}

impl MSFT_NetBranchCachePrimaryCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCacheCache::new(),
            current_active_cache_size: None,
        }
    }


    /// Sets the value of CurrentActiveCacheSize
    pub fn set_current_active_cache_size(&mut self, value: u64) {
        self.current_active_cache_size = Some(value);
    }

    /// Gets the value of CurrentActiveCacheSize
    pub fn get_current_active_cache_size(&self) -> Option<&u64> {
        self.current_active_cache_size.as_ref()
    }
}

