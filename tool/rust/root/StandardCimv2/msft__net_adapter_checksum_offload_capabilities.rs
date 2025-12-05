// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterChecksumOffloadCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterChecksumOffloadCapabilities {

/// 
    #[serde(rename = "IPv4ReceiveEncapsulation")]
    pub ipv4_receive_encapsulation: Option<MSFT_NetAdapterChecksumOffloadEncapsulationTypes>,

/// 
    #[serde(rename = "IPv4ReceiveIpChecksumSupported")]
    pub ipv4_receive_ip_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4ReceiveIpOptionsSupported")]
    pub ipv4_receive_ip_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4ReceiveTcpChecksumSupported")]
    pub ipv4_receive_tcp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4ReceiveTcpOptionsSupported")]
    pub ipv4_receive_tcp_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4ReceiveUdpChecksumSupported")]
    pub ipv4_receive_udp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4TransmitEncapsulation")]
    pub ipv4_transmit_encapsulation: Option<MSFT_NetAdapterChecksumOffloadEncapsulationTypes>,

/// 
    #[serde(rename = "IPv4TransmitIpChecksumSupported")]
    pub ipv4_transmit_ip_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4TransmitIpOptionsSupported")]
    pub ipv4_transmit_ip_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4TransmitTcpChecksumSupported")]
    pub ipv4_transmit_tcp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4TransmitTcpOptionsSupported")]
    pub ipv4_transmit_tcp_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4TransmitUdpChecksumSupported")]
    pub ipv4_transmit_udp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6ReceiveEncapsulation")]
    pub ipv6_receive_encapsulation: Option<MSFT_NetAdapterChecksumOffloadEncapsulationTypes>,

/// 
    #[serde(rename = "IPv6ReceiveIpExtensionHeadersSupported")]
    pub ipv6_receive_ip_extension_headers_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6ReceiveTcpChecksumSupported")]
    pub ipv6_receive_tcp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6ReceiveTcpOptionsSupported")]
    pub ipv6_receive_tcp_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6ReceiveUdpChecksumSupported")]
    pub ipv6_receive_udp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6TransmitEncapsulation")]
    pub ipv6_transmit_encapsulation: Option<MSFT_NetAdapterChecksumOffloadEncapsulationTypes>,

/// 
    #[serde(rename = "IPv6TransmitIpExtensionHeadersSupported")]
    pub ipv6_transmit_ip_extension_headers_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6TransmitTcpChecksumSupported")]
    pub ipv6_transmit_tcp_checksum_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6TransmitTcpOptionsSupported")]
    pub ipv6_transmit_tcp_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6TransmitUdpChecksumSupported")]
    pub ipv6_transmit_udp_checksum_supported: Option<bool>,
}

