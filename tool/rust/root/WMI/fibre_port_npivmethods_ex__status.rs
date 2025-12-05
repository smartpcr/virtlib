// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FibrePortNPIVMethodsEx_Status
//////////////////////////////////////////////

/// FibrePortNPIVMethodsEx_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FibrePortNPIVMethodsEx_Status {
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
    /// NPIV_AUTHENTICATION_MECHANISM_NOT_USABLE
    #[serde(rename = "NPIV_AUTHENTICATION_MECHANISM_NOT_USABLE")]
    NPIVAUTHENTICATIONMECHANISMNOTUSABLE = 6,
    /// NPIV_HASH_FUNCTION_NOT_USABLE
    #[serde(rename = "NPIV_HASH_FUNCTION_NOT_USABLE")]
    NPIVHASHFUNCTIONNOTUSABLE = 7,
    /// NPIV_AUTHENTICATION_TRANSACTION_ALREADY_STARTED
    #[serde(rename = "NPIV_AUTHENTICATION_TRANSACTION_ALREADY_STARTED")]
    NPIVAUTHENTICATIONTRANSACTIONALREADYSTARTED = 8,
    /// NPIV_AUTHENTICATION_FAILED
    #[serde(rename = "NPIV_AUTHENTICATION_FAILED")]
    NPIVAUTHENTICATIONFAILED = 9,
    /// NPIV_UNSUPPORTED_PROTOCOL_VERSION
    #[serde(rename = "NPIV_UNSUPPORTED_PROTOCOL_VERSION")]
    NPIVUNSUPPORTEDPROTOCOLVERSION = 10,
}

impl Default for FibrePortNPIVMethodsEx_Status {
    fn default() -> Self {
        Self::NPIVSUCCESS
    }
}

