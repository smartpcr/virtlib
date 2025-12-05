// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_InitiatorPortToiSCSITarget struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_InitiatorPortToiSCSITarget {

/// 
    #[serde(rename = "InitiatorPort")]
    pub initiator_port: Option<MSFT_InitiatorPort>,

/// 
    #[serde(rename = "iSCSITarget")]
    pub i_scsitarget: Option<MSFT_iSCSITarget>,
}

impl MSFT_InitiatorPortToiSCSITarget {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_port: None,
            i_scsitarget: None,
        }
    }


    /// Sets the value of InitiatorPort
    pub fn set_initiator_port(&mut self, value: MSFT_InitiatorPort) {
        self.initiator_port = Some(value);
    }

    /// Gets the value of InitiatorPort
    pub fn get_initiator_port(&self) -> Option<&MSFT_InitiatorPort> {
        self.initiator_port.as_ref()
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

