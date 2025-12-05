// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiTcpIpChecksumOffload struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiTcpIpChecksumOffload {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "IPv4Receive")]
    pub ipv4_receive: Option<MSNdis_WmiTcpIpChecksumOffload_IPv4TransmitReceive>,

/// 
    #[serde(rename = "IPv4Transmit")]
    pub ipv4_transmit: Option<MSNdis_WmiTcpIpChecksumOffload_IPv4TransmitReceive>,

/// 
    #[serde(rename = "IPv6Receive")]
    pub ipv6_receive: Option<MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive>,

/// 
    #[serde(rename = "IPv6Transmit")]
    pub ipv6_transmit: Option<MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive>,
}

impl MSNdis_WmiTcpIpChecksumOffload {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ipv4_receive: None,
            ipv4_transmit: None,
            ipv6_receive: None,
            ipv6_transmit: None,
        }
    }


    /// Sets the value of IPv4Receive
    pub fn set_ipv4_receive(&mut self, value: MSNdis_WmiTcpIpChecksumOffload_IPv4TransmitReceive) {
        self.ipv4_receive = Some(value);
    }

    /// Gets the value of IPv4Receive
    pub fn get_ipv4_receive(&self) -> Option<&MSNdis_WmiTcpIpChecksumOffload_IPv4TransmitReceive> {
        self.ipv4_receive.as_ref()
    }

    /// Sets the value of IPv4Transmit
    pub fn set_ipv4_transmit(&mut self, value: MSNdis_WmiTcpIpChecksumOffload_IPv4TransmitReceive) {
        self.ipv4_transmit = Some(value);
    }

    /// Gets the value of IPv4Transmit
    pub fn get_ipv4_transmit(&self) -> Option<&MSNdis_WmiTcpIpChecksumOffload_IPv4TransmitReceive> {
        self.ipv4_transmit.as_ref()
    }

    /// Sets the value of IPv6Receive
    pub fn set_ipv6_receive(&mut self, value: MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive) {
        self.ipv6_receive = Some(value);
    }

    /// Gets the value of IPv6Receive
    pub fn get_ipv6_receive(&self) -> Option<&MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive> {
        self.ipv6_receive.as_ref()
    }

    /// Sets the value of IPv6Transmit
    pub fn set_ipv6_transmit(&mut self, value: MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive) {
        self.ipv6_transmit = Some(value);
    }

    /// Gets the value of IPv6Transmit
    pub fn get_ipv6_transmit(&self) -> Option<&MSNdis_WmiTcpIpChecksumOffload_IPv6TransmitReceive> {
        self.ipv6_transmit.as_ref()
    }
}

