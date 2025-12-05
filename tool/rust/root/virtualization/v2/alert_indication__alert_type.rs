// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AlertIndication_AlertType
//////////////////////////////////////////////

/// AlertIndication_AlertType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AlertIndication_AlertType {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Communications_Alert
    #[serde(rename = "Communications_Alert")]
    CommunicationsAlert = 2,
    /// Quality_of_Service_Alert
    #[serde(rename = "Quality_of_Service_Alert")]
    QualityOfServiceAlert = 3,
    /// Processing_Error
    #[serde(rename = "Processing_Error")]
    ProcessingError = 4,
    /// Device_Alert
    #[serde(rename = "Device_Alert")]
    DeviceAlert = 5,
    /// Environmental_Alert
    #[serde(rename = "Environmental_Alert")]
    EnvironmentalAlert = 6,
    /// Model_Change
    #[serde(rename = "Model_Change")]
    ModelChange = 7,
    /// Security_Alert
    #[serde(rename = "Security_Alert")]
    SecurityAlert = 8,
}

impl Default for AlertIndication_AlertType {
    fn default() -> Self {
        Self::Other
    }
}

