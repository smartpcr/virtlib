// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NetworkAdapter {
    #[serde(flatten)]
    pub base: CIM_NetworkAdapter,

/// 
    #[serde(rename = "AdapterType")]
    pub adapter_type: Option<String>,

/// 
    #[serde(rename = "AdapterTypeId")]
    pub adapter_type_id: Option<u16>,

/// 
    #[serde(rename = "GUID")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "Installed")]
    pub installed: Option<bool>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "MACAddress")]
    pub macaddress: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MaxNumberControlled")]
    pub max_number_controlled: Option<u32>,

/// 
    #[serde(rename = "NetConnectionID")]
    pub net_connection_id: Option<String>,

/// 
    #[serde(rename = "NetConnectionStatus")]
    pub net_connection_status: Option<u16>,

/// 
    #[serde(rename = "NetEnabled")]
    pub net_enabled: Option<bool>,

/// 
    #[serde(rename = "PhysicalAdapter")]
    pub physical_adapter: Option<bool>,

/// 
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// 
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

/// 
    #[serde(rename = "TimeOfLastReset")]
    pub time_of_last_reset: Option<String>,
}

impl Win32_NetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NetworkAdapter::new(),
            adapter_type: None,
            adapter_type_id: None,
            guid: None,
            index: None,
            installed: None,
            interface_index: None,
            macaddress: None,
            manufacturer: None,
            max_number_controlled: None,
            net_connection_id: None,
            net_connection_status: None,
            net_enabled: None,
            physical_adapter: None,
            product_name: None,
            service_name: None,
            time_of_last_reset: None,
        }
    }


    /// Sets the value of AdapterType
    pub fn set_adapter_type(&mut self, value: String) {
        self.adapter_type = Some(value);
    }

    /// Gets the value of AdapterType
    pub fn get_adapter_type(&self) -> Option<&String> {
        self.adapter_type.as_ref()
    }

    /// Sets the value of AdapterTypeId
    pub fn set_adapter_type_id(&mut self, value: u16) {
        self.adapter_type_id = Some(value);
    }

    /// Gets the value of AdapterTypeId
    pub fn get_adapter_type_id(&self) -> Option<&u16> {
        self.adapter_type_id.as_ref()
    }

    /// Sets the value of GUID
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of GUID
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of Installed
    pub fn set_installed(&mut self, value: bool) {
        self.installed = Some(value);
    }

    /// Gets the value of Installed
    pub fn get_installed(&self) -> Option<&bool> {
        self.installed.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of MACAddress
    pub fn set_macaddress(&mut self, value: String) {
        self.macaddress = Some(value);
    }

    /// Gets the value of MACAddress
    pub fn get_macaddress(&self) -> Option<&String> {
        self.macaddress.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MaxNumberControlled
    pub fn set_max_number_controlled(&mut self, value: u32) {
        self.max_number_controlled = Some(value);
    }

    /// Gets the value of MaxNumberControlled
    pub fn get_max_number_controlled(&self) -> Option<&u32> {
        self.max_number_controlled.as_ref()
    }

    /// Sets the value of NetConnectionID
    pub fn set_net_connection_id(&mut self, value: String) {
        self.net_connection_id = Some(value);
    }

    /// Gets the value of NetConnectionID
    pub fn get_net_connection_id(&self) -> Option<&String> {
        self.net_connection_id.as_ref()
    }

    /// Sets the value of NetConnectionStatus
    pub fn set_net_connection_status(&mut self, value: u16) {
        self.net_connection_status = Some(value);
    }

    /// Gets the value of NetConnectionStatus
    pub fn get_net_connection_status(&self) -> Option<&u16> {
        self.net_connection_status.as_ref()
    }

    /// Sets the value of NetEnabled
    pub fn set_net_enabled(&mut self, value: bool) {
        self.net_enabled = Some(value);
    }

    /// Gets the value of NetEnabled
    pub fn get_net_enabled(&self) -> Option<&bool> {
        self.net_enabled.as_ref()
    }

    /// Sets the value of PhysicalAdapter
    pub fn set_physical_adapter(&mut self, value: bool) {
        self.physical_adapter = Some(value);
    }

    /// Gets the value of PhysicalAdapter
    pub fn get_physical_adapter(&self) -> Option<&bool> {
        self.physical_adapter.as_ref()
    }

    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    /// Sets the value of TimeOfLastReset
    pub fn set_time_of_last_reset(&mut self, value: String) {
        self.time_of_last_reset = Some(value);
    }

    /// Gets the value of TimeOfLastReset
    pub fn get_time_of_last_reset(&self) -> Option<&String> {
        self.time_of_last_reset.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }

}

