// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionReferencePointExportJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionReferencePointExportJob {
    #[serde(flatten)]
    pub base: CIM_ConcreteJob,

/// 
    #[serde(rename = "BaseReferencePointGroupId")]
    pub base_reference_point_group_id: Option<String>,

/// 
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,

/// 
    #[serde(rename = "CollectionId")]
    pub collection_id: Option<String>,

/// 
    #[serde(rename = "ErrorSummaryDescription")]
    pub error_summary_description: Option<String>,

/// 
    #[serde(rename = "ExportDirectory")]
    pub export_directory: Option<String>,

/// 
    #[serde(rename = "ExportedConfigFilePaths")]
    pub exported_config_file_paths: Vec<String>,

/// 
    #[serde(rename = "ExportedDisks")]
    pub exported_disks: Vec<String>,

/// 
    #[serde(rename = "ExportedGuestStateFilePaths")]
    pub exported_guest_state_file_paths: Vec<String>,

/// 
    #[serde(rename = "ExportedLogFilePaths")]
    pub exported_log_file_paths: Vec<String>,

/// 
    #[serde(rename = "ExportedRuntimeFilePaths")]
    pub exported_runtime_file_paths: Vec<String>,

/// 
    #[serde(rename = "ReferencePointGroupId")]
    pub reference_point_group_id: Option<String>,

/// 
    #[serde(rename = "VirtualMachineId")]
    pub virtual_machine_id: Vec<String>,
}

impl Msvm_CollectionReferencePointExportJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ConcreteJob::new(),
            base_reference_point_group_id: None,
            cancellable: None,
            collection_id: None,
            error_summary_description: None,
            export_directory: None,
            exported_config_file_paths: Vec::new(),
            exported_disks: Vec::new(),
            exported_guest_state_file_paths: Vec::new(),
            exported_log_file_paths: Vec::new(),
            exported_runtime_file_paths: Vec::new(),
            reference_point_group_id: None,
            virtual_machine_id: Vec::new(),
        }
    }


    /// Sets the value of BaseReferencePointGroupId
    pub fn set_base_reference_point_group_id(&mut self, value: String) {
        self.base_reference_point_group_id = Some(value);
    }

    /// Gets the value of BaseReferencePointGroupId
    pub fn get_base_reference_point_group_id(&self) -> Option<&String> {
        self.base_reference_point_group_id.as_ref()
    }

    /// Sets the value of Cancellable
    pub fn set_cancellable(&mut self, value: bool) {
        self.cancellable = Some(value);
    }

    /// Gets the value of Cancellable
    pub fn get_cancellable(&self) -> Option<&bool> {
        self.cancellable.as_ref()
    }

    /// Sets the value of CollectionId
    pub fn set_collection_id(&mut self, value: String) {
        self.collection_id = Some(value);
    }

    /// Gets the value of CollectionId
    pub fn get_collection_id(&self) -> Option<&String> {
        self.collection_id.as_ref()
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

    /// Sets the value of ExportedConfigFilePaths
    pub fn set_exported_config_file_paths(&mut self, value: Vec<String>) {
        self.exported_config_file_paths = value;
    }

    /// Gets the value of ExportedConfigFilePaths
    pub fn get_exported_config_file_paths(&self) -> &Vec<String> {
        &self.exported_config_file_paths
    }

    /// Sets the value of ExportedDisks
    pub fn set_exported_disks(&mut self, value: Vec<String>) {
        self.exported_disks = value;
    }

    /// Gets the value of ExportedDisks
    pub fn get_exported_disks(&self) -> &Vec<String> {
        &self.exported_disks
    }

    /// Sets the value of ExportedGuestStateFilePaths
    pub fn set_exported_guest_state_file_paths(&mut self, value: Vec<String>) {
        self.exported_guest_state_file_paths = value;
    }

    /// Gets the value of ExportedGuestStateFilePaths
    pub fn get_exported_guest_state_file_paths(&self) -> &Vec<String> {
        &self.exported_guest_state_file_paths
    }

    /// Sets the value of ExportedLogFilePaths
    pub fn set_exported_log_file_paths(&mut self, value: Vec<String>) {
        self.exported_log_file_paths = value;
    }

    /// Gets the value of ExportedLogFilePaths
    pub fn get_exported_log_file_paths(&self) -> &Vec<String> {
        &self.exported_log_file_paths
    }

    /// Sets the value of ExportedRuntimeFilePaths
    pub fn set_exported_runtime_file_paths(&mut self, value: Vec<String>) {
        self.exported_runtime_file_paths = value;
    }

    /// Gets the value of ExportedRuntimeFilePaths
    pub fn get_exported_runtime_file_paths(&self) -> &Vec<String> {
        &self.exported_runtime_file_paths
    }

    /// Sets the value of ReferencePointGroupId
    pub fn set_reference_point_group_id(&mut self, value: String) {
        self.reference_point_group_id = Some(value);
    }

    /// Gets the value of ReferencePointGroupId
    pub fn get_reference_point_group_id(&self) -> Option<&String> {
        self.reference_point_group_id.as_ref()
    }

    /// Sets the value of VirtualMachineId
    pub fn set_virtual_machine_id(&mut self, value: Vec<String>) {
        self.virtual_machine_id = value;
    }

    /// Gets the value of VirtualMachineId
    pub fn get_virtual_machine_id(&self) -> &Vec<String> {
        &self.virtual_machine_id
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

