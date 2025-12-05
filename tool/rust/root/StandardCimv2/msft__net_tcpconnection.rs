// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetTCPConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetTCPConnection {
    #[serde(flatten)]
    pub base: MSFT_NetTransportConnection,

/// 
    #[serde(rename = "AppliedSetting")]
    pub applied_setting: Option<u8>,

/// 
    #[serde(rename = "OffloadState")]
    pub offload_state: Option<u8>,

/// 
    #[serde(rename = "RemoteAddress")]
    pub remote_address: Option<String>,

/// 
    #[serde(rename = "RemotePort")]
    pub remote_port: Option<u16>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u8>,
}

impl MSFT_NetTCPConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetTransportConnection::new(),
            applied_setting: None,
            offload_state: None,
            remote_address: None,
            remote_port: None,
            state: None,
        }
    }


    /// Sets the value of AppliedSetting
    pub fn set_applied_setting(&mut self, value: u8) {
        self.applied_setting = Some(value);
    }

    /// Gets the value of AppliedSetting
    pub fn get_applied_setting(&self) -> Option<&u8> {
        self.applied_setting.as_ref()
    }

    /// Sets the value of OffloadState
    pub fn set_offload_state(&mut self, value: u8) {
        self.offload_state = Some(value);
    }

    /// Gets the value of OffloadState
    pub fn get_offload_state(&self) -> Option<&u8> {
        self.offload_state.as_ref()
    }

    /// Sets the value of RemoteAddress
    pub fn set_remote_address(&mut self, value: String) {
        self.remote_address = Some(value);
    }

    /// Gets the value of RemoteAddress
    pub fn get_remote_address(&self) -> Option<&String> {
        self.remote_address.as_ref()
    }

    /// Sets the value of RemotePort
    pub fn set_remote_port(&mut self, value: u16) {
        self.remote_port = Some(value);
    }

    /// Gets the value of RemotePort
    pub fn get_remote_port(&self) -> Option<&u16> {
        self.remote_port.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u8) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u8> {
        self.state.as_ref()
    }
}

