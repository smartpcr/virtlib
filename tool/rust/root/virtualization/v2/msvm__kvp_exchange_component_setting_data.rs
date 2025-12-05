// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_KvpExchangeComponentSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_KvpExchangeComponentSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "DisableHostKVPItems")]
    pub disable_host_kvpitems: Option<bool>,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u16>,

/// 
    #[serde(rename = "HostExchangeItems")]
    pub host_exchange_items: Vec<String>,

/// 
    #[serde(rename = "HostOnlyItems")]
    pub host_only_items: Vec<String>,
}

impl Msvm_KvpExchangeComponentSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            disable_host_kvpitems: None,
            enabled_state: None,
            host_exchange_items: Vec::new(),
            host_only_items: Vec::new(),
        }
    }


    /// Sets the value of DisableHostKVPItems
    pub fn set_disable_host_kvpitems(&mut self, value: bool) {
        self.disable_host_kvpitems = Some(value);
    }

    /// Gets the value of DisableHostKVPItems
    pub fn get_disable_host_kvpitems(&self) -> Option<&bool> {
        self.disable_host_kvpitems.as_ref()
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u16) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u16> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of HostExchangeItems
    pub fn set_host_exchange_items(&mut self, value: Vec<String>) {
        self.host_exchange_items = value;
    }

    /// Gets the value of HostExchangeItems
    pub fn get_host_exchange_items(&self) -> &Vec<String> {
        &self.host_exchange_items
    }

    /// Sets the value of HostOnlyItems
    pub fn set_host_only_items(&mut self, value: Vec<String>) {
        self.host_only_items = value;
    }

    /// Gets the value of HostOnlyItems
    pub fn get_host_only_items(&self) -> &Vec<String> {
        &self.host_only_items
    }
}

impl Msvm_KvpExchangeComponentSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

