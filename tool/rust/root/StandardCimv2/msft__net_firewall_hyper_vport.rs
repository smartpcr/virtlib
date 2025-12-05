// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallHyperVPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallHyperVPort {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Constrained")]
    pub constrained: Option<u16>,

/// 
    #[serde(rename = "InterfaceGuid")]
    pub interface_guid: Option<String>,

/// 
    #[serde(rename = "NetworkType")]
    pub network_type: Option<u16>,

/// 
    #[serde(rename = "PartitionGuid")]
    pub partition_guid: Option<String>,

/// 
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<u16>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,

/// 
    #[serde(rename = "VMCreatorId")]
    pub vmcreator_id: Option<String>,
}

impl MSFT_NetFirewallHyperVPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            constrained: None,
            interface_guid: None,
            network_type: None,
            partition_guid: None,
            port_name: None,
            profile: None,
            switch_name: None,
            vmcreator_id: None,
        }
    }


    /// Sets the value of Constrained
    pub fn set_constrained(&mut self, value: u16) {
        self.constrained = Some(value);
    }

    /// Gets the value of Constrained
    pub fn get_constrained(&self) -> Option<&u16> {
        self.constrained.as_ref()
    }

    /// Sets the value of InterfaceGuid
    pub fn set_interface_guid(&mut self, value: String) {
        self.interface_guid = Some(value);
    }

    /// Gets the value of InterfaceGuid
    pub fn get_interface_guid(&self) -> Option<&String> {
        self.interface_guid.as_ref()
    }

    /// Sets the value of NetworkType
    pub fn set_network_type(&mut self, value: u16) {
        self.network_type = Some(value);
    }

    /// Gets the value of NetworkType
    pub fn get_network_type(&self) -> Option<&u16> {
        self.network_type.as_ref()
    }

    /// Sets the value of PartitionGuid
    pub fn set_partition_guid(&mut self, value: String) {
        self.partition_guid = Some(value);
    }

    /// Gets the value of PartitionGuid
    pub fn get_partition_guid(&self) -> Option<&String> {
        self.partition_guid.as_ref()
    }

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: u16) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&u16> {
        self.profile.as_ref()
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }

    /// Sets the value of VMCreatorId
    pub fn set_vmcreator_id(&mut self, value: String) {
        self.vmcreator_id = Some(value);
    }

    /// Gets the value of VMCreatorId
    pub fn get_vmcreator_id(&self) -> Option<&String> {
        self.vmcreator_id.as_ref()
    }
}

