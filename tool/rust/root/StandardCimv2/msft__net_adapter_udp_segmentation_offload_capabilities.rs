// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterUdpSegmentationOffloadCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterUdpSegmentationOffloadCapabilities {

/// 
    #[serde(rename = "IPv4Encapsulation")]
    pub ipv4_encapsulation: Option<MSFT_NetAdapterUsoEncapsulationTypes>,

/// 
    #[serde(rename = "IPv4MaxOffloadSizeSupported")]
    pub ipv4_max_offload_size_supported: Option<u32>,

/// 
    #[serde(rename = "IPv4MinSegmentCountSupported")]
    pub ipv4_min_segment_count_supported: Option<u32>,

/// 
    #[serde(rename = "IPv4SubMssFinalSegmentSupported")]
    pub ipv4_sub_mss_final_segment_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6Encapsulation")]
    pub ipv6_encapsulation: Option<MSFT_NetAdapterUsoEncapsulationTypes>,

/// 
    #[serde(rename = "IPv6IpExtensionHeadersSupported")]
    pub ipv6_ip_extension_headers_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6MaxOffLoadSizeSupported")]
    pub ipv6_max_off_load_size_supported: Option<u32>,

/// 
    #[serde(rename = "IPv6MinSegmentCountSupported")]
    pub ipv6_min_segment_count_supported: Option<u32>,

/// 
    #[serde(rename = "IPv6SubMssFinalSegmentSupported")]
    pub ipv6_sub_mss_final_segment_supported: Option<bool>,
}

impl MSFT_NetAdapterUdpSegmentationOffloadCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ipv4_encapsulation: None,
            ipv4_max_offload_size_supported: None,
            ipv4_min_segment_count_supported: None,
            ipv4_sub_mss_final_segment_supported: None,
            ipv6_encapsulation: None,
            ipv6_ip_extension_headers_supported: None,
            ipv6_max_off_load_size_supported: None,
            ipv6_min_segment_count_supported: None,
            ipv6_sub_mss_final_segment_supported: None,
        }
    }


    /// Sets the value of IPv4Encapsulation
    pub fn set_ipv4_encapsulation(&mut self, value: MSFT_NetAdapterUsoEncapsulationTypes) {
        self.ipv4_encapsulation = Some(value);
    }

    /// Gets the value of IPv4Encapsulation
    pub fn get_ipv4_encapsulation(&self) -> Option<&MSFT_NetAdapterUsoEncapsulationTypes> {
        self.ipv4_encapsulation.as_ref()
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

    /// Sets the value of IPv4SubMssFinalSegmentSupported
    pub fn set_ipv4_sub_mss_final_segment_supported(&mut self, value: bool) {
        self.ipv4_sub_mss_final_segment_supported = Some(value);
    }

    /// Gets the value of IPv4SubMssFinalSegmentSupported
    pub fn get_ipv4_sub_mss_final_segment_supported(&self) -> Option<&bool> {
        self.ipv4_sub_mss_final_segment_supported.as_ref()
    }

    /// Sets the value of IPv6Encapsulation
    pub fn set_ipv6_encapsulation(&mut self, value: MSFT_NetAdapterUsoEncapsulationTypes) {
        self.ipv6_encapsulation = Some(value);
    }

    /// Gets the value of IPv6Encapsulation
    pub fn get_ipv6_encapsulation(&self) -> Option<&MSFT_NetAdapterUsoEncapsulationTypes> {
        self.ipv6_encapsulation.as_ref()
    }

    /// Sets the value of IPv6IpExtensionHeadersSupported
    pub fn set_ipv6_ip_extension_headers_supported(&mut self, value: bool) {
        self.ipv6_ip_extension_headers_supported = Some(value);
    }

    /// Gets the value of IPv6IpExtensionHeadersSupported
    pub fn get_ipv6_ip_extension_headers_supported(&self) -> Option<&bool> {
        self.ipv6_ip_extension_headers_supported.as_ref()
    }

    /// Sets the value of IPv6MaxOffLoadSizeSupported
    pub fn set_ipv6_max_off_load_size_supported(&mut self, value: u32) {
        self.ipv6_max_off_load_size_supported = Some(value);
    }

    /// Gets the value of IPv6MaxOffLoadSizeSupported
    pub fn get_ipv6_max_off_load_size_supported(&self) -> Option<&u32> {
        self.ipv6_max_off_load_size_supported.as_ref()
    }

    /// Sets the value of IPv6MinSegmentCountSupported
    pub fn set_ipv6_min_segment_count_supported(&mut self, value: u32) {
        self.ipv6_min_segment_count_supported = Some(value);
    }

    /// Gets the value of IPv6MinSegmentCountSupported
    pub fn get_ipv6_min_segment_count_supported(&self) -> Option<&u32> {
        self.ipv6_min_segment_count_supported.as_ref()
    }

    /// Sets the value of IPv6SubMssFinalSegmentSupported
    pub fn set_ipv6_sub_mss_final_segment_supported(&mut self, value: bool) {
        self.ipv6_sub_mss_final_segment_supported = Some(value);
    }

    /// Gets the value of IPv6SubMssFinalSegmentSupported
    pub fn get_ipv6_sub_mss_final_segment_supported(&self) -> Option<&bool> {
        self.ipv6_sub_mss_final_segment_supported.as_ref()
    }
}

