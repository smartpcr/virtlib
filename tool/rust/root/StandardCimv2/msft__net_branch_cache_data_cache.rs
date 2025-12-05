// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheDataCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheDataCache {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCachePrimaryCache,

/// 
    #[serde(rename = "DataCacheExtensions")]
    pub data_cache_extensions: Vec<MSFT_NetBranchCacheDataCacheExtension>,
}

impl MSFT_NetBranchCacheDataCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCachePrimaryCache::new(),
            data_cache_extensions: Vec::new(),
        }
    }


    /// Sets the value of DataCacheExtensions
    pub fn set_data_cache_extensions(&mut self, value: Vec<MSFT_NetBranchCacheDataCacheExtension>) {
        self.data_cache_extensions = value;
    }

    /// Gets the value of DataCacheExtensions
    pub fn get_data_cache_extensions(&self) -> &Vec<MSFT_NetBranchCacheDataCacheExtension> {
        &self.data_cache_extensions
    }
}

