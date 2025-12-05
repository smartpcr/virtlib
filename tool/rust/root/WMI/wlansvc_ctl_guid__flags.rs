// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WlansvcCtlGuid_Flags
//////////////////////////////////////////////

/// WlansvcCtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WlansvcCtlGuid_Flags {
    /// DOT11_AUTOCONF
    #[serde(rename = "DOT11_AUTOCONF")]
    DOT11AUTOCONF = 1,
    /// DOT11_AUTOCONF_CLIENT
    #[serde(rename = "DOT11_AUTOCONF_CLIENT")]
    DOT11AUTOCONFCLIENT = 2,
    /// DOT11_AUTOCONF_UI
    #[serde(rename = "DOT11_AUTOCONF_UI")]
    DOT11AUTOCONFUI = 3,
    /// DOT11_FATMSM
    #[serde(rename = "DOT11_FATMSM")]
    DOT11FATMSM = 4,
    /// DOT11_COMMON
    #[serde(rename = "DOT11_COMMON")]
    DOT11COMMON = 5,
    /// DOT11_WLANGPA
    #[serde(rename = "DOT11_WLANGPA")]
    DOT11WLANGPA = 6,
    /// DOT11_CLASS_COINSTALLER
    #[serde(rename = "DOT11_CLASS_COINSTALLER")]
    DOT11CLASSCOINSTALLER = 7,
    /// DOT11_MSM
    #[serde(rename = "DOT11_MSM")]
    DOT11MSM = 8,
    /// DOT11_MSM_ADAPT
    #[serde(rename = "DOT11_MSM_ADAPT")]
    DOT11MSMADAPT = 9,
    /// DOT11_MSM_SCAN
    #[serde(rename = "DOT11_MSM_SCAN")]
    DOT11MSMSCAN = 10,
    /// DOT11_MSM_CONNECT
    #[serde(rename = "DOT11_MSM_CONNECT")]
    DOT11MSMCONNECT = 11,
    /// DOT11_MSM_SECURITY_PKT
    #[serde(rename = "DOT11_MSM_SECURITY_PKT")]
    DOT11MSMSECURITYPKT = 12,
}

impl Default for WlansvcCtlGuid_Flags {
    fn default() -> Self {
        Self::DOT11AUTOCONF
    }
}

