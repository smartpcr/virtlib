// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NetworkPort_LinkTechnology
//////////////////////////////////////////////

/// NetworkPort_LinkTechnology enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NetworkPort_LinkTechnology {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Ethernet
    #[serde(rename = "Ethernet")]
    Ethernet = 2,
    /// IB
    #[serde(rename = "IB")]
    IB = 3,
    /// FC
    #[serde(rename = "FC")]
    FC = 4,
    /// FDDI
    #[serde(rename = "FDDI")]
    FDDI = 5,
    /// ATM
    #[serde(rename = "ATM")]
    ATM = 6,
    /// Token_Ring
    #[serde(rename = "Token_Ring")]
    TokenRing = 7,
    /// Frame_Relay
    #[serde(rename = "Frame_Relay")]
    FrameRelay = 8,
    /// Infrared
    #[serde(rename = "Infrared")]
    Infrared = 9,
    /// BlueTooth
    #[serde(rename = "BlueTooth")]
    BlueTooth = 10,
    /// Wireless_LAN
    #[serde(rename = "Wireless_LAN")]
    WirelessLAN = 11,
}

impl Default for NetworkPort_LinkTechnology {
    fn default() -> Self {
        Self::Unknown
    }
}

