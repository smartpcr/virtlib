// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiIPSecOffloadV1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiIPSecOffloadV1 {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "WmiIPv4AH")]
    pub wmi_ipv4_ah: Option<MSNdis_WmiIPSecOffloadV1_IPv4AH>,

/// 
    #[serde(rename = "WmiIPv4ESP")]
    pub wmi_ipv4_esp: Option<MSNdis_WmiIPSecOffloadV1_IPv4ESP>,

/// 
    #[serde(rename = "WmiSupported")]
    pub wmi_supported: Option<MSNdis_WmiIPSecOffloadV1_Supported>,
}

impl MSNdis_WmiIPSecOffloadV1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            wmi_ipv4_ah: None,
            wmi_ipv4_esp: None,
            wmi_supported: None,
        }
    }


    /// Sets the value of WmiIPv4AH
    pub fn set_wmi_ipv4_ah(&mut self, value: MSNdis_WmiIPSecOffloadV1_IPv4AH) {
        self.wmi_ipv4_ah = Some(value);
    }

    /// Gets the value of WmiIPv4AH
    pub fn get_wmi_ipv4_ah(&self) -> Option<&MSNdis_WmiIPSecOffloadV1_IPv4AH> {
        self.wmi_ipv4_ah.as_ref()
    }

    /// Sets the value of WmiIPv4ESP
    pub fn set_wmi_ipv4_esp(&mut self, value: MSNdis_WmiIPSecOffloadV1_IPv4ESP) {
        self.wmi_ipv4_esp = Some(value);
    }

    /// Gets the value of WmiIPv4ESP
    pub fn get_wmi_ipv4_esp(&self) -> Option<&MSNdis_WmiIPSecOffloadV1_IPv4ESP> {
        self.wmi_ipv4_esp.as_ref()
    }

    /// Sets the value of WmiSupported
    pub fn set_wmi_supported(&mut self, value: MSNdis_WmiIPSecOffloadV1_Supported) {
        self.wmi_supported = Some(value);
    }

    /// Gets the value of WmiSupported
    pub fn get_wmi_supported(&self) -> Option<&MSNdis_WmiIPSecOffloadV1_Supported> {
        self.wmi_supported.as_ref()
    }
}

