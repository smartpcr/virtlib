// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_VmNetAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_VmNetAdapter {

/// 
    #[serde(rename = "Acls")]
    pub acls: Vec<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<String>,

/// 
    #[serde(rename = "IsDynamicMacAddressEnabled")]
    pub is_dynamic_mac_address_enabled: Option<bool>,

/// 
    #[serde(rename = "IsMacAddressSpoofingAllowed")]
    pub is_mac_address_spoofing_allowed: Option<bool>,

/// 
    #[serde(rename = "IsManagementOS")]
    pub is_management_os: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PhysicalAddress")]
    pub physical_address: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u16>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,

/// 
    #[serde(rename = "VLanId")]
    pub vlan_id: Option<String>,

/// 
    #[serde(rename = "VmId")]
    pub vm_id: Option<String>,

/// 
    #[serde(rename = "VmName")]
    pub vm_name: Option<String>,
}

impl SDDC_VmNetAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            acls: Vec::new(),
            id: None,
            ip_addresses: Vec::new(),
            is_dynamic_mac_address_enabled: None,
            is_mac_address_spoofing_allowed: None,
            is_management_os: None,
            name: None,
            physical_address: None,
            status: None,
            switch_name: None,
            vlan_id: None,
            vm_id: None,
            vm_name: None,
        }
    }


    /// Sets the value of Acls
    pub fn set_acls(&mut self, value: Vec<String>) {
        self.acls = value;
    }

    /// Gets the value of Acls
    pub fn get_acls(&self) -> &Vec<String> {
        &self.acls
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IpAddresses
    pub fn set_ip_addresses(&mut self, value: Vec<String>) {
        self.ip_addresses = value;
    }

    /// Gets the value of IpAddresses
    pub fn get_ip_addresses(&self) -> &Vec<String> {
        &self.ip_addresses
    }

    /// Sets the value of IsDynamicMacAddressEnabled
    pub fn set_is_dynamic_mac_address_enabled(&mut self, value: bool) {
        self.is_dynamic_mac_address_enabled = Some(value);
    }

    /// Gets the value of IsDynamicMacAddressEnabled
    pub fn get_is_dynamic_mac_address_enabled(&self) -> Option<&bool> {
        self.is_dynamic_mac_address_enabled.as_ref()
    }

    /// Sets the value of IsMacAddressSpoofingAllowed
    pub fn set_is_mac_address_spoofing_allowed(&mut self, value: bool) {
        self.is_mac_address_spoofing_allowed = Some(value);
    }

    /// Gets the value of IsMacAddressSpoofingAllowed
    pub fn get_is_mac_address_spoofing_allowed(&self) -> Option<&bool> {
        self.is_mac_address_spoofing_allowed.as_ref()
    }

    /// Sets the value of IsManagementOS
    pub fn set_is_management_os(&mut self, value: bool) {
        self.is_management_os = Some(value);
    }

    /// Gets the value of IsManagementOS
    pub fn get_is_management_os(&self) -> Option<&bool> {
        self.is_management_os.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PhysicalAddress
    pub fn set_physical_address(&mut self, value: String) {
        self.physical_address = Some(value);
    }

    /// Gets the value of PhysicalAddress
    pub fn get_physical_address(&self) -> Option<&String> {
        self.physical_address.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u16) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u16> {
        self.status.as_ref()
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }

    /// Sets the value of VLanId
    pub fn set_vlan_id(&mut self, value: String) {
        self.vlan_id = Some(value);
    }

    /// Gets the value of VLanId
    pub fn get_vlan_id(&self) -> Option<&String> {
        self.vlan_id.as_ref()
    }

    /// Sets the value of VmId
    pub fn set_vm_id(&mut self, value: String) {
        self.vm_id = Some(value);
    }

    /// Gets the value of VmId
    pub fn get_vm_id(&self) -> Option<&String> {
        self.vm_id.as_ref()
    }

    /// Sets the value of VmName
    pub fn set_vm_name(&mut self, value: String) {
        self.vm_name = Some(value);
    }

    /// Gets the value of VmName
    pub fn get_vm_name(&self) -> Option<&String> {
        self.vm_name.as_ref()
    }
}

