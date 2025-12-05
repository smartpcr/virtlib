// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualEthernetSwitch struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualEthernetSwitch {
    #[serde(flatten)]
    pub base: CIM_ComputerSystem,

/// 
    #[serde(rename = "MaxIOVOffloads")]
    pub max_iovoffloads: Option<u32>,

/// 
    #[serde(rename = "MaxVMQOffloads")]
    pub max_vmqoffloads: Option<u32>,
}

impl Msvm_VirtualEthernetSwitch {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystem::new(),
            max_iovoffloads: None,
            max_vmqoffloads: None,
        }
    }


    /// Sets the value of MaxIOVOffloads
    pub fn set_max_iovoffloads(&mut self, value: u32) {
        self.max_iovoffloads = Some(value);
    }

    /// Gets the value of MaxIOVOffloads
    pub fn get_max_iovoffloads(&self) -> Option<&u32> {
        self.max_iovoffloads.as_ref()
    }

    /// Sets the value of MaxVMQOffloads
    pub fn set_max_vmqoffloads(&mut self, value: u32) {
        self.max_vmqoffloads = Some(value);
    }

    /// Gets the value of MaxVMQOffloads
    pub fn get_max_vmqoffloads(&self) -> Option<&u32> {
        self.max_vmqoffloads.as_ref()
    }
}

impl Msvm_VirtualEthernetSwitch {
    /// Gets the related Msvm_EthernetSwitchBandwidthData object(s)
    pub fn get_related__ethernet_switch_bandwidth_data(&self) -> Result<Msvm_EthernetSwitchBandwidthData, WmiError> {
        self.get_related("Msvm_EthernetSwitchBandwidthData")
    }

    /// Gets the related Msvm_EthernetSwitchOperationalData object(s)
    pub fn get_related__ethernet_switch_operational_data(&self) -> Result<Msvm_EthernetSwitchOperationalData, WmiError> {
        self.get_related("Msvm_EthernetSwitchOperationalData")
    }

    /// Gets the related Msvm_EthernetSwitchHardwareOffloadData object(s)
    pub fn get_related__ethernet_switch_hardware_offload_data(&self) -> Result<Msvm_EthernetSwitchHardwareOffloadData, WmiError> {
        self.get_related("Msvm_EthernetSwitchHardwareOffloadData")
    }

    /// Gets the related Msvm_TransparentBridgingService object(s)
    pub fn get_related__transparent_bridging_service(&self) -> Result<Msvm_TransparentBridgingService, WmiError> {
        self.get_related("Msvm_TransparentBridgingService")
    }

    /// Gets the related Msvm_EthernetSwitchExtension object(s)
    pub fn get_related__ethernet_switch_extension(&self) -> Result<Vec<Msvm_EthernetSwitchExtension>, WmiError> {
        self.get_all_related("Msvm_EthernetSwitchExtension")
    }

    /// Gets the related Msvm_ResourcePool object(s)
    pub fn get_related__resource_pool(&self) -> Result<Msvm_ResourcePool, WmiError> {
        self.get_related("Msvm_ResourcePool")
    }

    /// Gets the related Msvm_VirtualEthernetSwitchSettingData object(s)
    pub fn get_related__virtual_ethernet_switch_setting_data(&self) -> Result<Msvm_VirtualEthernetSwitchSettingData, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitchSettingData")
    }

    /// Gets the related Msvm_EthernetSwitchPort object(s)
    pub fn get_related__ethernet_switch_port(&self) -> Result<Vec<Msvm_EthernetSwitchPort>, WmiError> {
        self.get_all_related("Msvm_EthernetSwitchPort")
    }

}

