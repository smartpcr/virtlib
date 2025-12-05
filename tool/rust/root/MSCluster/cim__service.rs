// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Service struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Service {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "Started")]
    pub started: Option<bool>,

/// 
    #[serde(rename = "StartMode")]
    pub start_mode: Option<String>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_Service {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            started: None,
            start_mode: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of Started
    pub fn set_started(&mut self, value: bool) {
        self.started = Some(value);
    }

    /// Gets the value of Started
    pub fn get_started(&self) -> Option<&bool> {
        self.started.as_ref()
    }

    /// Sets the value of StartMode
    pub fn set_start_mode(&mut self, value: String) {
        self.start_mode = Some(value);
    }

    /// Gets the value of StartMode
    pub fn get_start_mode(&self) -> Option<&String> {
        self.start_mode.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

