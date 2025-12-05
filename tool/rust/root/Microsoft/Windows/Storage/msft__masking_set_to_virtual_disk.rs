// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MaskingSetToVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MaskingSetToVirtualDisk {

/// 
    #[serde(rename = "MaskingSet")]
    pub masking_set: Option<MSFT_MaskingSet>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_MaskingSetToVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            masking_set: None,
            virtual_disk: None,
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

    /// Sets the value of VirtualDisk
    pub fn set_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.virtual_disk = Some(value);
    }

    /// Gets the value of VirtualDisk
    pub fn get_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.virtual_disk.as_ref()
    }
}

