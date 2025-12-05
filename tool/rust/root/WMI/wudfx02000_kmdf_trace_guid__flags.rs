// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Wudfx02000KmdfTraceGuid_Flags
//////////////////////////////////////////////

/// Wudfx02000KmdfTraceGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Wudfx02000KmdfTraceGuid_Flags {
    /// TRACINGFULL
    #[serde(rename = "TRACINGFULL")]
    TRACINGFULL = 0,
    /// TRACINGERROR
    #[serde(rename = "TRACINGERROR")]
    TRACINGERROR = 1,
    /// TRACINGDBGPRINT
    #[serde(rename = "TRACINGDBGPRINT")]
    TRACINGDBGPRINT = 2,
    /// TRACINGFRAMEWORKS
    #[serde(rename = "TRACINGFRAMEWORKS")]
    TRACINGFRAMEWORKS = 3,
    /// TRACINGAPI
    #[serde(rename = "TRACINGAPI")]
    TRACINGAPI = 4,
    /// TRACINGAPIERROR
    #[serde(rename = "TRACINGAPIERROR")]
    TRACINGAPIERROR = 5,
    /// TRACINGRESOURCES
    #[serde(rename = "TRACINGRESOURCES")]
    TRACINGRESOURCES = 6,
    /// TRACINGLOCKING
    #[serde(rename = "TRACINGLOCKING")]
    TRACINGLOCKING = 7,
    /// TRACINGCONTEXT
    #[serde(rename = "TRACINGCONTEXT")]
    TRACINGCONTEXT = 8,
    /// TRACINGPOOL
    #[serde(rename = "TRACINGPOOL")]
    TRACINGPOOL = 9,
    /// TRACINGHANDLE
    #[serde(rename = "TRACINGHANDLE")]
    TRACINGHANDLE = 10,
    /// TRACINGPNP
    #[serde(rename = "TRACINGPNP")]
    TRACINGPNP = 11,
    /// TRACINGIO
    #[serde(rename = "TRACINGIO")]
    TRACINGIO = 12,
    /// TRACINGIOTARGET
    #[serde(rename = "TRACINGIOTARGET")]
    TRACINGIOTARGET = 13,
    /// TRACINGDMA
    #[serde(rename = "TRACINGDMA")]
    TRACINGDMA = 14,
    /// TRACINGREQUEST
    #[serde(rename = "TRACINGREQUEST")]
    TRACINGREQUEST = 15,
    /// TRACINGDRIVER
    #[serde(rename = "TRACINGDRIVER")]
    TRACINGDRIVER = 16,
    /// TRACINGDEVICE
    #[serde(rename = "TRACINGDEVICE")]
    TRACINGDEVICE = 17,
    /// TRACINGUSEROBJECT
    #[serde(rename = "TRACINGUSEROBJECT")]
    TRACINGUSEROBJECT = 18,
    /// TRACINGOBJECT
    #[serde(rename = "TRACINGOBJECT")]
    TRACINGOBJECT = 19,
    /// TRACINGPNPPOWERSTATES
    #[serde(rename = "TRACINGPNPPOWERSTATES")]
    TRACINGPNPPOWERSTATES = 20,
}

impl Default for Wudfx02000KmdfTraceGuid_Flags {
    fn default() -> Self {
        Self::TRACINGFULL
    }
}

