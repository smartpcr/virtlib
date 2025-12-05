// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities {

/// 
    #[serde(rename = "LsoV2Supported")]
    pub lso_v2_supported: Option<u32>,

/// 
    #[serde(rename = "ReceiveChecksumOffloadSupported")]
    pub receive_checksum_offload_supported: Option<u32>,

/// 
    #[serde(rename = "RssSupported")]
    pub rss_supported: Option<u32>,

/// 
    #[serde(rename = "TransmitChecksumOffloadSupported")]
    pub transmit_checksum_offload_supported: Option<u32>,

/// 
    #[serde(rename = "VmqSupported")]
    pub vmq_supported: Option<u32>,
}

impl MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            lso_v2_supported: None,
            receive_checksum_offload_supported: None,
            rss_supported: None,
            transmit_checksum_offload_supported: None,
            vmq_supported: None,
        }
    }


    /// Sets the value of LsoV2Supported
    pub fn set_lso_v2_supported(&mut self, value: u32) {
        self.lso_v2_supported = Some(value);
    }

    /// Gets the value of LsoV2Supported
    pub fn get_lso_v2_supported(&self) -> Option<&u32> {
        self.lso_v2_supported.as_ref()
    }

    /// Sets the value of ReceiveChecksumOffloadSupported
    pub fn set_receive_checksum_offload_supported(&mut self, value: u32) {
        self.receive_checksum_offload_supported = Some(value);
    }

    /// Gets the value of ReceiveChecksumOffloadSupported
    pub fn get_receive_checksum_offload_supported(&self) -> Option<&u32> {
        self.receive_checksum_offload_supported.as_ref()
    }

    /// Sets the value of RssSupported
    pub fn set_rss_supported(&mut self, value: u32) {
        self.rss_supported = Some(value);
    }

    /// Gets the value of RssSupported
    pub fn get_rss_supported(&self) -> Option<&u32> {
        self.rss_supported.as_ref()
    }

    /// Sets the value of TransmitChecksumOffloadSupported
    pub fn set_transmit_checksum_offload_supported(&mut self, value: u32) {
        self.transmit_checksum_offload_supported = Some(value);
    }

    /// Gets the value of TransmitChecksumOffloadSupported
    pub fn get_transmit_checksum_offload_supported(&self) -> Option<&u32> {
        self.transmit_checksum_offload_supported.as_ref()
    }

    /// Sets the value of VmqSupported
    pub fn set_vmq_supported(&mut self, value: u32) {
        self.vmq_supported = Some(value);
    }

    /// Gets the value of VmqSupported
    pub fn get_vmq_supported(&self) -> Option<&u32> {
        self.vmq_supported.as_ref()
    }
}

