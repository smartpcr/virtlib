// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemReferencePointExportJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemReferencePointExportJob {
    #[serde(flatten)]
    pub base: CIM_ConcreteJob,

/// 
    #[serde(rename = "BaseReferencePointId")]
    pub base_reference_point_id: Option<String>,

/// 
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,

/// 
    #[serde(rename = "ErrorSummaryDescription")]
    pub error_summary_description: Option<String>,

/// 
    #[serde(rename = "ExportDirectory")]
    pub export_directory: Option<String>,

/// 
    #[serde(rename = "ExportedConfigFilePath")]
    pub exported_config_file_path: Option<String>,

/// 
    #[serde(rename = "ExportedDisks")]
    pub exported_disks: Vec<String>,

/// 
    #[serde(rename = "ExportedGuestStateFilePath")]
    pub exported_guest_state_file_path: Option<String>,

/// 
    #[serde(rename = "ExportedLogFilePaths")]
    pub exported_log_file_paths: Vec<String>,

/// 
    #[serde(rename = "ExportedRuntimeFilePath")]
    pub exported_runtime_file_path: Option<String>,

/// 
    #[serde(rename = "ReferencePointId")]
    pub reference_point_id: Option<String>,

/// 
    #[serde(rename = "VirtualMachineId")]
    pub virtual_machine_id: Option<String>,
}

impl Msvm_VirtualSystemReferencePointExportJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ConcreteJob::new(),
            base_reference_point_id: None,
            cancellable: None,
            error_summary_description: None,
            export_directory: None,
            exported_config_file_path: None,
            exported_disks: Vec::new(),
            exported_guest_state_file_path: None,
            exported_log_file_paths: Vec::new(),
            exported_runtime_file_path: None,
            reference_point_id: None,
            virtual_machine_id: None,
        }
    }


    /// Sets the value of BaseReferencePointId
    pub fn set_base_reference_point_id(&mut self, value: String) {
        self.base_reference_point_id = Some(value);
    }

    /// Gets the value of BaseReferencePointId
    pub fn get_base_reference_point_id(&self) -> Option<&String> {
        self.base_reference_point_id.as_ref()
    }

    /// Sets the value of Cancellable
    pub fn set_cancellable(&mut self, value: bool) {
        self.cancellable = Some(value);
    }

    /// Gets the value of Cancellable
    pub fn get_cancellable(&self) -> Option<&bool> {
        self.cancellable.as_ref()
    }

    /// Sets the value of ErrorSummaryDescription
    pub fn set_error_summary_description(&mut self, value: String) {
        self.error_summary_description = Some(value);
    }

    /// Gets the value of ErrorSummaryDescription
    pub fn get_error_summary_description(&self) -> Option<&String> {
        self.error_summary_description.as_ref()
    }

    /// Sets the value of ExportDirectory
    pub fn set_export_directory(&mut self, value: String) {
        self.export_directory = Some(value);
    }

    /// Gets the value of ExportDirectory
    pub fn get_export_directory(&self) -> Option<&String> {
        self.export_directory.as_ref()
    }

    /// Sets the value of ExportedConfigFilePath
    pub fn set_exported_config_file_path(&mut self, value: String) {
        self.exported_config_file_path = Some(value);
    }

    /// Gets the value of ExportedConfigFilePath
    pub fn get_exported_config_file_path(&self) -> Option<&String> {
        self.exported_config_file_path.as_ref()
    }

    /// Sets the value of ExportedDisks
    pub fn set_exported_disks(&mut self, value: Vec<String>) {
        self.exported_disks = value;
    }

    /// Gets the value of ExportedDisks
    pub fn get_exported_disks(&self) -> &Vec<String> {
        &self.exported_disks
    }

    /// Sets the value of ExportedGuestStateFilePath
    pub fn set_exported_guest_state_file_path(&mut self, value: String) {
        self.exported_guest_state_file_path = Some(value);
    }

    /// Gets the value of ExportedGuestStateFilePath
    pub fn get_exported_guest_state_file_path(&self) -> Option<&String> {
        self.exported_guest_state_file_path.as_ref()
    }

    /// Sets the value of ExportedLogFilePaths
    pub fn set_exported_log_file_paths(&mut self, value: Vec<String>) {
        self.exported_log_file_paths = value;
    }

    /// Gets the value of ExportedLogFilePaths
    pub fn get_exported_log_file_paths(&self) -> &Vec<String> {
        &self.exported_log_file_paths
    }

    /// Sets the value of ExportedRuntimeFilePath
    pub fn set_exported_runtime_file_path(&mut self, value: String) {
        self.exported_runtime_file_path = Some(value);
    }

    /// Gets the value of ExportedRuntimeFilePath
    pub fn get_exported_runtime_file_path(&self) -> Option<&String> {
        self.exported_runtime_file_path.as_ref()
    }

    /// Sets the value of ReferencePointId
    pub fn set_reference_point_id(&mut self, value: String) {
        self.reference_point_id = Some(value);
    }

    /// Gets the value of ReferencePointId
    pub fn get_reference_point_id(&self) -> Option<&String> {
        self.reference_point_id.as_ref()
    }

    /// Sets the value of VirtualMachineId
    pub fn set_virtual_machine_id(&mut self, value: String) {
        self.virtual_machine_id = Some(value);
    }

    /// Gets the value of VirtualMachineId
    pub fn get_virtual_machine_id(&self) -> Option<&String> {
        self.virtual_machine_id.as_ref()
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

