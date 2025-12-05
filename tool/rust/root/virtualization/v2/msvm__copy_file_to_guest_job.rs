// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CopyFileToGuestJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CopyFileToGuestJob {
    #[serde(flatten)]
    pub base: CIM_ConcreteJob,

/// 
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,

/// 
    #[serde(rename = "CopyFileToGuestSettingData")]
    pub copy_file_to_guest_setting_data: Vec<String>,

/// 
    #[serde(rename = "ErrorSummaryDescription")]
    pub error_summary_description: Option<String>,

/// 
    #[serde(rename = "VirtualSystemName")]
    pub virtual_system_name: Option<String>,
}

impl Msvm_CopyFileToGuestJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ConcreteJob::new(),
            cancellable: None,
            copy_file_to_guest_setting_data: Vec::new(),
            error_summary_description: None,
            virtual_system_name: None,
        }
    }


    /// Sets the value of Cancellable
    pub fn set_cancellable(&mut self, value: bool) {
        self.cancellable = Some(value);
    }

    /// Gets the value of Cancellable
    pub fn get_cancellable(&self) -> Option<&bool> {
        self.cancellable.as_ref()
    }

    /// Sets the value of CopyFileToGuestSettingData
    pub fn set_copy_file_to_guest_setting_data(&mut self, value: Vec<String>) {
        self.copy_file_to_guest_setting_data = value;
    }

    /// Gets the value of CopyFileToGuestSettingData
    pub fn get_copy_file_to_guest_setting_data(&self) -> &Vec<String> {
        &self.copy_file_to_guest_setting_data
    }

    /// Sets the value of ErrorSummaryDescription
    pub fn set_error_summary_description(&mut self, value: String) {
        self.error_summary_description = Some(value);
    }

    /// Gets the value of ErrorSummaryDescription
    pub fn get_error_summary_description(&self) -> Option<&String> {
        self.error_summary_description.as_ref()
    }

    /// Sets the value of VirtualSystemName
    pub fn set_virtual_system_name(&mut self, value: String) {
        self.virtual_system_name = Some(value);
    }

    /// Gets the value of VirtualSystemName
    pub fn get_virtual_system_name(&self) -> Option<&String> {
        self.virtual_system_name.as_ref()
    }

/// 

    /// * `errors` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_error_ex(&self, errors: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetErrorEx", &[])?;
        let errors = result.get_value("Errors")?;
        Ok(result.return_value)

    }

}

