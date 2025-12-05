// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortAclSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortAclSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "AclType")]
    pub acl_type: Option<u8>,

/// 
    #[serde(rename = "Action")]
    pub action: Option<u8>,

/// 
    #[serde(rename = "Applicability")]
    pub applicability: Option<u8>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<u8>,

/// 
    #[serde(rename = "LocalAddress")]
    pub local_address: Option<String>,

/// 
    #[serde(rename = "LocalAddressPrefixLength")]
    pub local_address_prefix_length: Option<u8>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RemoteAddress")]
    pub remote_address: Option<String>,

/// 
    #[serde(rename = "RemoteAddressPrefixLength")]
    pub remote_address_prefix_length: Option<u8>,
}

impl Msvm_EthernetSwitchPortAclSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            acl_type: None,
            action: None,
            applicability: None,
            direction: None,
            local_address: None,
            local_address_prefix_length: None,
            name: None,
            remote_address: None,
            remote_address_prefix_length: None,
        }
    }


    /// Sets the value of AclType
    pub fn set_acl_type(&mut self, value: u8) {
        self.acl_type = Some(value);
    }

    /// Gets the value of AclType
    pub fn get_acl_type(&self) -> Option<&u8> {
        self.acl_type.as_ref()
    }

    /// Sets the value of Action
    pub fn set_action(&mut self, value: u8) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&u8> {
        self.action.as_ref()
    }

    /// Sets the value of Applicability
    pub fn set_applicability(&mut self, value: u8) {
        self.applicability = Some(value);
    }

    /// Gets the value of Applicability
    pub fn get_applicability(&self) -> Option<&u8> {
        self.applicability.as_ref()
    }

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: u8) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&u8> {
        self.direction.as_ref()
    }

    /// Sets the value of LocalAddress
    pub fn set_local_address(&mut self, value: String) {
        self.local_address = Some(value);
    }

    /// Gets the value of LocalAddress
    pub fn get_local_address(&self) -> Option<&String> {
        self.local_address.as_ref()
    }

    /// Sets the value of LocalAddressPrefixLength
    pub fn set_local_address_prefix_length(&mut self, value: u8) {
        self.local_address_prefix_length = Some(value);
    }

    /// Gets the value of LocalAddressPrefixLength
    pub fn get_local_address_prefix_length(&self) -> Option<&u8> {
        self.local_address_prefix_length.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of RemoteAddress
    pub fn set_remote_address(&mut self, value: String) {
        self.remote_address = Some(value);
    }

    /// Gets the value of RemoteAddress
    pub fn get_remote_address(&self) -> Option<&String> {
        self.remote_address.as_ref()
    }

    /// Sets the value of RemoteAddressPrefixLength
    pub fn set_remote_address_prefix_length(&mut self, value: u8) {
        self.remote_address_prefix_length = Some(value);
    }

    /// Gets the value of RemoteAddressPrefixLength
    pub fn get_remote_address_prefix_length(&self) -> Option<&u8> {
        self.remote_address_prefix_length.as_ref()
    }
}

impl Msvm_EthernetSwitchPortAclSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

