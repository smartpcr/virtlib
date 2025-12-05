// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortRdmaSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortRdmaSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "RdmaOffloadWeight")]
    pub rdma_offload_weight: Option<u32>,
}

impl Msvm_EthernetSwitchPortRdmaSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            rdma_offload_weight: None,
        }
    }


    /// Sets the value of RdmaOffloadWeight
    pub fn set_rdma_offload_weight(&mut self, value: u32) {
        self.rdma_offload_weight = Some(value);
    }

    /// Gets the value of RdmaOffloadWeight
    pub fn get_rdma_offload_weight(&self) -> Option<&u32> {
        self.rdma_offload_weight.as_ref()
    }
}

impl Msvm_EthernetSwitchPortRdmaSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

