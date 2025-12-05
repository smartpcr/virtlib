// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSTapeDriveProblemEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSTapeDriveProblemEvent {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DriveProblemType")]
    pub drive_problem_type: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "TapeData")]
    pub tape_data: Vec<u8>,
}

impl MSTapeDriveProblemEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            drive_problem_type: None,
            instance_name: None,
            tape_data: Vec::new(),
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

    /// Sets the value of DriveProblemType
    pub fn set_drive_problem_type(&mut self, value: u32) {
        self.drive_problem_type = Some(value);
    }

    /// Gets the value of DriveProblemType
    pub fn get_drive_problem_type(&self) -> Option<&u32> {
        self.drive_problem_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of TapeData
    pub fn set_tape_data(&mut self, value: Vec<u8>) {
        self.tape_data = value;
    }

    /// Gets the value of TapeData
    pub fn get_tape_data(&self) -> &Vec<u8> {
        &self.tape_data
    }
}

