// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortBandwidthData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortBandwidthData {
    #[serde(flatten)]
    pub base: Msvm_EthernetPortData,

/// 
    #[serde(rename = "CurrentBandwidthReservationPercentage")]
    pub current_bandwidth_reservation_percentage: Option<u32>,
}

impl Msvm_EthernetSwitchPortBandwidthData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetPortData::new(),
            current_bandwidth_reservation_percentage: None,
        }
    }


    /// Sets the value of CurrentBandwidthReservationPercentage
    pub fn set_current_bandwidth_reservation_percentage(&mut self, value: u32) {
        self.current_bandwidth_reservation_percentage = Some(value);
    }

    /// Gets the value of CurrentBandwidthReservationPercentage
    pub fn get_current_bandwidth_reservation_percentage(&self) -> Option<&u32> {
        self.current_bandwidth_reservation_percentage.as_ref()
    }
}

impl Msvm_EthernetSwitchPortBandwidthData {
    /// Gets the related Msvm_EthernetSwitchPort object(s)
    pub fn get_related__ethernet_switch_port(&self) -> Result<Msvm_EthernetSwitchPort, WmiError> {
        self.get_related("Msvm_EthernetSwitchPort")
    }

}

