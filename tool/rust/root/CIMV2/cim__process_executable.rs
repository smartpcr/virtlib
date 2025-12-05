// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProcessExecutable struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProcessExecutable {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// 
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<u64>,

/// 
    #[serde(rename = "GlobalProcessCount")]
    pub global_process_count: Option<u32>,

/// 
    #[serde(rename = "ModuleInstance")]
    pub module_instance: Option<u32>,

/// 
    #[serde(rename = "ProcessCount")]
    pub process_count: Option<u32>,
}

impl CIM_ProcessExecutable {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            base_address: None,
            global_process_count: None,
            module_instance: None,
            process_count: None,
        }
    }


    /// Sets the value of BaseAddress
    pub fn set_base_address(&mut self, value: u64) {
        self.base_address = Some(value);
    }

    /// Gets the value of BaseAddress
    pub fn get_base_address(&self) -> Option<&u64> {
        self.base_address.as_ref()
    }

    /// Sets the value of GlobalProcessCount
    pub fn set_global_process_count(&mut self, value: u32) {
        self.global_process_count = Some(value);
    }

    /// Gets the value of GlobalProcessCount
    pub fn get_global_process_count(&self) -> Option<&u32> {
        self.global_process_count.as_ref()
    }

    /// Sets the value of ModuleInstance
    pub fn set_module_instance(&mut self, value: u32) {
        self.module_instance = Some(value);
    }

    /// Gets the value of ModuleInstance
    pub fn get_module_instance(&self) -> Option<&u32> {
        self.module_instance.as_ref()
    }

    /// Sets the value of ProcessCount
    pub fn set_process_count(&mut self, value: u32) {
        self.process_count = Some(value);
    }

    /// Gets the value of ProcessCount
    pub fn get_process_count(&self) -> Option<&u32> {
        self.process_count.as_ref()
    }
}

