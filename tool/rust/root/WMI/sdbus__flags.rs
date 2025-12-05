// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Sdbus_Flags
//////////////////////////////////////////////

/// Sdbus_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Sdbus_Flags {
    /// SDBUS_DEBUG_FAIL
    #[serde(rename = "SDBUS_DEBUG_FAIL")]
    SDBUSDEBUGFAIL = 1,
    /// SDBUS_DEBUG_WARNING
    #[serde(rename = "SDBUS_DEBUG_WARNING")]
    SDBUSDEBUGWARNING = 2,
    /// SDBUS_DEBUG_INFO
    #[serde(rename = "SDBUS_DEBUG_INFO")]
    SDBUSDEBUGINFO = 3,
    /// SDBUS_DEBUG_PNP
    #[serde(rename = "SDBUS_DEBUG_PNP")]
    SDBUSDEBUGPNP = 4,
    /// SDBUS_DEBUG_POWER
    #[serde(rename = "SDBUS_DEBUG_POWER")]
    SDBUSDEBUGPOWER = 5,
    /// SDBUS_DEBUG_ENUM
    #[serde(rename = "SDBUS_DEBUG_ENUM")]
    SDBUSDEBUGENUM = 6,
    /// SDBUS_DEBUG_DEVINIT
    #[serde(rename = "SDBUS_DEBUG_DEVINIT")]
    SDBUSDEBUGDEVINIT = 7,
    /// SDBUS_DEBUG_DEVCFG
    #[serde(rename = "SDBUS_DEBUG_DEVCFG")]
    SDBUSDEBUGDEVCFG = 8,
    /// SDBUS_DEBUG_INTERFACE
    #[serde(rename = "SDBUS_DEBUG_INTERFACE")]
    SDBUSDEBUGINTERFACE = 9,
    /// SDBUS_DEBUG_DEVCMD
    #[serde(rename = "SDBUS_DEBUG_DEVCMD")]
    SDBUSDEBUGDEVCMD = 10,
    /// SDBUS_DEBUG_EVENT
    #[serde(rename = "SDBUS_DEBUG_EVENT")]
    SDBUSDEBUGEVENT = 11,
    /// SDBUS_DEBUG_CARD_EVT
    #[serde(rename = "SDBUS_DEBUG_CARD_EVT")]
    SDBUSDEBUGCARDEVT = 12,
    /// SDBUS_DEBUG_SDCMD
    #[serde(rename = "SDBUS_DEBUG_SDCMD")]
    SDBUSDEBUGSDCMD = 13,
    /// SDBUS_DEBUG_SDRESP
    #[serde(rename = "SDBUS_DEBUG_SDRESP")]
    SDBUSDEBUGSDRESP = 14,
    /// SDBUS_DEBUG_DEVICE
    #[serde(rename = "SDBUS_DEBUG_DEVICE")]
    SDBUSDEBUGDEVICE = 15,
    /// SDBUS_DEBUG_RECOVERY
    #[serde(rename = "SDBUS_DEBUG_RECOVERY")]
    SDBUSDEBUGRECOVERY = 16,
    /// SDBUS_DEBUG_TRANSFER
    #[serde(rename = "SDBUS_DEBUG_TRANSFER")]
    SDBUSDEBUGTRANSFER = 17,
    /// SDBUS_DEBUG_INACTIVITY
    #[serde(rename = "SDBUS_DEBUG_INACTIVITY")]
    SDBUSDEBUGINACTIVITY = 18,
    /// SDBUS_DEBUG_EVT_HANDLING
    #[serde(rename = "SDBUS_DEBUG_EVT_HANDLING")]
    SDBUSDEBUGEVTHANDLING = 19,
    /// SDBUS_DEBUG_WORKENG
    #[serde(rename = "SDBUS_DEBUG_WORKENG")]
    SDBUSDEBUGWORKENG = 20,
    /// SDBUS_DEBUG_WORKENG2
    #[serde(rename = "SDBUS_DEBUG_WORKENG2")]
    SDBUSDEBUGWORKENG2 = 21,
    /// SDBUS_DEBUG_DUMP_REGS
    #[serde(rename = "SDBUS_DEBUG_DUMP_REGS")]
    SDBUSDEBUGDUMPREGS = 22,
}

impl Default for Sdbus_Flags {
    fn default() -> Self {
        Self::SDBUSDEBUGFAIL
    }
}

