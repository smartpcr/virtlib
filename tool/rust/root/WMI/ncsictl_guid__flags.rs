// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NCSICtlGuid_Flags
//////////////////////////////////////////////

/// NCSICtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NCSICtlGuid_Flags {
    /// TM_NLAPROVIDER
    #[serde(rename = "TM_NLAPROVIDER")]
    TMNLAPROVIDER = 1,
    /// TM_INTERFACEHOP
    #[serde(rename = "TM_INTERFACEHOP")]
    TMINTERFACEHOP = 2,
    /// TM_MEDIA
    #[serde(rename = "TM_MEDIA")]
    TMMEDIA = 3,
    /// TM_WEBPROBE
    #[serde(rename = "TM_WEBPROBE")]
    TMWEBPROBE = 4,
    /// TM_SUPPORT
    #[serde(rename = "TM_SUPPORT")]
    TMSUPPORT = 5,
    /// TM_LOCK
    #[serde(rename = "TM_LOCK")]
    TMLOCK = 6,
}

impl Default for NCSICtlGuid_Flags {
    fn default() -> Self {
        Self::TMNLAPROVIDER
    }
}

