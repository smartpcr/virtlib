// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Thread struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Thread {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "ExecutionState")]
    pub execution_state: Option<u16>,

/// 
    #[serde(rename = "Handle")]
    pub handle: Option<String>,

/// 
    #[serde(rename = "KernelModeTime")]
    pub kernel_mode_time: Option<u64>,

/// 
    #[serde(rename = "OSCreationClassName")]
    pub oscreation_class_name: Option<String>,

/// 
    #[serde(rename = "OSName")]
    pub osname: Option<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "ProcessCreationClassName")]
    pub process_creation_class_name: Option<String>,

/// 
    #[serde(rename = "ProcessHandle")]
    pub process_handle: Option<String>,

/// 
    #[serde(rename = "UserModeTime")]
    pub user_mode_time: Option<u64>,
}

impl CIM_Thread {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            cscreation_class_name: None,
            csname: None,
            execution_state: None,
            handle: None,
            kernel_mode_time: None,
            oscreation_class_name: None,
            osname: None,
            priority: None,
            process_creation_class_name: None,
            process_handle: None,
            user_mode_time: None,
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

    /// Sets the value of CSCreationClassName
    pub fn set_cscreation_class_name(&mut self, value: String) {
        self.cscreation_class_name = Some(value);
    }

    /// Gets the value of CSCreationClassName
    pub fn get_cscreation_class_name(&self) -> Option<&String> {
        self.cscreation_class_name.as_ref()
    }

    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of ExecutionState
    pub fn set_execution_state(&mut self, value: u16) {
        self.execution_state = Some(value);
    }

    /// Gets the value of ExecutionState
    pub fn get_execution_state(&self) -> Option<&u16> {
        self.execution_state.as_ref()
    }

    /// Sets the value of Handle
    pub fn set_handle(&mut self, value: String) {
        self.handle = Some(value);
    }

    /// Gets the value of Handle
    pub fn get_handle(&self) -> Option<&String> {
        self.handle.as_ref()
    }

    /// Sets the value of KernelModeTime
    pub fn set_kernel_mode_time(&mut self, value: u64) {
        self.kernel_mode_time = Some(value);
    }

    /// Gets the value of KernelModeTime
    pub fn get_kernel_mode_time(&self) -> Option<&u64> {
        self.kernel_mode_time.as_ref()
    }

    /// Sets the value of OSCreationClassName
    pub fn set_oscreation_class_name(&mut self, value: String) {
        self.oscreation_class_name = Some(value);
    }

    /// Gets the value of OSCreationClassName
    pub fn get_oscreation_class_name(&self) -> Option<&String> {
        self.oscreation_class_name.as_ref()
    }

    /// Sets the value of OSName
    pub fn set_osname(&mut self, value: String) {
        self.osname = Some(value);
    }

    /// Gets the value of OSName
    pub fn get_osname(&self) -> Option<&String> {
        self.osname.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of ProcessCreationClassName
    pub fn set_process_creation_class_name(&mut self, value: String) {
        self.process_creation_class_name = Some(value);
    }

    /// Gets the value of ProcessCreationClassName
    pub fn get_process_creation_class_name(&self) -> Option<&String> {
        self.process_creation_class_name.as_ref()
    }

    /// Sets the value of ProcessHandle
    pub fn set_process_handle(&mut self, value: String) {
        self.process_handle = Some(value);
    }

    /// Gets the value of ProcessHandle
    pub fn get_process_handle(&self) -> Option<&String> {
        self.process_handle.as_ref()
    }

    /// Sets the value of UserModeTime
    pub fn set_user_mode_time(&mut self, value: u64) {
        self.user_mode_time = Some(value);
    }

    /// Gets the value of UserModeTime
    pub fn get_user_mode_time(&self) -> Option<&u64> {
        self.user_mode_time.as_ref()
    }
}

