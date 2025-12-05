// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortTeamMappingSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortTeamMappingSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "DisableOnFailover")]
    pub disable_on_failover: Option<u32>,

/// 
    #[serde(rename = "NetAdapterDeviceId")]
    pub net_adapter_device_id: Option<String>,

/// 
    #[serde(rename = "NetAdapterName")]
    pub net_adapter_name: Option<String>,
}

impl Msvm_EthernetSwitchPortTeamMappingSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            disable_on_failover: None,
            net_adapter_device_id: None,
            net_adapter_name: None,
        }
    }


    /// Sets the value of DisableOnFailover
    pub fn set_disable_on_failover(&mut self, value: u32) {
        self.disable_on_failover = Some(value);
    }

    /// Gets the value of DisableOnFailover
    pub fn get_disable_on_failover(&self) -> Option<&u32> {
        self.disable_on_failover.as_ref()
    }

    /// Sets the value of NetAdapterDeviceId
    pub fn set_net_adapter_device_id(&mut self, value: String) {
        self.net_adapter_device_id = Some(value);
    }

    /// Gets the value of NetAdapterDeviceId
    pub fn get_net_adapter_device_id(&self) -> Option<&String> {
        self.net_adapter_device_id.as_ref()
    }

    /// Sets the value of NetAdapterName
    pub fn set_net_adapter_name(&mut self, value: String) {
        self.net_adapter_name = Some(value);
    }

    /// Gets the value of NetAdapterName
    pub fn get_net_adapter_name(&self) -> Option<&String> {
        self.net_adapter_name.as_ref()
    }
}

impl Msvm_EthernetSwitchPortTeamMappingSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

