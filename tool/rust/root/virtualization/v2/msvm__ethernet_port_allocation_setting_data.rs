// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetPortAllocationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetPortAllocationSettingData {
    #[serde(flatten)]
    pub base: CIM_EthernetPortAllocationSettingData,

/// 
    #[serde(rename = "CompartmentGuid")]
    pub compartment_guid: Option<String>,

/// EnabledState is an integer enumeration that indicates whether the allocation request is enabled or disabled. When an allocation request is marked as Disabled (3), then the allocation is not processed. The EnabledState for an active configuration is always marked as Enabled (2).
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<EthernetPortAllocationSettingData_EnabledState>,

/// The last known friendly name of the switch this port had a hard affinity to, if any.
    #[serde(rename = "LastKnownSwitchName")]
    pub last_known_switch_name: Option<String>,

/// 
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,

/// 
    #[serde(rename = "RequiredFeatureHints")]
    pub required_feature_hints: Vec<String>,

/// 
    #[serde(rename = "RequiredFeatures")]
    pub required_features: Vec<String>,

/// 
    #[serde(rename = "TestReplicaPoolID")]
    pub test_replica_pool_id: Option<String>,

/// 
    #[serde(rename = "TestReplicaSwitchName")]
    pub test_replica_switch_name: Option<String>,
}

impl Msvm_EthernetPortAllocationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EthernetPortAllocationSettingData::new(),
            compartment_guid: None,
            enabled_state: None,
            last_known_switch_name: None,
            port_name: None,
            required_feature_hints: Vec::new(),
            required_features: Vec::new(),
            test_replica_pool_id: None,
            test_replica_switch_name: None,
        }
    }


    /// Sets the value of CompartmentGuid
    pub fn set_compartment_guid(&mut self, value: String) {
        self.compartment_guid = Some(value);
    }

    /// Gets the value of CompartmentGuid
    pub fn get_compartment_guid(&self) -> Option<&String> {
        self.compartment_guid.as_ref()
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: EthernetPortAllocationSettingData_EnabledState) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&EthernetPortAllocationSettingData_EnabledState> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of LastKnownSwitchName
    pub fn set_last_known_switch_name(&mut self, value: String) {
        self.last_known_switch_name = Some(value);
    }

    /// Gets the value of LastKnownSwitchName
    pub fn get_last_known_switch_name(&self) -> Option<&String> {
        self.last_known_switch_name.as_ref()
    }

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }

    /// Sets the value of RequiredFeatureHints
    pub fn set_required_feature_hints(&mut self, value: Vec<String>) {
        self.required_feature_hints = value;
    }

    /// Gets the value of RequiredFeatureHints
    pub fn get_required_feature_hints(&self) -> &Vec<String> {
        &self.required_feature_hints
    }

    /// Sets the value of RequiredFeatures
    pub fn set_required_features(&mut self, value: Vec<String>) {
        self.required_features = value;
    }

    /// Gets the value of RequiredFeatures
    pub fn get_required_features(&self) -> &Vec<String> {
        &self.required_features
    }

    /// Sets the value of TestReplicaPoolID
    pub fn set_test_replica_pool_id(&mut self, value: String) {
        self.test_replica_pool_id = Some(value);
    }

    /// Gets the value of TestReplicaPoolID
    pub fn get_test_replica_pool_id(&self) -> Option<&String> {
        self.test_replica_pool_id.as_ref()
    }

    /// Sets the value of TestReplicaSwitchName
    pub fn set_test_replica_switch_name(&mut self, value: String) {
        self.test_replica_switch_name = Some(value);
    }

    /// Gets the value of TestReplicaSwitchName
    pub fn get_test_replica_switch_name(&self) -> Option<&String> {
        self.test_replica_switch_name.as_ref()
    }
}

impl Msvm_EthernetPortAllocationSettingData {
    /// Gets the related Msvm_VirtualEthernetSwitchSettingData object(s)
    pub fn get_related__virtual_ethernet_switch_setting_data(&self) -> Result<Msvm_VirtualEthernetSwitchSettingData, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitchSettingData")
    }

    /// Gets the related Msvm_EthernetSwitchPort object(s)
    pub fn get_related__ethernet_switch_port(&self) -> Result<Msvm_EthernetSwitchPort, WmiError> {
        self.get_related("Msvm_EthernetSwitchPort")
    }

}

