// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPort {
    #[serde(flatten)]
    pub base: CIM_EthernetPort,

/// 
    #[serde(rename = "IOVOffloadUsage")]
    pub iovoffload_usage: Option<u32>,

/// 
    #[serde(rename = "VMQOffloadUsage")]
    pub vmqoffload_usage: Option<u32>,
}

impl Msvm_EthernetSwitchPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EthernetPort::new(),
            iovoffload_usage: None,
            vmqoffload_usage: None,
        }
    }


    /// Sets the value of IOVOffloadUsage
    pub fn set_iovoffload_usage(&mut self, value: u32) {
        self.iovoffload_usage = Some(value);
    }

    /// Gets the value of IOVOffloadUsage
    pub fn get_iovoffload_usage(&self) -> Option<&u32> {
        self.iovoffload_usage.as_ref()
    }

    /// Sets the value of VMQOffloadUsage
    pub fn set_vmqoffload_usage(&mut self, value: u32) {
        self.vmqoffload_usage = Some(value);
    }

    /// Gets the value of VMQOffloadUsage
    pub fn get_vmqoffload_usage(&self) -> Option<&u32> {
        self.vmqoffload_usage.as_ref()
    }
}

impl Msvm_EthernetSwitchPort {
    /// Gets the related Msvm_DynamicForwardingEntry object(s)
    pub fn get_related__dynamic_forwarding_entry(&self) -> Result<Msvm_DynamicForwardingEntry, WmiError> {
        self.get_related("Msvm_DynamicForwardingEntry")
    }

    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

    /// Gets the related Msvm_EthernetSwitchPortOffloadData object(s)
    pub fn get_related__ethernet_switch_port_offload_data(&self) -> Result<Msvm_EthernetSwitchPortOffloadData, WmiError> {
        self.get_related("Msvm_EthernetSwitchPortOffloadData")
    }

    /// Gets the related Msvm_EthernetSwitchPortBandwidthData object(s)
    pub fn get_related__ethernet_switch_port_bandwidth_data(&self) -> Result<Msvm_EthernetSwitchPortBandwidthData, WmiError> {
        self.get_related("Msvm_EthernetSwitchPortBandwidthData")
    }

    /// Gets the related Msvm_LANEndpoint object(s)
    pub fn get_related__lanendpoint(&self) -> Result<Msvm_LANEndpoint, WmiError> {
        self.get_related("Msvm_LANEndpoint")
    }

    /// Gets the related Msvm_EthernetPortAllocationSettingData object(s)
    pub fn get_related__ethernet_port_allocation_setting_data(&self) -> Result<Msvm_EthernetPortAllocationSettingData, WmiError> {
        self.get_related("Msvm_EthernetPortAllocationSettingData")
    }

}

