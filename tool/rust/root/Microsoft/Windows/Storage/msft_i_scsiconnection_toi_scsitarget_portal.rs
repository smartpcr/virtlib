// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSIConnectionToiSCSITargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSIConnectionToiSCSITargetPortal {

/// 
    #[serde(rename = "iSCSIConnection")]
    pub i_scsiconnection: Option<MSFT_iSCSIConnection>,

/// 
    #[serde(rename = "iSCSITargetPortal")]
    pub i_scsitarget_portal: Option<MSFT_iSCSITargetPortal>,
}

impl MSFT_iSCSIConnectionToiSCSITargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsiconnection: None,
            i_scsitarget_portal: None,
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

    /// Sets the value of iSCSITargetPortal
    pub fn set_i_scsitarget_portal(&mut self, value: MSFT_iSCSITargetPortal) {
        self.i_scsitarget_portal = Some(value);
    }

    /// Gets the value of iSCSITargetPortal
    pub fn get_i_scsitarget_portal(&self) -> Option<&MSFT_iSCSITargetPortal> {
        self.i_scsitarget_portal.as_ref()
    }
}

