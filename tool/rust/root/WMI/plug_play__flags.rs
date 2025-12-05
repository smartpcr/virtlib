// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PlugPlay_Flags
//////////////////////////////////////////////

/// PlugPlay_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PlugPlay_Flags {
    /// PNP_ERROR_LEVEL
    #[serde(rename = "PNP_ERROR_LEVEL")]
    PNPERRORLEVEL = 1,
    /// PNP_WARNING_LEVEL
    #[serde(rename = "PNP_WARNING_LEVEL")]
    PNPWARNINGLEVEL = 2,
    /// PNP_TRACE_LEVEL
    #[serde(rename = "PNP_TRACE_LEVEL")]
    PNPTRACELEVEL = 3,
    /// PNP_INFO_LEVEL
    #[serde(rename = "PNP_INFO_LEVEL")]
    PNPINFOLEVEL = 4,
    /// PNP_REGISTRY_WARNING_LEVEL
    #[serde(rename = "PNP_REGISTRY_WARNING_LEVEL")]
    PNPREGISTRYWARNINGLEVEL = 5,
    /// PNP_REGISTRY_TRACE_LEVEL
    #[serde(rename = "PNP_REGISTRY_TRACE_LEVEL")]
    PNPREGISTRYTRACELEVEL = 6,
    /// PNP_REGISTRY_INFO_LEVEL
    #[serde(rename = "PNP_REGISTRY_INFO_LEVEL")]
    PNPREGISTRYINFOLEVEL = 7,
    /// PNP_REGISTRY_VERBOSE_LEVEL
    #[serde(rename = "PNP_REGISTRY_VERBOSE_LEVEL")]
    PNPREGISTRYVERBOSELEVEL = 8,
    /// PNP_RESOURCE_WARNING_LEVEL
    #[serde(rename = "PNP_RESOURCE_WARNING_LEVEL")]
    PNPRESOURCEWARNINGLEVEL = 9,
    /// PNP_RESOURCE_TRACE_LEVEL
    #[serde(rename = "PNP_RESOURCE_TRACE_LEVEL")]
    PNPRESOURCETRACELEVEL = 10,
    /// PNP_RESOURCE_INFO_LEVEL
    #[serde(rename = "PNP_RESOURCE_INFO_LEVEL")]
    PNPRESOURCEINFOLEVEL = 11,
    /// PNP_RESOURCE_VERBOSE_LEVEL
    #[serde(rename = "PNP_RESOURCE_VERBOSE_LEVEL")]
    PNPRESOURCEVERBOSELEVEL = 12,
    /// PNP_EVENT_WARNING_LEVEL
    #[serde(rename = "PNP_EVENT_WARNING_LEVEL")]
    PNPEVENTWARNINGLEVEL = 13,
    /// PNP_EVENT_TRACE_LEVEL
    #[serde(rename = "PNP_EVENT_TRACE_LEVEL")]
    PNPEVENTTRACELEVEL = 14,
    /// PNP_EVENT_INFO_LEVEL
    #[serde(rename = "PNP_EVENT_INFO_LEVEL")]
    PNPEVENTINFOLEVEL = 15,
    /// PNP_EVENT_VERBOSE_LEVEL
    #[serde(rename = "PNP_EVENT_VERBOSE_LEVEL")]
    PNPEVENTVERBOSELEVEL = 16,
    /// PNP_INSTALL_WARNING_LEVEL
    #[serde(rename = "PNP_INSTALL_WARNING_LEVEL")]
    PNPINSTALLWARNINGLEVEL = 17,
    /// PNP_INSTALL_TRACE_LEVEL
    #[serde(rename = "PNP_INSTALL_TRACE_LEVEL")]
    PNPINSTALLTRACELEVEL = 18,
    /// PNP_INSTALL_INFO_LEVEL
    #[serde(rename = "PNP_INSTALL_INFO_LEVEL")]
    PNPINSTALLINFOLEVEL = 19,
    /// PNP_INSTALL_VERBOSE_LEVEL
    #[serde(rename = "PNP_INSTALL_VERBOSE_LEVEL")]
    PNPINSTALLVERBOSELEVEL = 20,
}

impl Default for PlugPlay_Flags {
    fn default() -> Self {
        Self::PNPERRORLEVEL
    }
}

