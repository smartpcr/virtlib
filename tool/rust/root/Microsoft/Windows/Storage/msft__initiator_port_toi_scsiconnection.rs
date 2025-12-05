// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_InitiatorPortToiSCSIConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_InitiatorPortToiSCSIConnection {

/// 
    #[serde(rename = "InitiatorPort")]
    pub initiator_port: Option<MSFT_InitiatorPort>,

/// 
    #[serde(rename = "iSCSIConnection")]
    pub i_scsiconnection: Option<MSFT_iSCSIConnection>,
}

impl MSFT_InitiatorPortToiSCSIConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_port: None,
            i_scsiconnection: None,
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

    /// Sets the value of iSCSIConnection
    pub fn set_i_scsiconnection(&mut self, value: MSFT_iSCSIConnection) {
        self.i_scsiconnection = Some(value);
    }

    /// Gets the value of iSCSIConnection
    pub fn get_i_scsiconnection(&self) -> Option<&MSFT_iSCSIConnection> {
        self.i_scsiconnection.as_ref()
    }
}

