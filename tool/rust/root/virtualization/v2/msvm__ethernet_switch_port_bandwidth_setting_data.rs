// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortBandwidthSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortBandwidthSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "BurstLimit")]
    pub burst_limit: Option<u64>,

/// 
    #[serde(rename = "BurstSize")]
    pub burst_size: Option<u64>,

/// 
    #[serde(rename = "Limit")]
    pub limit: Option<u64>,

/// 
    #[serde(rename = "Reservation")]
    pub reservation: Option<u64>,

/// 
    #[serde(rename = "Weight")]
    pub weight: Option<u64>,
}

impl Msvm_EthernetSwitchPortBandwidthSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            burst_limit: None,
            burst_size: None,
            limit: None,
            reservation: None,
            weight: None,
        }
    }


    /// Sets the value of BurstLimit
    pub fn set_burst_limit(&mut self, value: u64) {
        self.burst_limit = Some(value);
    }

    /// Gets the value of BurstLimit
    pub fn get_burst_limit(&self) -> Option<&u64> {
        self.burst_limit.as_ref()
    }

    /// Sets the value of BurstSize
    pub fn set_burst_size(&mut self, value: u64) {
        self.burst_size = Some(value);
    }

    /// Gets the value of BurstSize
    pub fn get_burst_size(&self) -> Option<&u64> {
        self.burst_size.as_ref()
    }

    /// Sets the value of Limit
    pub fn set_limit(&mut self, value: u64) {
        self.limit = Some(value);
    }

    /// Gets the value of Limit
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit.as_ref()
    }

    /// Sets the value of Reservation
    pub fn set_reservation(&mut self, value: u64) {
        self.reservation = Some(value);
    }

    /// Gets the value of Reservation
    pub fn get_reservation(&self) -> Option<&u64> {
        self.reservation.as_ref()
    }

    /// Sets the value of Weight
    pub fn set_weight(&mut self, value: u64) {
        self.weight = Some(value);
    }

    /// Gets the value of Weight
    pub fn get_weight(&self) -> Option<&u64> {
        self.weight.as_ref()
    }
}

impl Msvm_EthernetSwitchPortBandwidthSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

