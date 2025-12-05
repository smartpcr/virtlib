// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_EthernetPerfProvider_HyperVLegacyNetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_EthernetPerfProvider_HyperVLegacyNetworkAdapter {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesDropped")]
    pub bytes_dropped: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesSentPersec")]
    pub bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "FramesDropped")]
    pub frames_dropped: Option<u64>,

/// 
    #[serde(rename = "FramesReceivedPersec")]
    pub frames_received_persec: Option<u64>,

/// 
    #[serde(rename = "FramesSentPersec")]
    pub frames_sent_persec: Option<u64>,
}

impl Win32_PerfRawData_EthernetPerfProvider_HyperVLegacyNetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_dropped: None,
            bytes_received_persec: None,
            bytes_sent_persec: None,
            frames_dropped: None,
            frames_received_persec: None,
            frames_sent_persec: None,
        }
    }


    /// Sets the value of BytesDropped
    pub fn set_bytes_dropped(&mut self, value: u64) {
        self.bytes_dropped = Some(value);
    }

    /// Gets the value of BytesDropped
    pub fn get_bytes_dropped(&self) -> Option<&u64> {
        self.bytes_dropped.as_ref()
    }

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesSentPersec
    pub fn set_bytes_sent_persec(&mut self, value: u64) {
        self.bytes_sent_persec = Some(value);
    }

    /// Gets the value of BytesSentPersec
    pub fn get_bytes_sent_persec(&self) -> Option<&u64> {
        self.bytes_sent_persec.as_ref()
    }

    /// Sets the value of FramesDropped
    pub fn set_frames_dropped(&mut self, value: u64) {
        self.frames_dropped = Some(value);
    }

    /// Gets the value of FramesDropped
    pub fn get_frames_dropped(&self) -> Option<&u64> {
        self.frames_dropped.as_ref()
    }

    /// Sets the value of FramesReceivedPersec
    pub fn set_frames_received_persec(&mut self, value: u64) {
        self.frames_received_persec = Some(value);
    }

    /// Gets the value of FramesReceivedPersec
    pub fn get_frames_received_persec(&self) -> Option<&u64> {
        self.frames_received_persec.as_ref()
    }

    /// Sets the value of FramesSentPersec
    pub fn set_frames_sent_persec(&mut self, value: u64) {
        self.frames_sent_persec = Some(value);
    }

    /// Gets the value of FramesSentPersec
    pub fn get_frames_sent_persec(&self) -> Option<&u64> {
        self.frames_sent_persec.as_ref()
    }
}

