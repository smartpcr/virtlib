// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelThermalConstraintChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelThermalConstraintChange {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Processors")]
    pub processors: Option<u64>,

/// 
    #[serde(rename = "ThermalConstraint")]
    pub thermal_constraint: Option<u32>,
}

impl KernelThermalConstraintChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            instance_name: None,
            processors: None,
            thermal_constraint: None,
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

    /// Sets the value of Processors
    pub fn set_processors(&mut self, value: u64) {
        self.processors = Some(value);
    }

    /// Gets the value of Processors
    pub fn get_processors(&self) -> Option<&u64> {
        self.processors.as_ref()
    }

    /// Sets the value of ThermalConstraint
    pub fn set_thermal_constraint(&mut self, value: u32) {
        self.thermal_constraint = Some(value);
    }

    /// Gets the value of ThermalConstraint
    pub fn get_thermal_constraint(&self) -> Option<&u32> {
        self.thermal_constraint.as_ref()
    }
}

