// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_TcpOffloadParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_TcpOffloadParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "IPsec")]
    pub ipsec: Option<u8>,

/// 
    #[serde(rename = "IPv4Checksum")]
    pub ipv4_checksum: Option<u8>,

/// 
    #[serde(rename = "LsoV1")]
    pub lso_v1: Option<u8>,

/// 
    #[serde(rename = "LsoV2IPv4")]
    pub lso_v2_ipv4: Option<u8>,

/// 
    #[serde(rename = "LsoV2IPv6")]
    pub lso_v2_ipv6: Option<u8>,

/// 
    #[serde(rename = "TcpConnectionIPv4")]
    pub tcp_connection_ipv4: Option<u8>,

/// 
    #[serde(rename = "TcpConnectionIPv6")]
    pub tcp_connection_ipv6: Option<u8>,

/// 
    #[serde(rename = "TCPIPv4Checksum")]
    pub tcpipv4_checksum: Option<u8>,

/// 
    #[serde(rename = "TCPIPv6Checksum")]
    pub tcpipv6_checksum: Option<u8>,

/// 
    #[serde(rename = "UDPIPv4Checksum")]
    pub udpipv4_checksum: Option<u8>,

/// 
    #[serde(rename = "UDPIPv6Checksum")]
    pub udpipv6_checksum: Option<u8>,
}

impl MSNdis_TcpOffloadParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            flags: None,
            header: None,
            ipsec: None,
            ipv4_checksum: None,
            lso_v1: None,
            lso_v2_ipv4: None,
            lso_v2_ipv6: None,
            tcp_connection_ipv4: None,
            tcp_connection_ipv6: None,
            tcpipv4_checksum: None,
            tcpipv6_checksum: None,
            udpipv4_checksum: None,
            udpipv6_checksum: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of IPsec
    pub fn set_ipsec(&mut self, value: u8) {
        self.ipsec = Some(value);
    }

    /// Gets the value of IPsec
    pub fn get_ipsec(&self) -> Option<&u8> {
        self.ipsec.as_ref()
    }

    /// Sets the value of IPv4Checksum
    pub fn set_ipv4_checksum(&mut self, value: u8) {
        self.ipv4_checksum = Some(value);
    }

    /// Gets the value of IPv4Checksum
    pub fn get_ipv4_checksum(&self) -> Option<&u8> {
        self.ipv4_checksum.as_ref()
    }

    /// Sets the value of LsoV1
    pub fn set_lso_v1(&mut self, value: u8) {
        self.lso_v1 = Some(value);
    }

    /// Gets the value of LsoV1
    pub fn get_lso_v1(&self) -> Option<&u8> {
        self.lso_v1.as_ref()
    }

    /// Sets the value of LsoV2IPv4
    pub fn set_lso_v2_ipv4(&mut self, value: u8) {
        self.lso_v2_ipv4 = Some(value);
    }

    /// Gets the value of LsoV2IPv4
    pub fn get_lso_v2_ipv4(&self) -> Option<&u8> {
        self.lso_v2_ipv4.as_ref()
    }

    /// Sets the value of LsoV2IPv6
    pub fn set_lso_v2_ipv6(&mut self, value: u8) {
        self.lso_v2_ipv6 = Some(value);
    }

    /// Gets the value of LsoV2IPv6
    pub fn get_lso_v2_ipv6(&self) -> Option<&u8> {
        self.lso_v2_ipv6.as_ref()
    }

    /// Sets the value of TcpConnectionIPv4
    pub fn set_tcp_connection_ipv4(&mut self, value: u8) {
        self.tcp_connection_ipv4 = Some(value);
    }

    /// Gets the value of TcpConnectionIPv4
    pub fn get_tcp_connection_ipv4(&self) -> Option<&u8> {
        self.tcp_connection_ipv4.as_ref()
    }

    /// Sets the value of TcpConnectionIPv6
    pub fn set_tcp_connection_ipv6(&mut self, value: u8) {
        self.tcp_connection_ipv6 = Some(value);
    }

    /// Gets the value of TcpConnectionIPv6
    pub fn get_tcp_connection_ipv6(&self) -> Option<&u8> {
        self.tcp_connection_ipv6.as_ref()
    }

    /// Sets the value of TCPIPv4Checksum
    pub fn set_tcpipv4_checksum(&mut self, value: u8) {
        self.tcpipv4_checksum = Some(value);
    }

    /// Gets the value of TCPIPv4Checksum
    pub fn get_tcpipv4_checksum(&self) -> Option<&u8> {
        self.tcpipv4_checksum.as_ref()
    }

    /// Sets the value of TCPIPv6Checksum
    pub fn set_tcpipv6_checksum(&mut self, value: u8) {
        self.tcpipv6_checksum = Some(value);
    }

    /// Gets the value of TCPIPv6Checksum
    pub fn get_tcpipv6_checksum(&self) -> Option<&u8> {
        self.tcpipv6_checksum.as_ref()
    }

    /// Sets the value of UDPIPv4Checksum
    pub fn set_udpipv4_checksum(&mut self, value: u8) {
        self.udpipv4_checksum = Some(value);
    }

    /// Gets the value of UDPIPv4Checksum
    pub fn get_udpipv4_checksum(&self) -> Option<&u8> {
        self.udpipv4_checksum.as_ref()
    }

    /// Sets the value of UDPIPv6Checksum
    pub fn set_udpipv6_checksum(&mut self, value: u8) {
        self.udpipv6_checksum = Some(value);
    }

    /// Gets the value of UDPIPv6Checksum
    pub fn get_udpipv6_checksum(&self) -> Option<&u8> {
        self.udpipv6_checksum.as_ref()
    }
}

