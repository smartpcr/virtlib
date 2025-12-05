// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_WinNatCounters_WinNATInstance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_WinNatCounters_WinNATInstance {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TCPPortsAvailable")]
    pub tcpports_available: Option<u32>,

/// 
    #[serde(rename = "TCPPortsInUse")]
    pub tcpports_in_use: Option<u32>,

/// 
    #[serde(rename = "UDPPortsAvailable")]
    pub udpports_available: Option<u32>,

/// 
    #[serde(rename = "UDPPortsInUse")]
    pub udpports_in_use: Option<u32>,
}

impl Win32_PerfRawData_WinNatCounters_WinNATInstance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            tcpports_available: None,
            tcpports_in_use: None,
            udpports_available: None,
            udpports_in_use: None,
        }
    }


    /// Sets the value of TCPPortsAvailable
    pub fn set_tcpports_available(&mut self, value: u32) {
        self.tcpports_available = Some(value);
    }

    /// Gets the value of TCPPortsAvailable
    pub fn get_tcpports_available(&self) -> Option<&u32> {
        self.tcpports_available.as_ref()
    }

    /// Sets the value of TCPPortsInUse
    pub fn set_tcpports_in_use(&mut self, value: u32) {
        self.tcpports_in_use = Some(value);
    }

    /// Gets the value of TCPPortsInUse
    pub fn get_tcpports_in_use(&self) -> Option<&u32> {
        self.tcpports_in_use.as_ref()
    }

    /// Sets the value of UDPPortsAvailable
    pub fn set_udpports_available(&mut self, value: u32) {
        self.udpports_available = Some(value);
    }

    /// Gets the value of UDPPortsAvailable
    pub fn get_udpports_available(&self) -> Option<&u32> {
        self.udpports_available.as_ref()
    }

    /// Sets the value of UDPPortsInUse
    pub fn set_udpports_in_use(&mut self, value: u32) {
        self.udpports_in_use = Some(value);
    }

    /// Gets the value of UDPPortsInUse
    pub fn get_udpports_in_use(&self) -> Option<&u32> {
        self.udpports_in_use.as_ref()
    }
}

