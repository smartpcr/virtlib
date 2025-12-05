// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Tcpip_UDPv6 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Tcpip_UDPv6 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "DatagramsNoPortPersec")]
    pub datagrams_no_port_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsPersec")]
    pub datagrams_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedErrors")]
    pub datagrams_received_errors: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedPersec")]
    pub datagrams_received_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsSentPersec")]
    pub datagrams_sent_persec: Option<u32>,
}

impl Win32_PerfRawData_Tcpip_UDPv6 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            datagrams_no_port_persec: None,
            datagrams_persec: None,
            datagrams_received_errors: None,
            datagrams_received_persec: None,
            datagrams_sent_persec: None,
        }
    }


    /// Sets the value of DatagramsNoPortPersec
    pub fn set_datagrams_no_port_persec(&mut self, value: u32) {
        self.datagrams_no_port_persec = Some(value);
    }

    /// Gets the value of DatagramsNoPortPersec
    pub fn get_datagrams_no_port_persec(&self) -> Option<&u32> {
        self.datagrams_no_port_persec.as_ref()
    }

    /// Sets the value of DatagramsPersec
    pub fn set_datagrams_persec(&mut self, value: u32) {
        self.datagrams_persec = Some(value);
    }

    /// Gets the value of DatagramsPersec
    pub fn get_datagrams_persec(&self) -> Option<&u32> {
        self.datagrams_persec.as_ref()
    }

    /// Sets the value of DatagramsReceivedErrors
    pub fn set_datagrams_received_errors(&mut self, value: u32) {
        self.datagrams_received_errors = Some(value);
    }

    /// Gets the value of DatagramsReceivedErrors
    pub fn get_datagrams_received_errors(&self) -> Option<&u32> {
        self.datagrams_received_errors.as_ref()
    }

    /// Sets the value of DatagramsReceivedPersec
    pub fn set_datagrams_received_persec(&mut self, value: u32) {
        self.datagrams_received_persec = Some(value);
    }

    /// Gets the value of DatagramsReceivedPersec
    pub fn get_datagrams_received_persec(&self) -> Option<&u32> {
        self.datagrams_received_persec.as_ref()
    }

    /// Sets the value of DatagramsSentPersec
    pub fn set_datagrams_sent_persec(&mut self, value: u32) {
        self.datagrams_sent_persec = Some(value);
    }

    /// Gets the value of DatagramsSentPersec
    pub fn get_datagrams_sent_persec(&self) -> Option<&u32> {
        self.datagrams_sent_persec.as_ref()
    }
}

