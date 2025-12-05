// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Processor_UpgradeMethod
//////////////////////////////////////////////

/// Processor_UpgradeMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Processor_UpgradeMethod {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Daughter_Board
    #[serde(rename = "Daughter_Board")]
    DaughterBoard = 3,
    /// ZIF_Socket
    #[serde(rename = "ZIF_Socket")]
    ZIFSocket = 4,
    /// Replacement_Piggy_Back
    #[serde(rename = "Replacement_Piggy_Back")]
    ReplacementPiggyBack = 5,
    /// None
    #[serde(rename = "None")]
    None = 6,
    /// LIF_Socket
    #[serde(rename = "LIF_Socket")]
    LIFSocket = 7,
    /// Slot_1
    #[serde(rename = "Slot_1")]
    Slot1 = 8,
    /// Slot_2
    #[serde(rename = "Slot_2")]
    Slot2 = 9,
    /// _370_Pin_Socket
    #[serde(rename = "_370_Pin_Socket")]
    V370PinSocket = 10,
    /// Slot_A
    #[serde(rename = "Slot_A")]
    SlotA = 11,
    /// Slot_M
    #[serde(rename = "Slot_M")]
    SlotM = 12,
    /// Socket_423
    #[serde(rename = "Socket_423")]
    Socket423 = 13,
    /// Socket_A__Socket_462_
    #[serde(rename = "Socket_A__Socket_462_")]
    SocketASocket462 = 14,
    /// Socket_478
    #[serde(rename = "Socket_478")]
    Socket478 = 15,
    /// Socket_754
    #[serde(rename = "Socket_754")]
    Socket754 = 16,
    /// Socket_940
    #[serde(rename = "Socket_940")]
    Socket940 = 17,
    /// Socket_939
    #[serde(rename = "Socket_939")]
    Socket939 = 18,
    /// Socket_mPGA604
    #[serde(rename = "Socket_mPGA604")]
    SocketMPGA604 = 19,
    /// Socket_LGA771
    #[serde(rename = "Socket_LGA771")]
    SocketLGA771 = 20,
    /// Socket_LGA775
    #[serde(rename = "Socket_LGA775")]
    SocketLGA775 = 21,
    /// Socket_S1
    #[serde(rename = "Socket_S1")]
    SocketS1 = 22,
    /// Socket_AM2
    #[serde(rename = "Socket_AM2")]
    SocketAM2 = 23,
    /// Socket_F__1207_
    #[serde(rename = "Socket_F__1207_")]
    SocketF1207 = 24,
    /// Socket_LGA1366
    #[serde(rename = "Socket_LGA1366")]
    SocketLGA1366 = 25,
    /// Socket_G34
    #[serde(rename = "Socket_G34")]
    SocketG34 = 26,
    /// Socket_AM3
    #[serde(rename = "Socket_AM3")]
    SocketAM3 = 27,
    /// Socket_C32
    #[serde(rename = "Socket_C32")]
    SocketC32 = 28,
    /// Socket_LGA1156
    #[serde(rename = "Socket_LGA1156")]
    SocketLGA1156 = 29,
    /// Socket_LGA1567
    #[serde(rename = "Socket_LGA1567")]
    SocketLGA1567 = 30,
    /// Socket_PGA988A
    #[serde(rename = "Socket_PGA988A")]
    SocketPGA988A = 31,
    /// Socket_BGA1288
    #[serde(rename = "Socket_BGA1288")]
    SocketBGA1288 = 32,
}

impl Default for Processor_UpgradeMethod {
    fn default() -> Self {
        Self::Other
    }
}

