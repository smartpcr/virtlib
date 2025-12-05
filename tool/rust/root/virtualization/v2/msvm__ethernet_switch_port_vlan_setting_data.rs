// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortVlanSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortVlanSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "AccessVlanId")]
    pub access_vlan_id: Option<u16>,

/// 
    #[serde(rename = "NativeVlanId")]
    pub native_vlan_id: Option<u16>,

/// 
    #[serde(rename = "OperationMode")]
    pub operation_mode: Option<u32>,

/// 
    #[serde(rename = "PrimaryVlanId")]
    pub primary_vlan_id: Option<u16>,

/// 
    #[serde(rename = "PruneVlanIdArray")]
    pub prune_vlan_id_array: Vec<u16>,

/// 
    #[serde(rename = "PvlanMode")]
    pub pvlan_mode: Option<u32>,

/// 
    #[serde(rename = "SecondaryVlanId")]
    pub secondary_vlan_id: Option<u16>,

/// 
    #[serde(rename = "SecondaryVlanIdArray")]
    pub secondary_vlan_id_array: Vec<u16>,

/// 
    #[serde(rename = "TrunkVlanIdArray")]
    pub trunk_vlan_id_array: Vec<u16>,
}

impl Msvm_EthernetSwitchPortVlanSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            access_vlan_id: None,
            native_vlan_id: None,
            operation_mode: None,
            primary_vlan_id: None,
            prune_vlan_id_array: Vec::new(),
            pvlan_mode: None,
            secondary_vlan_id: None,
            secondary_vlan_id_array: Vec::new(),
            trunk_vlan_id_array: Vec::new(),
        }
    }


    /// Sets the value of AccessVlanId
    pub fn set_access_vlan_id(&mut self, value: u16) {
        self.access_vlan_id = Some(value);
    }

    /// Gets the value of AccessVlanId
    pub fn get_access_vlan_id(&self) -> Option<&u16> {
        self.access_vlan_id.as_ref()
    }

    /// Sets the value of NativeVlanId
    pub fn set_native_vlan_id(&mut self, value: u16) {
        self.native_vlan_id = Some(value);
    }

    /// Gets the value of NativeVlanId
    pub fn get_native_vlan_id(&self) -> Option<&u16> {
        self.native_vlan_id.as_ref()
    }

    /// Sets the value of OperationMode
    pub fn set_operation_mode(&mut self, value: u32) {
        self.operation_mode = Some(value);
    }

    /// Gets the value of OperationMode
    pub fn get_operation_mode(&self) -> Option<&u32> {
        self.operation_mode.as_ref()
    }

    /// Sets the value of PrimaryVlanId
    pub fn set_primary_vlan_id(&mut self, value: u16) {
        self.primary_vlan_id = Some(value);
    }

    /// Gets the value of PrimaryVlanId
    pub fn get_primary_vlan_id(&self) -> Option<&u16> {
        self.primary_vlan_id.as_ref()
    }

    /// Sets the value of PruneVlanIdArray
    pub fn set_prune_vlan_id_array(&mut self, value: Vec<u16>) {
        self.prune_vlan_id_array = value;
    }

    /// Gets the value of PruneVlanIdArray
    pub fn get_prune_vlan_id_array(&self) -> &Vec<u16> {
        &self.prune_vlan_id_array
    }

    /// Sets the value of PvlanMode
    pub fn set_pvlan_mode(&mut self, value: u32) {
        self.pvlan_mode = Some(value);
    }

    /// Gets the value of PvlanMode
    pub fn get_pvlan_mode(&self) -> Option<&u32> {
        self.pvlan_mode.as_ref()
    }

    /// Sets the value of SecondaryVlanId
    pub fn set_secondary_vlan_id(&mut self, value: u16) {
        self.secondary_vlan_id = Some(value);
    }

    /// Gets the value of SecondaryVlanId
    pub fn get_secondary_vlan_id(&self) -> Option<&u16> {
        self.secondary_vlan_id.as_ref()
    }

    /// Sets the value of SecondaryVlanIdArray
    pub fn set_secondary_vlan_id_array(&mut self, value: Vec<u16>) {
        self.secondary_vlan_id_array = value;
    }

    /// Gets the value of SecondaryVlanIdArray
    pub fn get_secondary_vlan_id_array(&self) -> &Vec<u16> {
        &self.secondary_vlan_id_array
    }

    /// Sets the value of TrunkVlanIdArray
    pub fn set_trunk_vlan_id_array(&mut self, value: Vec<u16>) {
        self.trunk_vlan_id_array = value;
    }

    /// Gets the value of TrunkVlanIdArray
    pub fn get_trunk_vlan_id_array(&self) -> &Vec<u16> {
        &self.trunk_vlan_id_array
    }
}

impl Msvm_EthernetSwitchPortVlanSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

