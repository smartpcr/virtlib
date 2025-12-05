// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FibrePortNPIVMethods_Status
//////////////////////////////////////////////

/// FibrePortNPIVMethods_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FibrePortNPIVMethods_Status {
    /// NPIV_SUCCESS
    #[serde(rename = "NPIV_SUCCESS")]
    NPIVSUCCESS = 1,
    /// NPIV_UNKNOWN_ERROR
    #[serde(rename = "NPIV_UNKNOWN_ERROR")]
    NPIVUNKNOWNERROR = 2,
    /// NPIV_NOT_SUPPORTED_HOST
    #[serde(rename = "NPIV_NOT_SUPPORTED_HOST")]
    NPIVNOTSUPPORTEDHOST = 3,
    /// NPIV_NOT_SUPPORTED_FABRIC
    #[serde(rename = "NPIV_NOT_SUPPORTED_FABRIC")]
    NPIVNOTSUPPORTEDFABRIC = 4,
    /// NPIV_OUT_OF_RESOURCES
    #[serde(rename = "NPIV_OUT_OF_RESOURCES")]
    NPIVOUTOFRESOURCES = 5,
    /// NPIV_MAX_VPORT_COUNT
    #[serde(rename = "NPIV_MAX_VPORT_COUNT")]
    NPIVMAXVPORTCOUNT = 6,
    /// NPIV_WWPN_IN_USE
    #[serde(rename = "NPIV_WWPN_IN_USE")]
    NPIVWWPNINUSE = 7,
    /// NPIV_WWPN_INVALID_FORMAT
    #[serde(rename = "NPIV_WWPN_INVALID_FORMAT")]
    NPIVWWPNINVALIDFORMAT = 8,
    /// NPIV_LINK_DOWN
    #[serde(rename = "NPIV_LINK_DOWN")]
    NPIVLINKDOWN = 9,
    /// NPIV_WWPN_NOT_FOUND
    #[serde(rename = "NPIV_WWPN_NOT_FOUND")]
    NPIVWWPNNOTFOUND = 10,
}

impl Default for FibrePortNPIVMethods_Status {
    fn default() -> Self {
        Self::NPIVSUCCESS
    }
}