impl MSFT_NetAdapterChecksumOffloadCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ipv4_receive_encapsulation: None,
            ipv4_receive_ip_checksum_supported: None,
            ipv4_receive_ip_options_supported: None,
            ipv4_receive_tcp_checksum_supported: None,
            ipv4_receive_tcp_options_supported: None,
            ipv4_receive_udp_checksum_supported: None,
            ipv4_transmit_encapsulation: None,
            ipv4_transmit_ip_checksum_supported: None,
            ipv4_transmit_ip_options_supported: None,
            ipv4_transmit_tcp_checksum_supported: None,
            ipv4_transmit_tcp_options_supported: None,
            ipv4_transmit_udp_checksum_supported: None,
            ipv6_receive_encapsulation: None,
            ipv6_receive_ip_extension_headers_supported: None,
            ipv6_receive_tcp_checksum_supported: None,
            ipv6_receive_tcp_options_supported: None,
            ipv6_receive_udp_checksum_supported: None,
            ipv6_transmit_encapsulation: None,
            ipv6_transmit_ip_extension_headers_supported: None,
            ipv6_transmit_tcp_checksum_supported: None,
            ipv6_transmit_tcp_options_supported: None,
            ipv6_transmit_udp_checksum_supported: None,
        }
    }


    /// Sets the value of IPv4ReceiveEncapsulation
    pub fn set_ipv4_receive_encapsulation(&mut self, value: MSFT_NetAdapterChecksumOffloadEncapsulationTypes) {
        self.ipv4_receive_encapsulation = Some(value);
    }

    /// Gets the value of IPv4ReceiveEncapsulation
    pub fn get_ipv4_receive_encapsulation(&self) -> Option<&MSFT_NetAdapterChecksumOffloadEncapsulationTypes> {
        self.ipv4_receive_encapsulation.as_ref()
    }

    /// Sets the value of IPv4ReceiveIpChecksumSupported
    pub fn set_ipv4_receive_ip_checksum_supported(&mut self, value: bool) {
        self.ipv4_receive_ip_checksum_supported = Some(value);
    }

    /// Gets the value of IPv4ReceiveIpChecksumSupported
    pub fn get_ipv4_receive_ip_checksum_supported(&self) -> Option<&bool> {
        self.ipv4_receive_ip_checksum_supported.as_ref()
    }

    /// Sets the value of IPv4ReceiveIpOptionsSupported
    pub fn set_ipv4_receive_ip_options_supported(&mut self, value: bool) {
        self.ipv4_receive_ip_options_supported = Some(value);
    }

    /// Gets the value of IPv4ReceiveIpOptionsSupported
    pub fn get_ipv4_receive_ip_options_supported(&self) -> Option<&bool> {
        self.ipv4_receive_ip_options_supported.as_ref()
    }

    /// Sets the value of IPv4ReceiveTcpChecksumSupported
    pub fn set_ipv4_receive_tcp_checksum_supported(&mut self, value: bool) {
        self.ipv4_receive_tcp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv4ReceiveTcpChecksumSupported
    pub fn get_ipv4_receive_tcp_checksum_supported(&self) -> Option<&bool> {
        self.ipv4_receive_tcp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv4ReceiveTcpOptionsSupported
    pub fn set_ipv4_receive_tcp_options_supported(&mut self, value: bool) {
        self.ipv4_receive_tcp_options_supported = Some(value);
    }

    /// Gets the value of IPv4ReceiveTcpOptionsSupported
    pub fn get_ipv4_receive_tcp_options_supported(&self) -> Option<&bool> {
        self.ipv4_receive_tcp_options_supported.as_ref()
    }

    /// Sets the value of IPv4ReceiveUdpChecksumSupported
    pub fn set_ipv4_receive_udp_checksum_supported(&mut self, value: bool) {
        self.ipv4_receive_udp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv4ReceiveUdpChecksumSupported
    pub fn get_ipv4_receive_udp_checksum_supported(&self) -> Option<&bool> {
        self.ipv4_receive_udp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv4TransmitEncapsulation
    pub fn set_ipv4_transmit_encapsulation(&mut self, value: MSFT_NetAdapterChecksumOffloadEncapsulationTypes) {
        self.ipv4_transmit_encapsulation = Some(value);
    }

    /// Gets the value of IPv4TransmitEncapsulation
    pub fn get_ipv4_transmit_encapsulation(&self) -> Option<&MSFT_NetAdapterChecksumOffloadEncapsulationTypes> {
        self.ipv4_transmit_encapsulation.as_ref()
    }

    /// Sets the value of IPv4TransmitIpChecksumSupported
    pub fn set_ipv4_transmit_ip_checksum_supported(&mut self, value: bool) {
        self.ipv4_transmit_ip_checksum_supported = Some(value);
    }

    /// Gets the value of IPv4TransmitIpChecksumSupported
    pub fn get_ipv4_transmit_ip_checksum_supported(&self) -> Option<&bool> {
        self.ipv4_transmit_ip_checksum_supported.as_ref()
    }

    /// Sets the value of IPv4TransmitIpOptionsSupported
    pub fn set_ipv4_transmit_ip_options_supported(&mut self, value: bool) {
        self.ipv4_transmit_ip_options_supported = Some(value);
    }

    /// Gets the value of IPv4TransmitIpOptionsSupported
    pub fn get_ipv4_transmit_ip_options_supported(&self) -> Option<&bool> {
        self.ipv4_transmit_ip_options_supported.as_ref()
    }

    /// Sets the value of IPv4TransmitTcpChecksumSupported
    pub fn set_ipv4_transmit_tcp_checksum_supported(&mut self, value: bool) {
        self.ipv4_transmit_tcp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv4TransmitTcpChecksumSupported
    pub fn get_ipv4_transmit_tcp_checksum_supported(&self) -> Option<&bool> {
        self.ipv4_transmit_tcp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv4TransmitTcpOptionsSupported
    pub fn set_ipv4_transmit_tcp_options_supported(&mut self, value: bool) {
        self.ipv4_transmit_tcp_options_supported = Some(value);
    }

    /// Gets the value of IPv4TransmitTcpOptionsSupported
    pub fn get_ipv4_transmit_tcp_options_supported(&self) -> Option<&bool> {
        self.ipv4_transmit_tcp_options_supported.as_ref()
    }

    /// Sets the value of IPv4TransmitUdpChecksumSupported
    pub fn set_ipv4_transmit_udp_checksum_supported(&mut self, value: bool) {
        self.ipv4_transmit_udp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv4TransmitUdpChecksumSupported
    pub fn get_ipv4_transmit_udp_checksum_supported(&self) -> Option<&bool> {
        self.ipv4_transmit_udp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv6ReceiveEncapsulation
    pub fn set_ipv6_receive_encapsulation(&mut self, value: MSFT_NetAdapterChecksumOffloadEncapsulationTypes) {
        self.ipv6_receive_encapsulation = Some(value);
    }

    /// Gets the value of IPv6ReceiveEncapsulation
    pub fn get_ipv6_receive_encapsulation(&self) -> Option<&MSFT_NetAdapterChecksumOffloadEncapsulationTypes> {
        self.ipv6_receive_encapsulation.as_ref()
    }

    /// Sets the value of IPv6ReceiveIpExtensionHeadersSupported
    pub fn set_ipv6_receive_ip_extension_headers_supported(&mut self, value: bool) {
        self.ipv6_receive_ip_extension_headers_supported = Some(value);
    }

    /// Gets the value of IPv6ReceiveIpExtensionHeadersSupported
    pub fn get_ipv6_receive_ip_extension_headers_supported(&self) -> Option<&bool> {
        self.ipv6_receive_ip_extension_headers_supported.as_ref()
    }

    /// Sets the value of IPv6ReceiveTcpChecksumSupported
    pub fn set_ipv6_receive_tcp_checksum_supported(&mut self, value: bool) {
        self.ipv6_receive_tcp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv6ReceiveTcpChecksumSupported
    pub fn get_ipv6_receive_tcp_checksum_supported(&self) -> Option<&bool> {
        self.ipv6_receive_tcp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv6ReceiveTcpOptionsSupported
    pub fn set_ipv6_receive_tcp_options_supported(&mut self, value: bool) {
        self.ipv6_receive_tcp_options_supported = Some(value);
    }

    /// Gets the value of IPv6ReceiveTcpOptionsSupported
    pub fn get_ipv6_receive_tcp_options_supported(&self) -> Option<&bool> {
        self.ipv6_receive_tcp_options_supported.as_ref()
    }

    /// Sets the value of IPv6ReceiveUdpChecksumSupported
    pub fn set_ipv6_receive_udp_checksum_supported(&mut self, value: bool) {
        self.ipv6_receive_udp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv6ReceiveUdpChecksumSupported
    pub fn get_ipv6_receive_udp_checksum_supported(&self) -> Option<&bool> {
        self.ipv6_receive_udp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv6TransmitEncapsulation
    pub fn set_ipv6_transmit_encapsulation(&mut self, value: MSFT_NetAdapterChecksumOffloadEncapsulationTypes) {
        self.ipv6_transmit_encapsulation = Some(value);
    }

    /// Gets the value of IPv6TransmitEncapsulation
    pub fn get_ipv6_transmit_encapsulation(&self) -> Option<&MSFT_NetAdapterChecksumOffloadEncapsulationTypes> {
        self.ipv6_transmit_encapsulation.as_ref()
    }

    /// Sets the value of IPv6TransmitIpExtensionHeadersSupported
    pub fn set_ipv6_transmit_ip_extension_headers_supported(&mut self, value: bool) {
        self.ipv6_transmit_ip_extension_headers_supported = Some(value);
    }

    /// Gets the value of IPv6TransmitIpExtensionHeadersSupported
    pub fn get_ipv6_transmit_ip_extension_headers_supported(&self) -> Option<&bool> {
        self.ipv6_transmit_ip_extension_headers_supported.as_ref()
    }

    /// Sets the value of IPv6TransmitTcpChecksumSupported
    pub fn set_ipv6_transmit_tcp_checksum_supported(&mut self, value: bool) {
        self.ipv6_transmit_tcp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv6TransmitTcpChecksumSupported
    pub fn get_ipv6_transmit_tcp_checksum_supported(&self) -> Option<&bool> {
        self.ipv6_transmit_tcp_checksum_supported.as_ref()
    }

    /// Sets the value of IPv6TransmitTcpOptionsSupported
    pub fn set_ipv6_transmit_tcp_options_supported(&mut self, value: bool) {
        self.ipv6_transmit_tcp_options_supported = Some(value);
    }

    /// Gets the value of IPv6TransmitTcpOptionsSupported
    pub fn get_ipv6_transmit_tcp_options_supported(&self) -> Option<&bool> {
        self.ipv6_transmit_tcp_options_supported.as_ref()
    }

    /// Sets the value of IPv6TransmitUdpChecksumSupported
    pub fn set_ipv6_transmit_udp_checksum_supported(&mut self, value: bool) {
        self.ipv6_transmit_udp_checksum_supported = Some(value);
    }

    /// Gets the value of IPv6TransmitUdpChecksumSupported
    pub fn get_ipv6_transmit_udp_checksum_supported(&self) -> Option<&bool> {
        self.ipv6_transmit_udp_checksum_supported.as_ref()
    }
}

