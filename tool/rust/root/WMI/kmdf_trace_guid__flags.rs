// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source KmdfTraceGuid_Flags
//////////////////////////////////////////////

/// KmdfTraceGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum KmdfTraceGuid_Flags {
    /// TRACINGFULL
    #[serde(rename = "TRACINGFULL")]
    TRACINGFULL = 1,
    /// TRACINGERROR
    #[serde(rename = "TRACINGERROR")]
    TRACINGERROR = 2,
    /// TRACINGDBGPRINT
    #[serde(rename = "TRACINGDBGPRINT")]
    TRACINGDBGPRINT = 3,
    /// TRACINGFRAMEWORKS
    #[serde(rename = "TRACINGFRAMEWORKS")]
    TRACINGFRAMEWORKS = 4,
    /// TRACINGAPI
    #[serde(rename = "TRACINGAPI")]
    TRACINGAPI = 5,
    /// TRACINGAPIERROR
    #[serde(rename = "TRACINGAPIERROR")]
    TRACINGAPIERROR = 6,
    /// TRACINGRESOURCES
    #[serde(rename = "TRACINGRESOURCES")]
    TRACINGRESOURCES = 7,
    /// TRACINGLOCKING
    #[serde(rename = "TRACINGLOCKING")]
    TRACINGLOCKING = 8,
    /// TRACINGCONTEXT
    #[serde(rename = "TRACINGCONTEXT")]
    TRACINGCONTEXT = 9,
    /// TRACINGPOOL
    #[serde(rename = "TRACINGPOOL")]
    TRACINGPOOL = 10,
    /// TRACINGHANDLE
    #[serde(rename = "TRACINGHANDLE")]
    TRACINGHANDLE = 11,
    /// TRACINGPNP
    #[serde(rename = "TRACINGPNP")]
    TRACINGPNP = 12,
    /// TRACINGIO
    #[serde(rename = "TRACINGIO")]
    TRACINGIO = 13,
    /// TRACINGIOTARGET
    #[serde(rename = "TRACINGIOTARGET")]
    TRACINGIOTARGET = 14,
    /// TRACINGDMA
    #[serde(rename = "TRACINGDMA")]
    TRACINGDMA = 15,
    /// TRACINGREQUEST
    #[serde(rename = "TRACINGREQUEST")]
    TRACINGREQUEST = 16,
    /// TRACINGDRIVER
    #[serde(rename = "TRACINGDRIVER")]
    TRACINGDRIVER = 17,
    /// TRACINGDEVICE
    #[serde(rename = "TRACINGDEVICE")]
    TRACINGDEVICE = 18,
    /// TRACINGUSEROBJECT
    #[serde(rename = "TRACINGUSEROBJECT")]
    TRACINGUSEROBJECT = 19,
    /// TRACINGOBJECT
    #[serde(rename = "TRACINGOBJECT")]
    TRACINGOBJECT = 20,
    /// TRACINGPNPPOWERSTATES
    #[serde(rename = "TRACINGPNPPOWERSTATES")]
    TRACINGPNPPOWERSTATES = 21,
}

impl Default for KmdfTraceGuid_Flags {
    fn default() -> Self {
        Self::TRACINGFULL
    }
}

