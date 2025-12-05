// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchFeatureCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchFeatureCapabilities {
    #[serde(flatten)]
    pub base: CIM_Capabilities,

/// 
    #[serde(rename = "Applicability")]
    pub applicability: Option<EthernetSwitchFeatureCapabilities_Applicability>,

/// 
    #[serde(rename = "FeatureId")]
    pub feature_id: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl Msvm_EthernetSwitchFeatureCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Capabilities::new(),
            applicability: None,
            feature_id: None,
            version: None,
        }
    }


    /// Sets the value of Applicability
    pub fn set_applicability(&mut self, value: EthernetSwitchFeatureCapabilities_Applicability) {
        self.applicability = Some(value);
    }

    /// Gets the value of Applicability
    pub fn get_applicability(&self) -> Option<&EthernetSwitchFeatureCapabilities_Applicability> {
        self.applicability.as_ref()
    }

    /// Sets the value of FeatureId
    pub fn set_feature_id(&mut self, value: String) {
        self.feature_id = Some(value);
    }

    /// Gets the value of FeatureId
    pub fn get_feature_id(&self) -> Option<&String> {
        self.feature_id.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

impl Msvm_EthernetSwitchFeatureCapabilities {
    /// Gets the related Msvm_InstalledEthernetSwitchExtension object(s)
    pub fn get_related__installed_ethernet_switch_extension(&self) -> Result<Msvm_InstalledEthernetSwitchExtension, WmiError> {
        self.get_related("Msvm_InstalledEthernetSwitchExtension")
    }

    /// Gets the related Msvm_EthernetSwitchHardwareOffloadSettingData object(s)
    pub fn get_related__ethernet_switch_hardware_offload_setting_data(&self) -> Result<Msvm_EthernetSwitchHardwareOffloadSettingData, WmiError> {
        self.get_related("Msvm_EthernetSwitchHardwareOffloadSettingData")
    }

}

