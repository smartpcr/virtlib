// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSISessionToiSCSIConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSISessionToiSCSIConnection {

/// 
    #[serde(rename = "iSCSIConnection")]
    pub i_scsiconnection: Option<MSFT_iSCSIConnection>,

/// 
    #[serde(rename = "iSCSISession")]
    pub i_scsisession: Option<MSFT_iSCSISession>,
}

impl MSFT_iSCSISessionToiSCSIConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsiconnection: None,
            i_scsisession: None,
        }
    }


    /// Sets the value of iSCSIConnection
    pub fn set_i_scsiconnection(&mut self, value: MSFT_iSCSIConnection) {
        self.i_scsiconnection = Some(value);
    }

    /// Gets the value of iSCSIConnection
    pub fn get_i_scsiconnection(&self) -> Option<&MSFT_iSCSIConnection> {
        self.i_scsiconnection.as_ref()
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

