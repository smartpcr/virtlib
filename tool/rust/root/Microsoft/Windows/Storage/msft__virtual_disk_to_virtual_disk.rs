// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDiskToVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDiskToVirtualDisk {
    #[serde(flatten)]
    pub base: MSFT_Synchronized,

/// 
    #[serde(rename = "SourceVirtualDisk")]
    pub source_virtual_disk: Option<MSFT_VirtualDisk>,

/// 
    #[serde(rename = "TargetVirtualDisk")]
    pub target_virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_VirtualDiskToVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Synchronized::new(),
            source_virtual_disk: None,
            target_virtual_disk: None,
        }
    }


    /// Sets the value of SourceVirtualDisk
    pub fn set_source_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.source_virtual_disk = Some(value);
    }

    /// Gets the value of SourceVirtualDisk
    pub fn get_source_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.source_virtual_disk.as_ref()
    }

    /// Sets the value of TargetVirtualDisk
    pub fn set_target_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.target_virtual_disk = Some(value);
    }

    /// Gets the value of TargetVirtualDisk
    pub fn get_target_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.target_virtual_disk.as_ref()
    }
}

