// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_AFDCounters_MicrosoftWinsockBSP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_AFDCounters_MicrosoftWinsockBSP {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "DroppedDatagrams")]
    pub dropped_datagrams: Option<u32>,

/// 
    #[serde(rename = "DroppedDatagramsPersec")]
    pub dropped_datagrams_persec: Option<u32>,

/// 
    #[serde(rename = "RejectedConnections")]
    pub rejected_connections: Option<u32>,

/// 
    #[serde(rename = "RejectedConnectionsPersec")]
    pub rejected_connections_persec: Option<u32>,
}

impl Win32_PerfRawData_AFDCounters_MicrosoftWinsockBSP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            dropped_datagrams: None,
            dropped_datagrams_persec: None,
            rejected_connections: None,
            rejected_connections_persec: None,
        }
    }


    /// Sets the value of DroppedDatagrams
    pub fn set_dropped_datagrams(&mut self, value: u32) {
        self.dropped_datagrams = Some(value);
    }

    /// Gets the value of DroppedDatagrams
    pub fn get_dropped_datagrams(&self) -> Option<&u32> {
        self.dropped_datagrams.as_ref()
    }

    /// Sets the value of DroppedDatagramsPersec
    pub fn set_dropped_datagrams_persec(&mut self, value: u32) {
        self.dropped_datagrams_persec = Some(value);
    }

    /// Gets the value of DroppedDatagramsPersec
    pub fn get_dropped_datagrams_persec(&self) -> Option<&u32> {
        self.dropped_datagrams_persec.as_ref()
    }

    /// Sets the value of RejectedConnections
    pub fn set_rejected_connections(&mut self, value: u32) {
        self.rejected_connections = Some(value);
    }

    /// Gets the value of RejectedConnections
    pub fn get_rejected_connections(&self) -> Option<&u32> {
        self.rejected_connections.as_ref()
    }

    /// Sets the value of RejectedConnectionsPersec
    pub fn set_rejected_connections_persec(&mut self, value: u32) {
        self.rejected_connections_persec = Some(value);
    }

    /// Gets the value of RejectedConnectionsPersec
    pub fn get_rejected_connections_persec(&self) -> Option<&u32> {
        self.rejected_connections_persec.as_ref()
    }
}

