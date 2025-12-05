// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source iScsi_Flags
//////////////////////////////////////////////

/// iScsi_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum iScsi_Flags {
    /// iScsiFlagGeneral
    #[serde(rename = "iScsiFlagGeneral")]
    IScsiFlagGeneral = 1,
    /// iScsiFlagAdapter
    #[serde(rename = "iScsiFlagAdapter")]
    IScsiFlagAdapter = 2,
    /// iScsiFlagUnit
    #[serde(rename = "iScsiFlagUnit")]
    IScsiFlagUnit = 3,
    /// iScsiFlagPnP
    #[serde(rename = "iScsiFlagPnP")]
    IScsiFlagPnP = 4,
    /// iScsiFlagPower
    #[serde(rename = "iScsiFlagPower")]
    IScsiFlagPower = 5,
    /// iScsiFlagIoctl
    #[serde(rename = "iScsiFlagIoctl")]
    IScsiFlagIoctl = 6,
    /// iScsiFlagQueue
    #[serde(rename = "iScsiFlagQueue")]
    IScsiFlagQueue = 7,
    /// iScsiFlagWmi
    #[serde(rename = "iScsiFlagWmi")]
    IScsiFlagWmi = 8,
    /// iScsiFlagTimer
    #[serde(rename = "iScsiFlagTimer")]
    IScsiFlagTimer = 9,
    /// iScsiFlagInit
    #[serde(rename = "iScsiFlagInit")]
    IScsiFlagInit = 10,
    /// iScsiFlagLock
    #[serde(rename = "iScsiFlagLock")]
    IScsiFlagLock = 11,
    /// iScsiFlagIsr
    #[serde(rename = "iScsiFlagIsr")]
    IScsiFlagIsr = 12,
    /// iScsiFlagEnum
    #[serde(rename = "iScsiFlagEnum")]
    IScsiFlagEnum = 13,
    /// iScsiFlagBoot
    #[serde(rename = "iScsiFlagBoot")]
    IScsiFlagBoot = 14,
    /// iScsiFlagErl
    #[serde(rename = "iScsiFlagErl")]
    IScsiFlagErl = 15,
    /// iScsiFlagNetwork
    #[serde(rename = "iScsiFlagNetwork")]
    IScsiFlagNetwork = 16,
    /// iScsiFlagRadius
    #[serde(rename = "iScsiFlagRadius")]
    IScsiFlagRadius = 17,
    /// iScsiFlagSockets
    #[serde(rename = "iScsiFlagSockets")]
    IScsiFlagSockets = 18,
    /// iScsiFlagScsi
    #[serde(rename = "iScsiFlagScsi")]
    IScsiFlagScsi = 19,
    /// iScsiFlagScsiDataIO
    #[serde(rename = "iScsiFlagScsiDataIO")]
    IScsiFlagScsiDataIO = 20,
    /// iScsiFlagProtocolLogInOut
    #[serde(rename = "iScsiFlagProtocolLogInOut")]
    IScsiFlagProtocolLogInOut = 21,
    /// iScsiFlagProtocolTMF
    #[serde(rename = "iScsiFlagProtocolTMF")]
    IScsiFlagProtocolTMF = 22,
    /// iScsiFlagProtocolTransfer
    #[serde(rename = "iScsiFlagProtocolTransfer")]
    IScsiFlagProtocolTransfer = 23,
    /// iScsiFlagCHAP
    #[serde(rename = "iScsiFlagCHAP")]
    IScsiFlagCHAP = 24,
    /// iScsiFlagDigest
    #[serde(rename = "iScsiFlagDigest")]
    IScsiFlagDigest = 25,
    /// iScsiFlagIpSec
    #[serde(rename = "iScsiFlagIpSec")]
    IScsiFlagIpSec = 26,
    /// iScsiDebug26
    #[serde(rename = "iScsiDebug26")]
    IScsiDebug26 = 27,
    /// iScsiDebug27
    #[serde(rename = "iScsiDebug27")]
    IScsiDebug27 = 28,
    /// iScsiDebug28
    #[serde(rename = "iScsiDebug28")]
    IScsiDebug28 = 29,
    /// iScsiDebug29
    #[serde(rename = "iScsiDebug29")]
    IScsiDebug29 = 30,
    /// iScsiDebug30
    #[serde(rename = "iScsiDebug30")]
    IScsiDebug30 = 31,
    /// iScsiDebug31
    #[serde(rename = "iScsiDebug31")]
    IScsiDebug31 = 32,
}

impl Default for iScsi_Flags {
    fn default() -> Self {
        Self::IScsiFlagGeneral
    }
}

