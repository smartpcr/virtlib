// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EmulatedEthernetPortSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EmulatedEthernetPortSettingData {
    #[serde(flatten)]
    pub base: CIM_EthernetPortAllocationSettingData,

/// 
    #[serde(rename = "ClusterMonitored")]
    pub cluster_monitored: Option<bool>,

/// 
    #[serde(rename = "StaticMacAddress")]
    pub static_mac_address: Option<bool>,
}

impl Msvm_EmulatedEthernetPortSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EthernetPortAllocationSettingData::new(),
            cluster_monitored: None,
            static_mac_address: None,
        }
    }


    /// Sets the value of ClusterMonitored
    pub fn set_cluster_monitored(&mut self, value: bool) {
        self.cluster_monitored = Some(value);
    }

    /// Gets the value of ClusterMonitored
    pub fn get_cluster_monitored(&self) -> Option<&bool> {
        self.cluster_monitored.as_ref()
    }

    /// Sets the value of StaticMacAddress
    pub fn set_static_mac_address(&mut self, value: bool) {
        self.static_mac_address = Some(value);
    }

    /// Gets the value of StaticMacAddress
    pub fn get_static_mac_address(&self) -> Option<&bool> {
        self.static_mac_address.as_ref()
    }
}

impl Msvm_EmulatedEthernetPortSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

