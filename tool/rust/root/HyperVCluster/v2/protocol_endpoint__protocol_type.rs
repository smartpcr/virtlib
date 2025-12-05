// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProtocolEndpoint_ProtocolType
//////////////////////////////////////////////

/// ProtocolEndpoint_ProtocolType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProtocolEndpoint_ProtocolType {
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
    /// IPX
    #[serde(rename = "IPX")]
    IPX = 4,
    /// AppleTalk
    #[serde(rename = "AppleTalk")]
    AppleTalk = 5,
    /// DECnet
    #[serde(rename = "DECnet")]
    DECnet = 6,
    /// SNA
    #[serde(rename = "SNA")]
    SNA = 7,
    /// CONP
    #[serde(rename = "CONP")]
    CONP = 8,
    /// CLNP
    #[serde(rename = "CLNP")]
    CLNP = 9,
    /// VINES
    #[serde(rename = "VINES")]
    VINES = 10,
    /// XNS
    #[serde(rename = "XNS")]
    XNS = 11,
    /// ATM
    #[serde(rename = "ATM")]
    ATM = 12,
    /// Frame_Relay
    #[serde(rename = "Frame_Relay")]
    FrameRelay = 13,
    /// Ethernet
    #[serde(rename = "Ethernet")]
    Ethernet = 14,
    /// TokenRing
    #[serde(rename = "TokenRing")]
    TokenRing = 15,
    /// FDDI
    #[serde(rename = "FDDI")]
    FDDI = 16,
    /// Infiniband
    #[serde(rename = "Infiniband")]
    Infiniband = 17,
    /// Fibre_Channel
    #[serde(rename = "Fibre_Channel")]
    FibreChannel = 18,
    /// ISDN_BRI_Endpoint
    #[serde(rename = "ISDN_BRI_Endpoint")]
    ISDNBRIEndpoint = 19,
    /// ISDN_B_Channel_Endpoint
    #[serde(rename = "ISDN_B_Channel_Endpoint")]
    ISDNBChannelEndpoint = 20,
    /// ISDN_D_Channel_Endpoint
    #[serde(rename = "ISDN_D_Channel_Endpoint")]
    ISDNDChannelEndpoint = 21,
    /// IPv4_v6
    #[serde(rename = "IPv4_v6")]
    IPv4V6 = 22,
    /// BGP
    #[serde(rename = "BGP")]
    BGP = 23,
    /// OSPF
    #[serde(rename = "OSPF")]
    OSPF = 24,
    /// MPLS
    #[serde(rename = "MPLS")]
    MPLS = 25,
    /// UDP
    #[serde(rename = "UDP")]
    UDP = 26,
    /// TCP
    #[serde(rename = "TCP")]
    TCP = 27,
}

impl Default for ProtocolEndpoint_ProtocolType {
    fn default() -> Self {
        Self::Unknown
    }
}

