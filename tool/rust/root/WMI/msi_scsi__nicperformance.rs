// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_NICPerformance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_NICPerformance {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u32>,

/// 
    #[serde(rename = "BytesTransmitted")]
    pub bytes_transmitted: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PDUReceived")]
    pub pdureceived: Option<u32>,

/// 
    #[serde(rename = "PDUTransmitted")]
    pub pdutransmitted: Option<u32>,
}

impl MSiSCSI_NICPerformance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            bytes_received: None,
            bytes_transmitted: None,
            instance_name: None,
            pdureceived: None,
            pdutransmitted: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u32) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u32> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesTransmitted
    pub fn set_bytes_transmitted(&mut self, value: u32) {
        self.bytes_transmitted = Some(value);
    }

    /// Gets the value of BytesTransmitted
    pub fn get_bytes_transmitted(&self) -> Option<&u32> {
        self.bytes_transmitted.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PDUReceived
    pub fn set_pdureceived(&mut self, value: u32) {
        self.pdureceived = Some(value);
    }

    /// Gets the value of PDUReceived
    pub fn get_pdureceived(&self) -> Option<&u32> {
        self.pdureceived.as_ref()
    }

    /// Sets the value of PDUTransmitted
    pub fn set_pdutransmitted(&mut self, value: u32) {
        self.pdutransmitted = Some(value);
    }

    /// Gets the value of PDUTransmitted
    pub fn get_pdutransmitted(&self) -> Option<&u32> {
        self.pdutransmitted.as_ref()
    }
}

