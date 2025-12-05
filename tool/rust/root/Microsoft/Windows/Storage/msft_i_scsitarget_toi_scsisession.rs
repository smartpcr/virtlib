// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSITargetToiSCSISession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSITargetToiSCSISession {

/// 
    #[serde(rename = "iSCSISession")]
    pub i_scsisession: Option<MSFT_iSCSISession>,

/// 
    #[serde(rename = "iSCSITarget")]
    pub i_scsitarget: Option<MSFT_iSCSITarget>,
}

impl MSFT_iSCSITargetToiSCSISession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsisession: None,
            i_scsitarget: None,
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

    /// Sets the value of iSCSITarget
    pub fn set_i_scsitarget(&mut self, value: MSFT_iSCSITarget) {
        self.i_scsitarget = Some(value);
    }

    /// Gets the value of iSCSITarget
    pub fn get_i_scsitarget(&self) -> Option<&MSFT_iSCSITarget> {
        self.i_scsitarget.as_ref()
    }
}

