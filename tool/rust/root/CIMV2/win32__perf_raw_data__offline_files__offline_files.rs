// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_OfflineFiles_OfflineFiles struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_OfflineFiles_OfflineFiles {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec_Base")]
    pub bytes_received_persec__base: Option<u32>,

/// 
    #[serde(rename = "BytesTransmitted")]
    pub bytes_transmitted: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedPersec")]
    pub bytes_transmitted_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedPersec_Base")]
    pub bytes_transmitted_persec__base: Option<u32>,
}

impl Win32_PerfRawData_OfflineFiles_OfflineFiles {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_received: None,
            bytes_received_persec: None,
            bytes_received_persec__base: None,
            bytes_transmitted: None,
            bytes_transmitted_persec: None,
            bytes_transmitted_persec__base: None,
        }
    }


    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u64) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u64> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesReceivedPersec_Base
    pub fn set_bytes_received_persec__base(&mut self, value: u32) {
        self.bytes_received_persec__base = Some(value);
    }

    /// Gets the value of BytesReceivedPersec_Base
    pub fn get_bytes_received_persec__base(&self) -> Option<&u32> {
        self.bytes_received_persec__base.as_ref()
    }

    /// Sets the value of BytesTransmitted
    pub fn set_bytes_transmitted(&mut self, value: u64) {
        self.bytes_transmitted = Some(value);
    }

    /// Gets the value of BytesTransmitted
    pub fn get_bytes_transmitted(&self) -> Option<&u64> {
        self.bytes_transmitted.as_ref()
    }

    /// Sets the value of BytesTransmittedPersec
    pub fn set_bytes_transmitted_persec(&mut self, value: u64) {
        self.bytes_transmitted_persec = Some(value);
    }

    /// Gets the value of BytesTransmittedPersec
    pub fn get_bytes_transmitted_persec(&self) -> Option<&u64> {
        self.bytes_transmitted_persec.as_ref()
    }

    /// Sets the value of BytesTransmittedPersec_Base
    pub fn set_bytes_transmitted_persec__base(&mut self, value: u32) {
        self.bytes_transmitted_persec__base = Some(value);
    }

    /// Gets the value of BytesTransmittedPersec_Base
    pub fn get_bytes_transmitted_persec__base(&self) -> Option<&u32> {
        self.bytes_transmitted_persec__base.as_ref()
    }
}

