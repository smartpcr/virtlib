// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StoragePoolToResiliencySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StoragePoolToResiliencySetting {

/// 
    #[serde(rename = "ResiliencySetting")]
    pub resiliency_setting: Option<MSFT_ResiliencySetting>,

/// 
    #[serde(rename = "StoragePool")]
    pub storage_pool: Option<MSFT_StoragePool>,
}

impl MSFT_StoragePoolToResiliencySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            resiliency_setting: None,
            storage_pool: None,
        }
    }


    /// Sets the value of ResiliencySetting
    pub fn set_resiliency_setting(&mut self, value: MSFT_ResiliencySetting) {
        self.resiliency_setting = Some(value);
    }

    /// Gets the value of ResiliencySetting
    pub fn get_resiliency_setting(&self) -> Option<&MSFT_ResiliencySetting> {
        self.resiliency_setting.as_ref()
    }

    /// Sets the value of StoragePool
    pub fn set_storage_pool(&mut self, value: MSFT_StoragePool) {
        self.storage_pool = Some(value);
    }

    /// Gets the value of StoragePool
    pub fn get_storage_pool(&self) -> Option<&MSFT_StoragePool> {
        self.storage_pool.as_ref()
    }
}

