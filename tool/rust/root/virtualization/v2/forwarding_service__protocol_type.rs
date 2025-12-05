// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ForwardingService_ProtocolType
//////////////////////////////////////////////

/// ForwardingService_ProtocolType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ForwardingService_ProtocolType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// IPv4
    #[serde(rename = "IPv4")]
    IPv4 = 2,
    /// IPv6
    #[serde(rename = "IPv6")]
    IPv6 = 3,
    /// IPv4_IPv6
    #[serde(rename = "IPv4_IPv6")]
    IPv4IPv6 = 4,
    /// IPX
    #[serde(rename = "IPX")]
    IPX = 5,
    /// AppleTalk
    #[serde(rename = "AppleTalk")]
    AppleTalk = 6,
    /// DECnet
    #[serde(rename = "DECnet")]
    DECnet = 7,
    /// SNA
    #[serde(rename = "SNA")]
    SNA = 8,
    /// CONP
    #[serde(rename = "CONP")]
    CONP = 9,
    /// CLNP
    #[serde(rename = "CLNP")]
    CLNP = 10,
    /// VINES
    #[serde(rename = "VINES")]
    VINES = 11,
    /// XNS
    #[serde(rename = "XNS")]
    XNS = 12,
    /// ATM
    #[serde(rename = "ATM")]
    ATM = 13,
    /// Frame_Relay
    #[serde(rename = "Frame_Relay")]
    FrameRelay = 14,
    /// Ethernet
    #[serde(rename = "Ethernet")]
    Ethernet = 15,
    /// TokenRing
    #[serde(rename = "TokenRing")]
    TokenRing = 16,
    /// FDDI
    #[serde(rename = "FDDI")]
    FDDI = 17,
    /// Infiniband
    #[serde(rename = "Infiniband")]
    Infiniband = 18,
    /// Fibre_Channel
    #[serde(rename = "Fibre_Channel")]
    FibreChannel = 19,
}

impl Default for ForwardingService_ProtocolType {
    fn default() -> Self {
        Self::Unknown
    }
}

