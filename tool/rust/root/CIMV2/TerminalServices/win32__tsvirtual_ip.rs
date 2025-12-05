// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSVirtualIP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSVirtualIP {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "NetworkAdapterDescription")]
    pub network_adapter_description: Option<String>,

/// 
    #[serde(rename = "NetworkAdapterDescriptionList")]
    pub network_adapter_description_list: Vec<String>,

/// 
    #[serde(rename = "NetworkAdapterMacAddress")]
    pub network_adapter_mac_address: Option<String>,

/// 
    #[serde(rename = "NetworkAdapterMacList")]
    pub network_adapter_mac_list: Vec<String>,

/// 
    #[serde(rename = "PolicySourceNetworkAdapter")]
    pub policy_source_network_adapter: Option<u32>,

/// 
    #[serde(rename = "PolicySourceProgramList")]
    pub policy_source_program_list: Option<u32>,

/// 
    #[serde(rename = "PolicySourceVirtualIPActive")]
    pub policy_source_virtual_ipactive: Option<u32>,

/// 
    #[serde(rename = "PolicySourceVirtualIPMode")]
    pub policy_source_virtual_ipmode: Option<u32>,

/// 
    #[serde(rename = "ProgramList")]
    pub program_list: Vec<String>,

/// 
    #[serde(rename = "VirtualIPActive")]
    pub virtual_ipactive: Option<u32>,

/// 
    #[serde(rename = "VirtualIPMode")]
    pub virtual_ipmode: Option<u32>,

/// 
    #[serde(rename = "VirtualizeLoopbackAddressesEnabled")]
    pub virtualize_loopback_addresses_enabled: Option<u32>,
}

impl Win32_TSVirtualIP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            network_adapter_description: None,
            network_adapter_description_list: Vec::new(),
            network_adapter_mac_address: None,
            network_adapter_mac_list: Vec::new(),
            policy_source_network_adapter: None,
            policy_source_program_list: None,
            policy_source_virtual_ipactive: None,
            policy_source_virtual_ipmode: None,
            program_list: Vec::new(),
            virtual_ipactive: None,
            virtual_ipmode: None,
            virtualize_loopback_addresses_enabled: None,
        }
    }


    /// Sets the value of NetworkAdapterDescription
    pub fn set_network_adapter_description(&mut self, value: String) {
        self.network_adapter_description = Some(value);
    }

    /// Gets the value of NetworkAdapterDescription
    pub fn get_network_adapter_description(&self) -> Option<&String> {
        self.network_adapter_description.as_ref()
    }

    /// Sets the value of NetworkAdapterDescriptionList
    pub fn set_network_adapter_description_list(&mut self, value: Vec<String>) {
        self.network_adapter_description_list = value;
    }

    /// Gets the value of NetworkAdapterDescriptionList
    pub fn get_network_adapter_description_list(&self) -> &Vec<String> {
        &self.network_adapter_description_list
    }

    /// Sets the value of NetworkAdapterMacAddress
    pub fn set_network_adapter_mac_address(&mut self, value: String) {
        self.network_adapter_mac_address = Some(value);
    }

    /// Gets the value of NetworkAdapterMacAddress
    pub fn get_network_adapter_mac_address(&self) -> Option<&String> {
        self.network_adapter_mac_address.as_ref()
    }

    /// Sets the value of NetworkAdapterMacList
    pub fn set_network_adapter_mac_list(&mut self, value: Vec<String>) {
        self.network_adapter_mac_list = value;
    }

    /// Gets the value of NetworkAdapterMacList
    pub fn get_network_adapter_mac_list(&self) -> &Vec<String> {
        &self.network_adapter_mac_list
    }

    /// Sets the value of PolicySourceNetworkAdapter
    pub fn set_policy_source_network_adapter(&mut self, value: u32) {
        self.policy_source_network_adapter = Some(value);
    }

    /// Gets the value of PolicySourceNetworkAdapter
    pub fn get_policy_source_network_adapter(&self) -> Option<&u32> {
        self.policy_source_network_adapter.as_ref()
    }

    /// Sets the value of PolicySourceProgramList
    pub fn set_policy_source_program_list(&mut self, value: u32) {
        self.policy_source_program_list = Some(value);
    }

    /// Gets the value of PolicySourceProgramList
    pub fn get_policy_source_program_list(&self) -> Option<&u32> {
        self.policy_source_program_list.as_ref()
    }

    /// Sets the value of PolicySourceVirtualIPActive
    pub fn set_policy_source_virtual_ipactive(&mut self, value: u32) {
        self.policy_source_virtual_ipactive = Some(value);
    }

    /// Gets the value of PolicySourceVirtualIPActive
    pub fn get_policy_source_virtual_ipactive(&self) -> Option<&u32> {
        self.policy_source_virtual_ipactive.as_ref()
    }

    /// Sets the value of PolicySourceVirtualIPMode
    pub fn set_policy_source_virtual_ipmode(&mut self, value: u32) {
        self.policy_source_virtual_ipmode = Some(value);
    }

    /// Gets the value of PolicySourceVirtualIPMode
    pub fn get_policy_source_virtual_ipmode(&self) -> Option<&u32> {
        self.policy_source_virtual_ipmode.as_ref()
    }

    /// Sets the value of ProgramList
    pub fn set_program_list(&mut self, value: Vec<String>) {
        self.program_list = value;
    }

    /// Gets the value of ProgramList
    pub fn get_program_list(&self) -> &Vec<String> {
        &self.program_list
    }

    /// Sets the value of VirtualIPActive
    pub fn set_virtual_ipactive(&mut self, value: u32) {
        self.virtual_ipactive = Some(value);
    }

    /// Gets the value of VirtualIPActive
    pub fn get_virtual_ipactive(&self) -> Option<&u32> {
        self.virtual_ipactive.as_ref()
    }

    /// Sets the value of VirtualIPMode
    pub fn set_virtual_ipmode(&mut self, value: u32) {
        self.virtual_ipmode = Some(value);
    }

    /// Gets the value of VirtualIPMode
    pub fn get_virtual_ipmode(&self) -> Option<&u32> {
        self.virtual_ipmode.as_ref()
    }

    /// Sets the value of VirtualizeLoopbackAddressesEnabled
    pub fn set_virtualize_loopback_addresses_enabled(&mut self, value: u32) {
        self.virtualize_loopback_addresses_enabled = Some(value);
    }

    /// Gets the value of VirtualizeLoopbackAddressesEnabled
    pub fn get_virtualize_loopback_addresses_enabled(&self) -> Option<&u32> {
        self.virtualize_loopback_addresses_enabled.as_ref()
    }

