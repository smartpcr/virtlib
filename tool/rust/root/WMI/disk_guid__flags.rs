// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DiskGuid_Flags
//////////////////////////////////////////////

/// DiskGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DiskGuid_Flags {
    /// TRACE_FLAG_GENERAL
    #[serde(rename = "TRACE_FLAG_GENERAL")]
    TRACEFLAGGENERAL = 1,
    /// TRACE_FLAG_PNP
    #[serde(rename = "TRACE_FLAG_PNP")]
    TRACEFLAGPNP = 2,
    /// TRACE_FLAG_POWER
    #[serde(rename = "TRACE_FLAG_POWER")]
    TRACEFLAGPOWER = 3,
    /// TRACE_FLAG_RW
    #[serde(rename = "TRACE_FLAG_RW")]
    TRACEFLAGRW = 4,
    /// TRACE_FLAG_IOCTL
    #[serde(rename = "TRACE_FLAG_IOCTL")]
    TRACEFLAGIOCTL = 5,
    /// TRACE_FLAG_QUEUE
    #[serde(rename = "TRACE_FLAG_QUEUE")]
    TRACEFLAGQUEUE = 6,
    /// TRACE_FLAG_WMI
    #[serde(rename = "TRACE_FLAG_WMI")]
    TRACEFLAGWMI = 7,
    /// TRACE_FLAG_TIMER
    #[serde(rename = "TRACE_FLAG_TIMER")]
    TRACEFLAGTIMER = 8,
    /// TRACE_FLAG_INIT
    #[serde(rename = "TRACE_FLAG_INIT")]
    TRACEFLAGINIT = 9,
    /// TRACE_FLAG_LOCK
    #[serde(rename = "TRACE_FLAG_LOCK")]
    TRACEFLAGLOCK = 10,
    /// TRACE_FLAG_DEBUG1
    #[serde(rename = "TRACE_FLAG_DEBUG1")]
    TRACEFLAGDEBUG1 = 11,
    /// TRACE_FLAG_DEBUG2
    #[serde(rename = "TRACE_FLAG_DEBUG2")]
    TRACEFLAGDEBUG2 = 12,
    /// TRACE_FLAG_MCN
    #[serde(rename = "TRACE_FLAG_MCN")]
    TRACEFLAGMCN = 13,
    /// TRACE_FLAG_ISR
    #[serde(rename = "TRACE_FLAG_ISR")]
    TRACEFLAGISR = 14,
    /// TRACE_FLAG_ENUM
    #[serde(rename = "TRACE_FLAG_ENUM")]
    TRACEFLAGENUM = 15,
}

impl Default for DiskGuid_Flags {
    fn default() -> Self {
        Self::TRACEFLAGGENERAL
    }
}

