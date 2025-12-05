// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterSriovSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterSriovSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "CurrentCapabilities")]
    pub current_capabilities: Option<MSFT_NetAdapterSriovCapabilities>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "HardwareCapabilities")]
    pub hardware_capabilities: Option<MSFT_NetAdapterSriovCapabilities>,

/// 
    #[serde(rename = "NumActiveDefaultVPortMacAddresses")]
    pub num_active_default_vport_mac_addresses: Option<u32>,

/// 
    #[serde(rename = "NumActiveDefaultVPortVlanIds")]
    pub num_active_default_vport_vlan_ids: Option<u32>,

/// 
    #[serde(rename = "NumActiveNonDefaultVPortMacAddresses")]
    pub num_active_non_default_vport_mac_addresses: Option<u32>,

/// 
    #[serde(rename = "NumActiveNonDefaultVPortVlanIds")]
    pub num_active_non_default_vport_vlan_ids: Option<u32>,

/// 
    #[serde(rename = "NumActiveVPorts")]
    pub num_active_vports: Option<u32>,

/// 
    #[serde(rename = "NumAllocatedVFs")]
    pub num_allocated_vfs: Option<u32>,

/// 
    #[serde(rename = "NumQueuePairsForDefaultVPort")]
    pub num_queue_pairs_for_default_vport: Option<u32>,

/// 
    #[serde(rename = "NumQueuePairsForNonDefaultVPorts")]
    pub num_queue_pairs_for_non_default_vports: Option<u32>,

/// 
    #[serde(rename = "NumVFs")]
    pub num_vfs: Option<u32>,

/// 
    #[serde(rename = "NumVPorts")]
    pub num_vports: Option<u32>,

/// 
    #[serde(rename = "SriovSupport")]
    pub sriov_support: Option<u32>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,

/// 
    #[serde(rename = "SwitchType")]
    pub switch_type: Option<u16>,
}

