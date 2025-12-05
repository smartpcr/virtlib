// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TargetPortToVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TargetPortToVirtualDisk {

/// 
    #[serde(rename = "TargetPort")]
    pub target_port: Option<MSFT_TargetPort>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_TargetPortToVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            target_port: None,
            virtual_disk: None,
        }
    }


    /// Sets the value of TargetPort
    pub fn set_target_port(&mut self, value: MSFT_TargetPort) {
        self.target_port = Some(value);
    }

    /// Gets the value of TargetPort
    pub fn get_target_port(&self) -> Option<&MSFT_TargetPort> {
        self.target_port.as_ref()
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

