// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSIConnectionToDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSIConnectionToDisk {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// 
    #[serde(rename = "iSCSIConnection")]
    pub i_scsiconnection: Option<MSFT_iSCSIConnection>,
}

impl MSFT_iSCSIConnectionToDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk: None,
            i_scsiconnection: None,
        }
    }


    /// Sets the value of Disk
    pub fn set_disk(&mut self, value: MSFT_Disk) {
        self.disk = Some(value);
    }

    /// Gets the value of Disk
    pub fn get_disk(&self) -> Option<&MSFT_Disk> {
        self.disk.as_ref()
    }

    /// Sets the value of iSCSIConnection
    pub fn set_i_scsiconnection(&mut self, value: MSFT_iSCSIConnection) {
        self.i_scsiconnection = Some(value);
    }

    /// Gets the value of iSCSIConnection
    pub fn get_i_scsiconnection(&self) -> Option<&MSFT_iSCSIConnection> {
        self.i_scsiconnection.as_ref()
    }
}

