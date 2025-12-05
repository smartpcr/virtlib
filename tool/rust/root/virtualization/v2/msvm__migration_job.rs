// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MigrationJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MigrationJob {
    #[serde(flatten)]
    pub base: CIM_ConcreteJob,

/// 
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,

/// 
    #[serde(rename = "DestinationHost")]
    pub destination_host: Option<String>,

/// 
    #[serde(rename = "ErrorSummaryDescription")]
    pub error_summary_description: Option<String>,

/// 
    #[serde(rename = "JobType")]
    pub job_type: Option<MigrationJob_JobType>,

/// 
    #[serde(rename = "MigrationType")]
    pub migration_type: Option<u16>,

/// 
    #[serde(rename = "NewResourceSettingData")]
    pub new_resource_setting_data: Vec<String>,

/// 
    #[serde(rename = "NewSystemSettingData")]
    pub new_system_setting_data: Option<String>,

/// 
    #[serde(rename = "VirtualSystemName")]
    pub virtual_system_name: Option<String>,
}

impl Msvm_MigrationJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ConcreteJob::new(),
            cancellable: None,
            destination_host: None,
            error_summary_description: None,
            job_type: None,
            migration_type: None,
            new_resource_setting_data: Vec::new(),
            new_system_setting_data: None,
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

    /// Sets the value of DestinationHost
    pub fn set_destination_host(&mut self, value: String) {
        self.destination_host = Some(value);
    }

    /// Gets the value of DestinationHost
    pub fn get_destination_host(&self) -> Option<&String> {
        self.destination_host.as_ref()
    }

    /// Sets the value of ErrorSummaryDescription
    pub fn set_error_summary_description(&mut self, value: String) {
        self.error_summary_description = Some(value);
    }

    /// Gets the value of ErrorSummaryDescription
    pub fn get_error_summary_description(&self) -> Option<&String> {
        self.error_summary_description.as_ref()
    }

    /// Sets the value of JobType
    pub fn set_job_type(&mut self, value: MigrationJob_JobType) {
        self.job_type = Some(value);
    }

    /// Gets the value of JobType
    pub fn get_job_type(&self) -> Option<&MigrationJob_JobType> {
        self.job_type.as_ref()
    }

    /// Sets the value of MigrationType
    pub fn set_migration_type(&mut self, value: u16) {
        self.migration_type = Some(value);
    }

    /// Gets the value of MigrationType
    pub fn get_migration_type(&self) -> Option<&u16> {
        self.migration_type.as_ref()
    }

    /// Sets the value of NewResourceSettingData
    pub fn set_new_resource_setting_data(&mut self, value: Vec<String>) {
        self.new_resource_setting_data = value;
    }

    /// Gets the value of NewResourceSettingData
    pub fn get_new_resource_setting_data(&self) -> &Vec<String> {
        &self.new_resource_setting_data
    }

    /// Sets the value of NewSystemSettingData
    pub fn set_new_system_setting_data(&mut self, value: String) {
        self.new_system_setting_data = Some(value);
    }

    /// Gets the value of NewSystemSettingData
    pub fn get_new_system_setting_data(&self) -> Option<&String> {
        self.new_system_setting_data.as_ref()
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

