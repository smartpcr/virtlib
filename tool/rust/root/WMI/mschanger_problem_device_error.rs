// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSChangerProblemDeviceError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSChangerProblemDeviceError {
    #[serde(flatten)]
    pub base: MSChangerDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "ChangerProblemType")]
    pub changer_problem_type: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSChangerProblemDeviceError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSChangerDriver::new(),
            active: None,
            changer_problem_type: None,
            instance_name: None,
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

    /// Sets the value of ChangerProblemType
    pub fn set_changer_problem_type(&mut self, value: u32) {
        self.changer_problem_type = Some(value);
    }

    /// Gets the value of ChangerProblemType
    pub fn get_changer_problem_type(&self) -> Option<&u32> {
        self.changer_problem_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }
}

