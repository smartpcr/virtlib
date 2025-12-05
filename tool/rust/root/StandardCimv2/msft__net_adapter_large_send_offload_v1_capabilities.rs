// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterLargeSendOffloadV1Capabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterLargeSendOffloadV1Capabilities {

/// 
    #[serde(rename = "IPv4Encapsulation")]
    pub ipv4_encapsulation: Option<MSFT_NetAdapterLsoEncapsulationTypes>,

/// 
    #[serde(rename = "IPv4IpOptionsSupported")]
    pub ipv4_ip_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4MaxOffloadSizeSupported")]
    pub ipv4_max_offload_size_supported: Option<u32>,

/// 
    #[serde(rename = "IPv4MinSegmentCountSupported")]
    pub ipv4_min_segment_count_supported: Option<u32>,

/// 
    #[serde(rename = "IPv4TcpOptionsSupported")]
    pub ipv4_tcp_options_supported: Option<bool>,
}

impl MSFT_NetAdapterLargeSendOffloadV1Capabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ipv4_encapsulation: None,
            ipv4_ip_options_supported: None,
            ipv4_max_offload_size_supported: None,
            ipv4_min_segment_count_supported: None,
            ipv4_tcp_options_supported: None,
        }
    }


    /// Sets the value of IPv4Encapsulation
    pub fn set_ipv4_encapsulation(&mut self, value: MSFT_NetAdapterLsoEncapsulationTypes) {
        self.ipv4_encapsulation = Some(value);
    }

    /// Gets the value of IPv4Encapsulation
    pub fn get_ipv4_encapsulation(&self) -> Option<&MSFT_NetAdapterLsoEncapsulationTypes> {
        self.ipv4_encapsulation.as_ref()
    }

    /// Sets the value of IPv4IpOptionsSupported
    pub fn set_ipv4_ip_options_supported(&mut self, value: bool) {
        self.ipv4_ip_options_supported = Some(value);
    }

    /// Gets the value of IPv4IpOptionsSupported
    pub fn get_ipv4_ip_options_supported(&self) -> Option<&bool> {
        self.ipv4_ip_options_supported.as_ref()
    }

    /// Sets the value of IPv4MaxOffloadSizeSupported
    pub fn set_ipv4_max_offload_size_supported(&mut self, value: u32) {
        self.ipv4_max_offload_size_supported = Some(value);
    }

    /// Gets the value of IPv4MaxOffloadSizeSupported
    pub fn get_ipv4_max_offload_size_supported(&self) -> Option<&u32> {
        self.ipv4_max_offload_size_supported.as_ref()
    }

    /// Sets the value of IPv4MinSegmentCountSupported
    pub fn set_ipv4_min_segment_count_supported(&mut self, value: u32) {
        self.ipv4_min_segment_count_supported = Some(value);
    }

    /// Gets the value of IPv4MinSegmentCountSupported
    pub fn get_ipv4_min_segment_count_supported(&self) -> Option<&u32> {
        self.ipv4_min_segment_count_supported.as_ref()
    }

    /// Sets the value of IPv4TcpOptionsSupported
    pub fn set_ipv4_tcp_options_supported(&mut self, value: bool) {
        self.ipv4_tcp_options_supported = Some(value);
    }

    /// Gets the value of IPv4TcpOptionsSupported
    pub fn get_ipv4_tcp_options_supported(&self) -> Option<&bool> {
        self.ipv4_tcp_options_supported.as_ref()
    }
}

