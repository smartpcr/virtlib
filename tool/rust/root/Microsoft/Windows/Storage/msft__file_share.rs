// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileShare {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "ContinuouslyAvailable")]
    pub continuously_available: Option<bool>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "EncryptData")]
    pub encrypt_data: Option<bool>,

/// 
    #[serde(rename = "FileSharingProtocol")]
    pub file_sharing_protocol: Option<u16>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "ShareState")]
    pub share_state: Option<u16>,

/// 
    #[serde(rename = "VolumeRelativePath")]
    pub volume_relative_path: Option<String>,
}

impl MSFT_FileShare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            continuously_available: None,
            description: None,
            encrypt_data: None,
            file_sharing_protocol: None,
            health_status: None,
            name: None,
            operational_status: Vec::new(),
            share_state: None,
            volume_relative_path: None,
        }
    }


    /// Sets the value of ContinuouslyAvailable
    pub fn set_continuously_available(&mut self, value: bool) {
        self.continuously_available = Some(value);
    }

    /// Gets the value of ContinuouslyAvailable
    pub fn get_continuously_available(&self) -> Option<&bool> {
        self.continuously_available.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of EncryptData
    pub fn set_encrypt_data(&mut self, value: bool) {
        self.encrypt_data = Some(value);
    }

    /// Gets the value of EncryptData
    pub fn get_encrypt_data(&self) -> Option<&bool> {
        self.encrypt_data.as_ref()
    }

    /// Sets the value of FileSharingProtocol
    pub fn set_file_sharing_protocol(&mut self, value: u16) {
        self.file_sharing_protocol = Some(value);
    }

    /// Gets the value of FileSharingProtocol
    pub fn get_file_sharing_protocol(&self) -> Option<&u16> {
        self.file_sharing_protocol.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of ShareState
    pub fn set_share_state(&mut self, value: u16) {
        self.share_state = Some(value);
    }

    /// Gets the value of ShareState
    pub fn get_share_state(&self) -> Option<&u16> {
        self.share_state.as_ref()
    }

    /// Sets the value of VolumeRelativePath
    pub fn set_volume_relative_path(&mut self, value: String) {
        self.volume_relative_path = Some(value);
    }

    /// Gets the value of VolumeRelativePath
    pub fn get_volume_relative_path(&self) -> Option<&String> {
        self.volume_relative_path.as_ref()
    }

/// 

    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("DeleteObject", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_description(&self, description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("SetDescription", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `encrypt_data` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, encrypt_data: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EncryptData".to_string(), value: encrypt_data.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_control_entries` -  (MSFT_FileShareAccessControlEntry[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_access_control_entries(&self, access_control_entries: &mut Vec<MSFT_FileShareAccessControlEntry>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAccessControlEntries", &[])?;
        let access_control_entries = result.get_value("AccessControlEntries")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_right` -  (u32)
    /// * `account_names` -  (String[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn grant_access(&self, account_names: &Vec<String>, access_right: u32, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountNames".to_string(), value: account_names.into() });
        args.push(MethodParameter { name: "AccessRight".to_string(), value: access_right.into() });

        let result = self.invoke_method("GrantAccess", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_names` -  (String[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn revoke_access(&self, account_names: &Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountNames".to_string(), value: account_names.into() });

        let result = self.invoke_method("RevokeAccess", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_names` -  (String[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn block_access(&self, account_names: &Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountNames".to_string(), value: account_names.into() });

        let result = self.invoke_method("BlockAccess", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_names` -  (String[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn unblock_access(&self, account_names: &Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountNames".to_string(), value: account_names.into() });

        let result = self.invoke_method("UnblockAccess", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `diagnose_results` -  (MSFT_StorageDiagnoseResult[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn diagnose(&self, diagnose_results: &mut Vec<MSFT_StorageDiagnoseResult>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Diagnose", &[])?;
        let diagnose_results = result.get_value("DiagnoseResults")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `action_results` -  (MSFT_HealthAction[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_actions(&self, action_results: &mut Vec<MSFT_HealthAction>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetActions", &[])?;
        let action_results = result.get_value("ActionResults")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

