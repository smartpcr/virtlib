// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualEthernetSwitchBandwidthSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualEthernetSwitchBandwidthSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchFeatureSettingData,

/// 
    #[serde(rename = "DefaultFlowReservation")]
    pub default_flow_reservation: Option<u64>,

/// 
    #[serde(rename = "DefaultFlowWeight")]
    pub default_flow_weight: Option<u64>,
}

impl Msvm_VirtualEthernetSwitchBandwidthSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchFeatureSettingData::new(),
            default_flow_reservation: None,
            default_flow_weight: None,
        }
    }


    /// Sets the value of DefaultFlowReservation
    pub fn set_default_flow_reservation(&mut self, value: u64) {
        self.default_flow_reservation = Some(value);
    }

    /// Gets the value of DefaultFlowReservation
    pub fn get_default_flow_reservation(&self) -> Option<&u64> {
        self.default_flow_reservation.as_ref()
    }

    /// Sets the value of DefaultFlowWeight
    pub fn set_default_flow_weight(&mut self, value: u64) {
        self.default_flow_weight = Some(value);
    }

    /// Gets the value of DefaultFlowWeight
    pub fn get_default_flow_weight(&self) -> Option<&u64> {
        self.default_flow_weight.as_ref()
    }
}

impl Msvm_VirtualEthernetSwitchBandwidthSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

