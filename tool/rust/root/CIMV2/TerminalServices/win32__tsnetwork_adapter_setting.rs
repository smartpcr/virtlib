// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSNetworkAdapterSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSNetworkAdapterSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "DeviceIDList")]
    pub device_idlist: Vec<String>,

/// 
    #[serde(rename = "MaximumConnections")]
    pub maximum_connections: Option<u32>,

/// 
    #[serde(rename = "NetworkAdapterID")]
    pub network_adapter_id: Option<String>,

/// 
    #[serde(rename = "NetworkAdapterLanaID")]
    pub network_adapter_lana_id: Option<u32>,

/// 
    #[serde(rename = "NetworkAdapterList")]
    pub network_adapter_list: Vec<String>,

/// 
    #[serde(rename = "NetworkAdapterName")]
    pub network_adapter_name: Option<String>,

/// 
    #[serde(rename = "PolicySourceMaximumConnections")]
    pub policy_source_maximum_connections: Option<u32>,
}

impl Win32_TSNetworkAdapterSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            device_idlist: Vec::new(),
            maximum_connections: None,
            network_adapter_id: None,
            network_adapter_lana_id: None,
            network_adapter_list: Vec::new(),
            network_adapter_name: None,
            policy_source_maximum_connections: None,
        }
    }


    /// Sets the value of DeviceIDList
    pub fn set_device_idlist(&mut self, value: Vec<String>) {
        self.device_idlist = value;
    }

    /// Gets the value of DeviceIDList
    pub fn get_device_idlist(&self) -> &Vec<String> {
        &self.device_idlist
    }

    /// Sets the value of MaximumConnections
    pub fn set_maximum_connections(&mut self, value: u32) {
        self.maximum_connections = Some(value);
    }

    /// Gets the value of MaximumConnections
    pub fn get_maximum_connections(&self) -> Option<&u32> {
        self.maximum_connections.as_ref()
    }

    /// Sets the value of NetworkAdapterID
    pub fn set_network_adapter_id(&mut self, value: String) {
        self.network_adapter_id = Some(value);
    }

    /// Gets the value of NetworkAdapterID
    pub fn get_network_adapter_id(&self) -> Option<&String> {
        self.network_adapter_id.as_ref()
    }

    /// Sets the value of NetworkAdapterLanaID
    pub fn set_network_adapter_lana_id(&mut self, value: u32) {
        self.network_adapter_lana_id = Some(value);
    }

    /// Gets the value of NetworkAdapterLanaID
    pub fn get_network_adapter_lana_id(&self) -> Option<&u32> {
        self.network_adapter_lana_id.as_ref()
    }

    /// Sets the value of NetworkAdapterList
    pub fn set_network_adapter_list(&mut self, value: Vec<String>) {
        self.network_adapter_list = value;
    }

    /// Gets the value of NetworkAdapterList
    pub fn get_network_adapter_list(&self) -> &Vec<String> {
        &self.network_adapter_list
    }

    /// Sets the value of NetworkAdapterName
    pub fn set_network_adapter_name(&mut self, value: String) {
        self.network_adapter_name = Some(value);
    }

    /// Gets the value of NetworkAdapterName
    pub fn get_network_adapter_name(&self) -> Option<&String> {
        self.network_adapter_name.as_ref()
    }

    /// Sets the value of PolicySourceMaximumConnections
    pub fn set_policy_source_maximum_connections(&mut self, value: u32) {
        self.policy_source_maximum_connections = Some(value);
    }

    /// Gets the value of PolicySourceMaximumConnections
    pub fn get_policy_source_maximum_connections(&self) -> Option<&u32> {
        self.policy_source_maximum_connections.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn select_all_network_adapters(&self) -> Result<(), WmiError> {
        self.invoke_method("SelectAllNetworkAdapters", &[])

    }


/// 

    /// * `network_adapter_lana_id` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_network_adapter_lana_id(&self, network_adapter_lana_id: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NetworkAdapterLanaID".to_string(), value: network_adapter_lana_id.into() });
        self.invoke_method("SetNetworkAdapterLanaID", &args)

    }


/// 

    /// * `network_adapter_ip` -  (String)

    /// * `return_value` -  (u32)
    pub fn select_network_adapter_ip(&self, network_adapter_ip: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NetworkAdapterIP".to_string(), value: network_adapter_ip.into() });
        self.invoke_method("SelectNetworkAdapterIP", &args)

    }

}

