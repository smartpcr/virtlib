// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AssociatedSupplyVoltageSensor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AssociatedSupplyVoltageSensor {
    #[serde(flatten)]
    pub base: CIM_AssociatedSensor,

/// 
    #[serde(rename = "MonitoringRange")]
    pub monitoring_range: Option<u16>,
}

impl CIM_AssociatedSupplyVoltageSensor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_AssociatedSensor::new(),
            monitoring_range: None,
        }
    }


    /// Sets the value of MonitoringRange
    pub fn set_monitoring_range(&mut self, value: u16) {
        self.monitoring_range = Some(value);
    }

    /// Gets the value of MonitoringRange
    pub fn get_monitoring_range(&self) -> Option<&u16> {
        self.monitoring_range.as_ref()
    }
}

