// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheDataCacheExtension struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheDataCacheExtension {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCacheSecondaryCache,
}

impl MSFT_NetBranchCacheDataCacheExtension {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCacheSecondaryCache::new(),
        }
    }

}

