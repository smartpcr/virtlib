// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetTransportConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetTransportConnection {
    #[serde(flatten)]
    pub base: CIM_NetworkPipe,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "LocalAddress")]
    pub local_address: Option<String>,

/// 
    #[serde(rename = "LocalPort")]
    pub local_port: Option<u16>,

/// 
    #[serde(rename = "OwningProcess")]
    pub owning_process: Option<u32>,
}

impl MSFT_NetTransportConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NetworkPipe::new(),
            creation_time: None,
            local_address: None,
            local_port: None,
            owning_process: None,
        }
    }


    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of LocalAddress
    pub fn set_local_address(&mut self, value: String) {
        self.local_address = Some(value);
    }

    /// Gets the value of LocalAddress
    pub fn get_local_address(&self) -> Option<&String> {
        self.local_address.as_ref()
    }

    /// Sets the value of LocalPort
    pub fn set_local_port(&mut self, value: u16) {
        self.local_port = Some(value);
    }

    /// Gets the value of LocalPort
    pub fn get_local_port(&self) -> Option<&u16> {
        self.local_port.as_ref()
    }

    /// Sets the value of OwningProcess
    pub fn set_owning_process(&mut self, value: u32) {
        self.owning_process = Some(value);
    }

    /// Gets the value of OwningProcess
    pub fn get_owning_process(&self) -> Option<&u32> {
        self.owning_process.as_ref()
    }
}

