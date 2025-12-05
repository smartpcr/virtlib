// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorAcpiTsd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorAcpiTsd {
    #[serde(flatten)]
    pub base: MSProcessorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "Dependency")]
    pub dependency: Vec<ProcessorAcpiTsdDependency>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl ProcessorAcpiTsd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSProcessorClass::new(),
            active: None,
            count: None,
            dependency: Vec::new(),
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

    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of Dependency
    pub fn set_dependency(&mut self, value: Vec<ProcessorAcpiTsdDependency>) {
        self.dependency = value;
    }

    /// Gets the value of Dependency
    pub fn get_dependency(&self) -> &Vec<ProcessorAcpiTsdDependency> {
        &self.dependency
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