impl MSFT_NetAdapterSriovSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            current_capabilities: None,
            enabled: None,
            hardware_capabilities: None,
            num_active_default_vport_mac_addresses: None,
            num_active_default_vport_vlan_ids: None,
            num_active_non_default_vport_mac_addresses: None,
            num_active_non_default_vport_vlan_ids: None,
            num_active_vports: None,
            num_allocated_vfs: None,
            num_queue_pairs_for_default_vport: None,
            num_queue_pairs_for_non_default_vports: None,
            num_vfs: None,
            num_vports: None,
            sriov_support: None,
            switch_name: None,
            switch_type: None,
        }
    }


    /// Sets the value of CurrentCapabilities
    pub fn set_current_capabilities(&mut self, value: MSFT_NetAdapterSriovCapabilities) {
        self.current_capabilities = Some(value);
    }

    /// Gets the value of CurrentCapabilities
    pub fn get_current_capabilities(&self) -> Option<&MSFT_NetAdapterSriovCapabilities> {
        self.current_capabilities.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of HardwareCapabilities
    pub fn set_hardware_capabilities(&mut self, value: MSFT_NetAdapterSriovCapabilities) {
        self.hardware_capabilities = Some(value);
    }

    /// Gets the value of HardwareCapabilities
    pub fn get_hardware_capabilities(&self) -> Option<&MSFT_NetAdapterSriovCapabilities> {
        self.hardware_capabilities.as_ref()
    }

    /// Sets the value of NumActiveDefaultVPortMacAddresses
    pub fn set_num_active_default_vport_mac_addresses(&mut self, value: u32) {
        self.num_active_default_vport_mac_addresses = Some(value);
    }

    /// Gets the value of NumActiveDefaultVPortMacAddresses
    pub fn get_num_active_default_vport_mac_addresses(&self) -> Option<&u32> {
        self.num_active_default_vport_mac_addresses.as_ref()
    }

    /// Sets the value of NumActiveDefaultVPortVlanIds
    pub fn set_num_active_default_vport_vlan_ids(&mut self, value: u32) {
        self.num_active_default_vport_vlan_ids = Some(value);
    }

    /// Gets the value of NumActiveDefaultVPortVlanIds
    pub fn get_num_active_default_vport_vlan_ids(&self) -> Option<&u32> {
        self.num_active_default_vport_vlan_ids.as_ref()
    }

    /// Sets the value of NumActiveNonDefaultVPortMacAddresses
    pub fn set_num_active_non_default_vport_mac_addresses(&mut self, value: u32) {
        self.num_active_non_default_vport_mac_addresses = Some(value);
    }

    /// Gets the value of NumActiveNonDefaultVPortMacAddresses
    pub fn get_num_active_non_default_vport_mac_addresses(&self) -> Option<&u32> {
        self.num_active_non_default_vport_mac_addresses.as_ref()
    }

    /// Sets the value of NumActiveNonDefaultVPortVlanIds
    pub fn set_num_active_non_default_vport_vlan_ids(&mut self, value: u32) {
        self.num_active_non_default_vport_vlan_ids = Some(value);
    }

    /// Gets the value of NumActiveNonDefaultVPortVlanIds
    pub fn get_num_active_non_default_vport_vlan_ids(&self) -> Option<&u32> {
        self.num_active_non_default_vport_vlan_ids.as_ref()
    }

    /// Sets the value of NumActiveVPorts
    pub fn set_num_active_vports(&mut self, value: u32) {
        self.num_active_vports = Some(value);
    }

    /// Gets the value of NumActiveVPorts
    pub fn get_num_active_vports(&self) -> Option<&u32> {
        self.num_active_vports.as_ref()
    }

    /// Sets the value of NumAllocatedVFs
    pub fn set_num_allocated_vfs(&mut self, value: u32) {
        self.num_allocated_vfs = Some(value);
    }

    /// Gets the value of NumAllocatedVFs
    pub fn get_num_allocated_vfs(&self) -> Option<&u32> {
        self.num_allocated_vfs.as_ref()
    }

    /// Sets the value of NumQueuePairsForDefaultVPort
    pub fn set_num_queue_pairs_for_default_vport(&mut self, value: u32) {
        self.num_queue_pairs_for_default_vport = Some(value);
    }

    /// Gets the value of NumQueuePairsForDefaultVPort
    pub fn get_num_queue_pairs_for_default_vport(&self) -> Option<&u32> {
        self.num_queue_pairs_for_default_vport.as_ref()
    }

    /// Sets the value of NumQueuePairsForNonDefaultVPorts
    pub fn set_num_queue_pairs_for_non_default_vports(&mut self, value: u32) {
        self.num_queue_pairs_for_non_default_vports = Some(value);
    }

    /// Gets the value of NumQueuePairsForNonDefaultVPorts
    pub fn get_num_queue_pairs_for_non_default_vports(&self) -> Option<&u32> {
        self.num_queue_pairs_for_non_default_vports.as_ref()
    }

    /// Sets the value of NumVFs
    pub fn set_num_vfs(&mut self, value: u32) {
        self.num_vfs = Some(value);
    }

    /// Gets the value of NumVFs
    pub fn get_num_vfs(&self) -> Option<&u32> {
        self.num_vfs.as_ref()
    }

    /// Sets the value of NumVPorts
    pub fn set_num_vports(&mut self, value: u32) {
        self.num_vports = Some(value);
    }

    /// Gets the value of NumVPorts
    pub fn get_num_vports(&self) -> Option<&u32> {
        self.num_vports.as_ref()
    }

    /// Sets the value of SriovSupport
    pub fn set_sriov_support(&mut self, value: u32) {
        self.sriov_support = Some(value);
    }

    /// Gets the value of SriovSupport
    pub fn get_sriov_support(&self) -> Option<&u32> {
        self.sriov_support.as_ref()
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }

    /// Sets the value of SwitchType
    pub fn set_switch_type(&mut self, value: u16) {
        self.switch_type = Some(value);
    }

    /// Gets the value of SwitchType
    pub fn get_switch_type(&self) -> Option<&u16> {
        self.switch_type.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterSriovSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterSriovSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterSriovSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterSriovSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