/// 

    /// * `virtual_ipactive` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_virtual_ipactive(&self, virtual_ipactive: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualIPActive".to_string(), value: virtual_ipactive.into() });
        self.invoke_method("SetVirtualIPActive", &args)

    }


/// 

    /// * `virtual_ipmode` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_virtual_ipmode(&self, virtual_ipmode: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualIPMode".to_string(), value: virtual_ipmode.into() });
        self.invoke_method("SetVirtualIPMode", &args)

    }


/// 

    /// * `program_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_program(&self, program_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProgramPath".to_string(), value: program_path.into() });
        self.invoke_method("AddProgram", &args)

    }


/// 

    /// * `program_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_program(&self, program_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProgramPath".to_string(), value: program_path.into() });
        self.invoke_method("RemoveProgram", &args)

    }


/// 

    /// * `program_list` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_program_list(&self, program_list: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProgramList".to_string(), value: program_list.into() });
        self.invoke_method("SetProgramList", &args)

    }


/// 

    /// * `network_adapter_mac_address` -  (String)

    /// * `return_value` -  (u32)
    pub fn select_network_adapter(&self, network_adapter_mac_address: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NetworkAdapterMacAddress".to_string(), value: network_adapter_mac_address.into() });
        self.invoke_method("SelectNetworkAdapter", &args)

    }


/// 

    /// * `virtualize_loopback_addresses_enabled` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_virtualize_loopback_addresses_enabled(&self, virtualize_loopback_addresses_enabled: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualizeLoopbackAddressesEnabled".to_string(), value: virtualize_loopback_addresses_enabled.into() });
        self.invoke_method("SetVirtualizeLoopbackAddressesEnabled", &args)

    }

}

