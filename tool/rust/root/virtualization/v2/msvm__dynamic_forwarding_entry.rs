// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_DynamicForwardingEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_DynamicForwardingEntry {
    #[serde(flatten)]
    pub base: CIM_DynamicForwardingEntry,

/// 
    #[serde(rename = "VlanId")]
    pub vlan_id: Option<u16>,
}

impl Msvm_DynamicForwardingEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DynamicForwardingEntry::new(),
            vlan_id: None,
        }
    }


    /// Sets the value of VlanId
    pub fn set_vlan_id(&mut self, value: u16) {
        self.vlan_id = Some(value);
    }

    /// Gets the value of VlanId
    pub fn get_vlan_id(&self) -> Option<&u16> {
        self.vlan_id.as_ref()
    }
}

impl Msvm_DynamicForwardingEntry {
    /// Gets the related Msvm_EthernetSwitchPort object(s)
    pub fn get_related__ethernet_switch_port(&self) -> Result<Msvm_EthernetSwitchPort, WmiError> {
        self.get_related("Msvm_EthernetSwitchPort")
    }

    /// Gets the related Msvm_TransparentBridgingService object(s)
    pub fn get_related__transparent_bridging_service(&self) -> Result<Msvm_TransparentBridgingService, WmiError> {
        self.get_related("Msvm_TransparentBridgingService")
    }

}

