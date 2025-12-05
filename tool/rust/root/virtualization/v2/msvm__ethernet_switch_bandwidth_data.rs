// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchBandwidthData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchBandwidthData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchData,

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// 
    #[serde(rename = "DefaultFlowReservation")]
    pub default_flow_reservation: Option<u64>,

/// 
    #[serde(rename = "DefaultFlowReservationPercentage")]
    pub default_flow_reservation_percentage: Option<u32>,

/// 
    #[serde(rename = "DefaultFlowWeight")]
    pub default_flow_weight: Option<u64>,

/// 
    #[serde(rename = "Reservation")]
    pub reservation: Option<u64>,
}

impl Msvm_EthernetSwitchBandwidthData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchData::new(),
            capacity: None,
            default_flow_reservation: None,
            default_flow_reservation_percentage: None,
            default_flow_weight: None,
            reservation: None,
        }
    }


    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of DefaultFlowReservation
    pub fn set_default_flow_reservation(&mut self, value: u64) {
        self.default_flow_reservation = Some(value);
    }

    /// Gets the value of DefaultFlowReservation
    pub fn get_default_flow_reservation(&self) -> Option<&u64> {
        self.default_flow_reservation.as_ref()
    }

    /// Sets the value of DefaultFlowReservationPercentage
    pub fn set_default_flow_reservation_percentage(&mut self, value: u32) {
        self.default_flow_reservation_percentage = Some(value);
    }

    /// Gets the value of DefaultFlowReservationPercentage
    pub fn get_default_flow_reservation_percentage(&self) -> Option<&u32> {
        self.default_flow_reservation_percentage.as_ref()
    }

    /// Sets the value of DefaultFlowWeight
    pub fn set_default_flow_weight(&mut self, value: u64) {
        self.default_flow_weight = Some(value);
    }

    /// Gets the value of DefaultFlowWeight
    pub fn get_default_flow_weight(&self) -> Option<&u64> {
        self.default_flow_weight.as_ref()
    }

    /// Sets the value of Reservation
    pub fn set_reservation(&mut self, value: u64) {
        self.reservation = Some(value);
    }

    /// Gets the value of Reservation
    pub fn get_reservation(&self) -> Option<&u64> {
        self.reservation.as_ref()
    }
}

impl Msvm_EthernetSwitchBandwidthData {
    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

}

