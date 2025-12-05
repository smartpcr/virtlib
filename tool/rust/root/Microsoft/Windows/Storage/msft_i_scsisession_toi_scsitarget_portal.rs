// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSISessionToiSCSITargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSISessionToiSCSITargetPortal {

/// 
    #[serde(rename = "iSCSISession")]
    pub i_scsisession: Option<MSFT_iSCSISession>,

/// 
    #[serde(rename = "iSCSITargetPortal")]
    pub i_scsitarget_portal: Option<MSFT_iSCSITargetPortal>,
}

impl MSFT_iSCSISessionToiSCSITargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsisession: None,
            i_scsitarget_portal: None,
        }
    }


    /// Sets the value of iSCSISession
    pub fn set_i_scsisession(&mut self, value: MSFT_iSCSISession) {
        self.i_scsisession = Some(value);
    }

    /// Gets the value of iSCSISession
    pub fn get_i_scsisession(&self) -> Option<&MSFT_iSCSISession> {
        self.i_scsisession.as_ref()
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

