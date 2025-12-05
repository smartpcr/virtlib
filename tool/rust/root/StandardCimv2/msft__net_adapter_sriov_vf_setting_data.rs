// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterSriovVfSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterSriovVfSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "CurrentMacAddress")]
    pub current_mac_address: Option<String>,

/// 
    #[serde(rename = "FunctionID")]
    pub function_id: Option<u16>,

/// 
    #[serde(rename = "PermanentMacAddress")]
    pub permanent_mac_address: Option<String>,

/// 
    #[serde(rename = "SwitchID")]
    pub switch_id: Option<u32>,

/// 
    #[serde(rename = "VmFriendlyName")]
    pub vm_friendly_name: Option<String>,

/// 
    #[serde(rename = "VmID")]
    pub vm_id: Option<String>,

/// 
    #[serde(rename = "VmNicID")]
    pub vm_nic_id: Option<String>,

/// 
    #[serde(rename = "VPortID")]
    pub vport_id: Vec<u32>,
}

impl MSFT_NetAdapterSriovVfSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            current_mac_address: None,
            function_id: None,
            permanent_mac_address: None,
            switch_id: None,
            vm_friendly_name: None,
            vm_id: None,
            vm_nic_id: None,
            vport_id: Vec::new(),
        }
    }


    /// Sets the value of CurrentMacAddress
    pub fn set_current_mac_address(&mut self, value: String) {
        self.current_mac_address = Some(value);
    }

    /// Gets the value of CurrentMacAddress
    pub fn get_current_mac_address(&self) -> Option<&String> {
        self.current_mac_address.as_ref()
    }

    /// Sets the value of FunctionID
    pub fn set_function_id(&mut self, value: u16) {
        self.function_id = Some(value);
    }

    /// Gets the value of FunctionID
    pub fn get_function_id(&self) -> Option<&u16> {
        self.function_id.as_ref()
    }

    /// Sets the value of PermanentMacAddress
    pub fn set_permanent_mac_address(&mut self, value: String) {
        self.permanent_mac_address = Some(value);
    }

    /// Gets the value of PermanentMacAddress
    pub fn get_permanent_mac_address(&self) -> Option<&String> {
        self.permanent_mac_address.as_ref()
    }

    /// Sets the value of SwitchID
    pub fn set_switch_id(&mut self, value: u32) {
        self.switch_id = Some(value);
    }

    /// Gets the value of SwitchID
    pub fn get_switch_id(&self) -> Option<&u32> {
        self.switch_id.as_ref()
    }

    /// Sets the value of VmFriendlyName
    pub fn set_vm_friendly_name(&mut self, value: String) {
        self.vm_friendly_name = Some(value);
    }

    /// Gets the value of VmFriendlyName
    pub fn get_vm_friendly_name(&self) -> Option<&String> {
        self.vm_friendly_name.as_ref()
    }

    /// Sets the value of VmID
    pub fn set_vm_id(&mut self, value: String) {
        self.vm_id = Some(value);
    }

    /// Gets the value of VmID
    pub fn get_vm_id(&self) -> Option<&String> {
        self.vm_id.as_ref()
    }

    /// Sets the value of VmNicID
    pub fn set_vm_nic_id(&mut self, value: String) {
        self.vm_nic_id = Some(value);
    }

    /// Gets the value of VmNicID
    pub fn get_vm_nic_id(&self) -> Option<&String> {
        self.vm_nic_id.as_ref()
    }

    /// Sets the value of VPortID
    pub fn set_vport_id(&mut self, value: Vec<u32>) {
        self.vport_id = value;
    }

    /// Gets the value of VPortID
    pub fn get_vport_id(&self) -> &Vec<u32> {
        &self.vport_id
    }
}

