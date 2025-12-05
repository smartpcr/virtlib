// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSITargetToiSCSITargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSITargetToiSCSITargetPortal {

/// 
    #[serde(rename = "iSCSITarget")]
    pub i_scsitarget: Option<MSFT_iSCSITarget>,

/// 
    #[serde(rename = "iSCSITargetPortal")]
    pub i_scsitarget_portal: Option<MSFT_iSCSITargetPortal>,
}

impl MSFT_iSCSITargetToiSCSITargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsitarget: None,
            i_scsitarget_portal: None,
        }
    }


    /// Sets the value of iSCSITarget
    pub fn set_i_scsitarget(&mut self, value: MSFT_iSCSITarget) {
        self.i_scsitarget = Some(value);
    }

    /// Gets the value of iSCSITarget
    pub fn get_i_scsitarget(&self) -> Option<&MSFT_iSCSITarget> {
        self.i_scsitarget.as_ref()
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

