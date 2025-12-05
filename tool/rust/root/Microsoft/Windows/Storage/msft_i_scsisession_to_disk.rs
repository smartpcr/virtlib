// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSISessionToDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSISessionToDisk {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// 
    #[serde(rename = "iSCSISession")]
    pub i_scsisession: Option<MSFT_iSCSISession>,
}

impl MSFT_iSCSISessionToDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk: None,
            i_scsisession: None,
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

    /// Sets the value of iSCSISession
    pub fn set_i_scsisession(&mut self, value: MSFT_iSCSISession) {
        self.i_scsisession = Some(value);
    }

    /// Gets the value of iSCSISession
    pub fn get_i_scsisession(&self) -> Option<&MSFT_iSCSISession> {
        self.i_scsisession.as_ref()
    }
}

