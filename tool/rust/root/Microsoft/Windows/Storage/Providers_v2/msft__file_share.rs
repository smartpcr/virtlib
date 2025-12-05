// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileShare {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// If TRUE the share is continuously available.
    #[serde(rename = "ContinuouslyAvailable")]
    pub continuously_available: Option<bool>,

/// A user settable description of the file share. This field can be used to store extra free-form information, such as notes or details about the intended usage.
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// If TRUE the share data transmission is encrypted.
    #[serde(rename = "EncryptData")]
    pub encrypt_data: Option<bool>,

/// The file sharing protocol used by the share.
    #[serde(rename = "FileSharingProtocol")]
    pub file_sharing_protocol: Option<FileShare_FileSharingProtocol>,

/// Denotes the current health status of the file share.
///  0 - 'Healthy': TBD.
/// 1 - 'Warning': TBD.
/// 2 - 'Unhealthy': TBD.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<FileShare_HealthStatus>,

/// Name is a semi-unique (scoped to the owning file server), human-readable string used to access and identify a file share.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// An array of values that denote the current operational status of the fileshare.
/// 0 - 'Unknown': The operational status is unknown.
/// 1 - 'Other': A vendor-specific OperationalStatus has been specified by setting the OtherOperationalStatusDescription property.
/// 2 - 'OK': The disk is responding to commands and is in a normal operating state.
/// 3 - 'Degraded': The disk is responding to commands, but is not running in an optimal operating state.
/// 4 - 'Stressed': The disk is functioning, but needs attention. For example, the disk might be overloaded or overheated.
/// 5 - 'Predictive Failure': The disk is functioning, but a failure is likely to occur in the near future.
/// 6 - 'Error': An error has occurred.
/// 7 - 'Non-Recoverable Error': A non-recoverable error has occurred.
/// 8 - 'Starting': The disk is in the process of starting.
/// 9 - 'Stopping': The disk is in the process of stopping.
/// 10 - 'Stopped': The disk was stopped or shut down in a clean and orderly fashion.
/// 11 - 'In Service': The disk is being configured, maintained, cleaned, or otherwise administered.
/// 12 - 'No Contact': The storage provider has knowledge of the disk, but has never been able to establish communication with it.
/// 13 - 'Lost Communication': The storage provider has knowledge of the disk and has contacted it successfully in the past, but the disk is currently unreachable.
/// 14 - 'Aborted': Similar to Stopped, except that the disk stopped abruptly and may require configuration or maintenance.
/// 15 - 'Dormant': The disk is reachable, but it is inactive.
/// 16 - 'Supporting Entity in Error': This status value does not necessarily indicate trouble with the disk, but it does indicate that another device or connection that the disk depends on may need attention.
/// 17 - 'Completed': The disk has completed an operation. This status value should be combined with OK, Error, or Degraded, depending on the outcome of the operation.
/// 0xD010 - 'Online': In Windows-based storage subsystems, this indicates that the object is online.
/// 0xD011 - 'Not Ready': In Windows-based storage subsystems, this indicates that the object is not ready.
/// 0xD012 - 'No Media': In Windows-based storage subsystems, this indicates that the object has no media present.
/// 0xD013 - 'Offline': In Windows-based storage subsystems, this indicates that the object is offline.
/// 0xD014 - 'Failed': In Windows-based storage subsystems, this indicates that the object is in a failed state.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<FileShare_OperationalStatus>,

/// TODO
    #[serde(rename = "ShareState")]
    pub share_state: Option<FileShare_ShareState>,

/// The volume relative path to the directory that is being shared.
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
    pub fn set_file_sharing_protocol(&mut self, value: FileShare_FileSharingProtocol) {
        self.file_sharing_protocol = Some(value);
    }

    /// Gets the value of FileSharingProtocol
    pub fn get_file_sharing_protocol(&self) -> Option<&FileShare_FileSharingProtocol> {
        self.file_sharing_protocol.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: FileShare_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&FileShare_HealthStatus> {
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
    pub fn set_operational_status(&mut self, value: Vec<FileShare_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<FileShare_OperationalStatus> {
        &self.operational_status
    }

    /// Sets the value of ShareState
    pub fn set_share_state(&mut self, value: FileShare_ShareState) {
        self.share_state = Some(value);
    }

    /// Gets the value of ShareState
    pub fn get_share_state(&self) -> Option<&FileShare_ShareState> {
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

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
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

