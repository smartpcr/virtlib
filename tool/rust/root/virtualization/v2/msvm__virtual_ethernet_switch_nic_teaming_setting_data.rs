// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualEthernetSwitchNicTeamingSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualEthernetSwitchNicTeamingSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchFeatureSettingData,

/// 
    #[serde(rename = "LoadBalancingAlgorithm")]
    pub load_balancing_algorithm: Option<u32>,

/// 
    #[serde(rename = "TeamingMode")]
    pub teaming_mode: Option<u32>,
}

impl Msvm_VirtualEthernetSwitchNicTeamingSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchFeatureSettingData::new(),
            load_balancing_algorithm: None,
            teaming_mode: None,
        }
    }


    /// Sets the value of LoadBalancingAlgorithm
    pub fn set_load_balancing_algorithm(&mut self, value: u32) {
        self.load_balancing_algorithm = Some(value);
    }

    /// Gets the value of LoadBalancingAlgorithm
    pub fn get_load_balancing_algorithm(&self) -> Option<&u32> {
        self.load_balancing_algorithm.as_ref()
    }

    /// Sets the value of TeamingMode
    pub fn set_teaming_mode(&mut self, value: u32) {
        self.teaming_mode = Some(value);
    }

    /// Gets the value of TeamingMode
    pub fn get_teaming_mode(&self) -> Option<&u32> {
        self.teaming_mode.as_ref()
    }
}

impl Msvm_VirtualEthernetSwitchNicTeamingSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

