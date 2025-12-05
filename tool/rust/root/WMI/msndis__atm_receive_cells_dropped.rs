// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_AtmReceiveCellsDropped struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_AtmReceiveCellsDropped {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NdisAtmReceiveCellsDropped")]
    pub ndis_atm_receive_cells_dropped: Option<u64>,
}

impl MSNdis_AtmReceiveCellsDropped {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            ndis_atm_receive_cells_dropped: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NdisAtmReceiveCellsDropped
    pub fn set_ndis_atm_receive_cells_dropped(&mut self, value: u64) {
        self.ndis_atm_receive_cells_dropped = Some(value);
    }

    /// Gets the value of NdisAtmReceiveCellsDropped
    pub fn get_ndis_atm_receive_cells_dropped(&self) -> Option<&u64> {
        self.ndis_atm_receive_cells_dropped.as_ref()
    }
}

