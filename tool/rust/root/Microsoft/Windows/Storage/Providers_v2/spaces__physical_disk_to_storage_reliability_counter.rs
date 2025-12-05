// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SPACES_PhysicalDiskToStorageReliabilityCounter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SPACES_PhysicalDiskToStorageReliabilityCounter {
    #[serde(flatten)]
    pub base: MSFT_PhysicalDiskToStorageReliabilityCounter,
}

impl SPACES_PhysicalDiskToStorageReliabilityCounter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_PhysicalDiskToStorageReliabilityCounter::new(),
        }
    }

}

