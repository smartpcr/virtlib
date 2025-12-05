// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_InitiatorIdToVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_InitiatorIdToVirtualDisk {

/// 
    #[serde(rename = "InitiatorId")]
    pub initiator_id: Option<MSFT_InitiatorId>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_InitiatorIdToVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_id: None,
            virtual_disk: None,
        }
    }


    /// Sets the value of InitiatorId
    pub fn set_initiator_id(&mut self, value: MSFT_InitiatorId) {
        self.initiator_id = Some(value);
    }

    /// Gets the value of InitiatorId
    pub fn get_initiator_id(&self) -> Option<&MSFT_InitiatorId> {
        self.initiator_id.as_ref()
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

