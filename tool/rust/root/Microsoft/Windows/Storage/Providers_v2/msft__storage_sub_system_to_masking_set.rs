// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToMaskingSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToMaskingSet {

/// 
    #[serde(rename = "MaskingSet")]
    pub masking_set: Option<MSFT_MaskingSet>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToMaskingSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            masking_set: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of MaskingSet
    pub fn set_masking_set(&mut self, value: MSFT_MaskingSet) {
        self.masking_set = Some(value);
    }

    /// Gets the value of MaskingSet
    pub fn get_masking_set(&self) -> Option<&MSFT_MaskingSet> {
        self.masking_set.as_ref()
    }

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

