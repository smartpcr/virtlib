// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Encapsulation")]
    pub encapsulation: Option<u32>,

/// 
    #[serde(rename = "IpExtensionHeadersSupported")]
    pub ip_extension_headers_supported: Option<u32>,

/// 
    #[serde(rename = "TcpChecksum")]
    pub tcp_checksum: Option<u32>,

/// 
    #[serde(rename = "TcpOptionsSupported")]
    pub tcp_options_supported: Option<u32>,

/// 
    #[serde(rename = "UdpChecksum")]
    pub udp_checksum: Option<u32>,
}

impl MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            encapsulation: None,
            ip_extension_headers_supported: None,
            tcp_checksum: None,
            tcp_options_supported: None,
            udp_checksum: None,
        }
    }


    /// Sets the value of Encapsulation
    pub fn set_encapsulation(&mut self, value: u32) {
        self.encapsulation = Some(value);
    }

    /// Gets the value of Encapsulation
    pub fn get_encapsulation(&self) -> Option<&u32> {
        self.encapsulation.as_ref()
    }

    /// Sets the value of IpExtensionHeadersSupported
    pub fn set_ip_extension_headers_supported(&mut self, value: u32) {
        self.ip_extension_headers_supported = Some(value);
    }

    /// Gets the value of IpExtensionHeadersSupported
    pub fn get_ip_extension_headers_supported(&self) -> Option<&u32> {
        self.ip_extension_headers_supported.as_ref()
    }

    /// Sets the value of TcpChecksum
    pub fn set_tcp_checksum(&mut self, value: u32) {
        self.tcp_checksum = Some(value);
    }

    /// Gets the value of TcpChecksum
    pub fn get_tcp_checksum(&self) -> Option<&u32> {
        self.tcp_checksum.as_ref()
    }

    /// Sets the value of TcpOptionsSupported
    pub fn set_tcp_options_supported(&mut self, value: u32) {
        self.tcp_options_supported = Some(value);
    }

    /// Gets the value of TcpOptionsSupported
    pub fn get_tcp_options_supported(&self) -> Option<&u32> {
        self.tcp_options_supported.as_ref()
    }

    /// Sets the value of UdpChecksum
    pub fn set_udp_checksum(&mut self, value: u32) {
        self.udp_checksum = Some(value);
    }

    /// Gets the value of UdpChecksum
    pub fn get_udp_checksum(&self) -> Option<&u32> {
        self.udp_checksum.as_ref()
    }
}

