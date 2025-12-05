// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported
//////////////////////////////////////////////

/// VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported {
    /// DomainNameFormatSupported
    #[serde(rename = "DomainNameFormatSupported")]
    DomainNameFormatSupported = 2,
    /// IPv4DottedDecimalFormatSupported
    #[serde(rename = "IPv4DottedDecimalFormatSupported")]
    IPv4DottedDecimalFormatSupported = 3,
    /// IPv6TextFormatSupported
    #[serde(rename = "IPv6TextFormatSupported")]
    IPv6TextFormatSupported = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
}

impl Default for VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported {
    fn default() -> Self {
        Self::DomainNameFormatSupported
    }
}

