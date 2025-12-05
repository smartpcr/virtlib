// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PortAuthParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PortAuthParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "RcvAuthorizationState")]
    pub rcv_authorization_state: Option<u32>,

/// 
    #[serde(rename = "RcvControlState")]
    pub rcv_control_state: Option<u32>,

/// 
    #[serde(rename = "SendAuthorizationState")]
    pub send_authorization_state: Option<u32>,

/// 
    #[serde(rename = "SendControlState")]
    pub send_control_state: Option<u32>,
}

impl MSNdis_PortAuthParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            header: None,
            rcv_authorization_state: None,
            rcv_control_state: None,
            send_authorization_state: None,
            send_control_state: None,
        }
    }


    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of RcvAuthorizationState
    pub fn set_rcv_authorization_state(&mut self, value: u32) {
        self.rcv_authorization_state = Some(value);
    }

    /// Gets the value of RcvAuthorizationState
    pub fn get_rcv_authorization_state(&self) -> Option<&u32> {
        self.rcv_authorization_state.as_ref()
    }

    /// Sets the value of RcvControlState
    pub fn set_rcv_control_state(&mut self, value: u32) {
        self.rcv_control_state = Some(value);
    }

    /// Gets the value of RcvControlState
    pub fn get_rcv_control_state(&self) -> Option<&u32> {
        self.rcv_control_state.as_ref()
    }

    /// Sets the value of SendAuthorizationState
    pub fn set_send_authorization_state(&mut self, value: u32) {
        self.send_authorization_state = Some(value);
    }

    /// Gets the value of SendAuthorizationState
    pub fn get_send_authorization_state(&self) -> Option<&u32> {
        self.send_authorization_state.as_ref()
    }

    /// Sets the value of SendControlState
    pub fn set_send_control_state(&mut self, value: u32) {
        self.send_control_state = Some(value);
    }

    /// Gets the value of SendControlState
    pub fn get_send_control_state(&self) -> Option<&u32> {
        self.send_control_state.as_ref()
    }
}

