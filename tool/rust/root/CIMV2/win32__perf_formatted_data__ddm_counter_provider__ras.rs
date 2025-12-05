// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_DdmCounterProvider_RAS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_DdmCounterProvider_RAS {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BytesReceivedByDisconnectedClients")]
    pub bytes_received_by_disconnected_clients: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedByDisconnectedClients")]
    pub bytes_transmitted_by_disconnected_clients: Option<u64>,

/// 
    #[serde(rename = "FailedAuthentications")]
    pub failed_authentications: Option<u32>,

/// 
    #[serde(rename = "MaxClients")]
    pub max_clients: Option<u32>,

/// 
    #[serde(rename = "TotalClients")]
    pub total_clients: Option<u32>,
}

impl Win32_PerfFormattedData_DdmCounterProvider_RAS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytes_received_by_disconnected_clients: None,
            bytes_transmitted_by_disconnected_clients: None,
            failed_authentications: None,
            max_clients: None,
            total_clients: None,
        }
    }


    /// Sets the value of BytesReceivedByDisconnectedClients
    pub fn set_bytes_received_by_disconnected_clients(&mut self, value: u64) {
        self.bytes_received_by_disconnected_clients = Some(value);
    }

    /// Gets the value of BytesReceivedByDisconnectedClients
    pub fn get_bytes_received_by_disconnected_clients(&self) -> Option<&u64> {
        self.bytes_received_by_disconnected_clients.as_ref()
    }

    /// Sets the value of BytesTransmittedByDisconnectedClients
    pub fn set_bytes_transmitted_by_disconnected_clients(&mut self, value: u64) {
        self.bytes_transmitted_by_disconnected_clients = Some(value);
    }

    /// Gets the value of BytesTransmittedByDisconnectedClients
    pub fn get_bytes_transmitted_by_disconnected_clients(&self) -> Option<&u64> {
        self.bytes_transmitted_by_disconnected_clients.as_ref()
    }

    /// Sets the value of FailedAuthentications
    pub fn set_failed_authentications(&mut self, value: u32) {
        self.failed_authentications = Some(value);
    }

    /// Gets the value of FailedAuthentications
    pub fn get_failed_authentications(&self) -> Option<&u32> {
        self.failed_authentications.as_ref()
    }

    /// Sets the value of MaxClients
    pub fn set_max_clients(&mut self, value: u32) {
        self.max_clients = Some(value);
    }

    /// Gets the value of MaxClients
    pub fn get_max_clients(&self) -> Option<&u32> {
        self.max_clients.as_ref()
    }

    /// Sets the value of TotalClients
    pub fn set_total_clients(&mut self, value: u32) {
        self.total_clients = Some(value);
    }

    /// Gets the value of TotalClients
    pub fn get_total_clients(&self) -> Option<&u32> {
        self.total_clients.as_ref()
    }
}

