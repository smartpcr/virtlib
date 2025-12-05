// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_InitiatorPortToiSCSISession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_InitiatorPortToiSCSISession {

/// 
    #[serde(rename = "InitiatorPort")]
    pub initiator_port: Option<MSFT_InitiatorPort>,

/// 
    #[serde(rename = "iSCSISession")]
    pub i_scsisession: Option<MSFT_iSCSISession>,
}

impl MSFT_InitiatorPortToiSCSISession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_port: None,
            i_scsisession: None,
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

    /// Sets the value of iSCSISession
    pub fn set_i_scsisession(&mut self, value: MSFT_iSCSISession) {
        self.i_scsisession = Some(value);
    }

    /// Gets the value of iSCSISession
    pub fn get_i_scsisession(&self) -> Option<&MSFT_iSCSISession> {
        self.i_scsisession.as_ref()
    }
}

