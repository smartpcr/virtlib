// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DeviceErrorCounts struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DeviceErrorCounts {
    #[serde(flatten)]
    pub base: CIM_StatisticalInformation,

/// 
    #[serde(rename = "CriticalErrorCount")]
    pub critical_error_count: Option<u64>,

/// 
    #[serde(rename = "DeviceCreationClassName")]
    pub device_creation_class_name: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "IndeterminateErrorCount")]
    pub indeterminate_error_count: Option<u64>,

/// 
    #[serde(rename = "MajorErrorCount")]
    pub major_error_count: Option<u64>,

/// 
    #[serde(rename = "MinorErrorCount")]
    pub minor_error_count: Option<u64>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,

/// 
    #[serde(rename = "WarningCount")]
    pub warning_count: Option<u64>,
}

impl CIM_DeviceErrorCounts {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StatisticalInformation::new(),
            critical_error_count: None,
            device_creation_class_name: None,
            device_id: None,
            indeterminate_error_count: None,
            major_error_count: None,
            minor_error_count: None,
            system_creation_class_name: None,
            system_name: None,
            warning_count: None,
        }
    }


    /// Sets the value of CriticalErrorCount
    pub fn set_critical_error_count(&mut self, value: u64) {
        self.critical_error_count = Some(value);
    }

    /// Gets the value of CriticalErrorCount
    pub fn get_critical_error_count(&self) -> Option<&u64> {
        self.critical_error_count.as_ref()
    }

    /// Sets the value of DeviceCreationClassName
    pub fn set_device_creation_class_name(&mut self, value: String) {
        self.device_creation_class_name = Some(value);
    }

    /// Gets the value of DeviceCreationClassName
    pub fn get_device_creation_class_name(&self) -> Option<&String> {
        self.device_creation_class_name.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of IndeterminateErrorCount
    pub fn set_indeterminate_error_count(&mut self, value: u64) {
        self.indeterminate_error_count = Some(value);
    }

    /// Gets the value of IndeterminateErrorCount
    pub fn get_indeterminate_error_count(&self) -> Option<&u64> {
        self.indeterminate_error_count.as_ref()
    }

    /// Sets the value of MajorErrorCount
    pub fn set_major_error_count(&mut self, value: u64) {
        self.major_error_count = Some(value);
    }

    /// Gets the value of MajorErrorCount
    pub fn get_major_error_count(&self) -> Option<&u64> {
        self.major_error_count.as_ref()
    }

    /// Sets the value of MinorErrorCount
    pub fn set_minor_error_count(&mut self, value: u64) {
        self.minor_error_count = Some(value);
    }

    /// Gets the value of MinorErrorCount
    pub fn get_minor_error_count(&self) -> Option<&u64> {
        self.minor_error_count.as_ref()
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

    /// Sets the value of WarningCount
    pub fn set_warning_count(&mut self, value: u64) {
        self.warning_count = Some(value);
    }

    /// Gets the value of WarningCount
    pub fn get_warning_count(&self) -> Option<&u64> {
        self.warning_count.as_ref()
    }

/// 

    /// * `selected_counter` -  (u16)

    /// * `return_value` -  (u32)
    pub fn reset_counter(&self, selected_counter: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SelectedCounter".to_string(), value: selected_counter.into() });
        self.invoke_method("ResetCounter", &args)

    }

}

