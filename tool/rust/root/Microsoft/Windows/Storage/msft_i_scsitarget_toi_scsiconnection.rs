// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSITargetToiSCSIConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSITargetToiSCSIConnection {

/// 
    #[serde(rename = "iSCSIConnection")]
    pub i_scsiconnection: Option<MSFT_iSCSIConnection>,

/// 
    #[serde(rename = "iSCSITarget")]
    pub i_scsitarget: Option<MSFT_iSCSITarget>,
}

impl MSFT_iSCSITargetToiSCSIConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsiconnection: None,
            i_scsitarget: None,
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

    /// Sets the value of iSCSITarget
    pub fn set_i_scsitarget(&mut self, value: MSFT_iSCSITarget) {
        self.i_scsitarget = Some(value);
    }

    /// Gets the value of iSCSITarget
    pub fn get_i_scsitarget(&self) -> Option<&MSFT_iSCSITarget> {
        self.i_scsitarget.as_ref()
    }
}

