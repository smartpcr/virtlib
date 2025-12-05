// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileServer {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "FileSharingProtocols")]
    pub file_sharing_protocols: Vec<u16>,

/// 
    #[serde(rename = "FileSharingProtocolVersions")]
    pub file_sharing_protocol_versions: Vec<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "HostNames")]
    pub host_names: Vec<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// 
    #[serde(rename = "SupportsContinuouslyAvailableFileShare")]
    pub supports_continuously_available_file_share: Option<bool>,

/// 
    #[serde(rename = "SupportsFileShareCreation")]
    pub supports_file_share_creation: Option<bool>,
}

impl MSFT_FileServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            file_sharing_protocols: Vec::new(),
            file_sharing_protocol_versions: Vec::new(),
            friendly_name: None,
            health_status: None,
            host_names: Vec::new(),
            operational_status: Vec::new(),
            other_operational_status_description: None,
            supports_continuously_available_file_share: None,
            supports_file_share_creation: None,
        }
    }


    /// Sets the value of FileSharingProtocols
    pub fn set_file_sharing_protocols(&mut self, value: Vec<u16>) {
        self.file_sharing_protocols = value;
    }

    /// Gets the value of FileSharingProtocols
    pub fn get_file_sharing_protocols(&self) -> &Vec<u16> {
        &self.file_sharing_protocols
    }

    /// Sets the value of FileSharingProtocolVersions
    pub fn set_file_sharing_protocol_versions(&mut self, value: Vec<String>) {
        self.file_sharing_protocol_versions = value;
    }

    /// Gets the value of FileSharingProtocolVersions
    pub fn get_file_sharing_protocol_versions(&self) -> &Vec<String> {
        &self.file_sharing_protocol_versions
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of HostNames
    pub fn set_host_names(&mut self, value: Vec<String>) {
        self.host_names = value;
    }

    /// Gets the value of HostNames
    pub fn get_host_names(&self) -> &Vec<String> {
        &self.host_names
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of OtherOperationalStatusDescription
    pub fn set_other_operational_status_description(&mut self, value: String) {
        self.other_operational_status_description = Some(value);
    }

    /// Gets the value of OtherOperationalStatusDescription
    pub fn get_other_operational_status_description(&self) -> Option<&String> {
        self.other_operational_status_description.as_ref()
    }

    /// Sets the value of SupportsContinuouslyAvailableFileShare
    pub fn set_supports_continuously_available_file_share(&mut self, value: bool) {
        self.supports_continuously_available_file_share = Some(value);
    }

    /// Gets the value of SupportsContinuouslyAvailableFileShare
    pub fn get_supports_continuously_available_file_share(&self) -> Option<&bool> {
        self.supports_continuously_available_file_share.as_ref()
    }

    /// Sets the value of SupportsFileShareCreation
    pub fn set_supports_file_share_creation(&mut self, value: bool) {
        self.supports_file_share_creation = Some(value);
    }

    /// Gets the value of SupportsFileShareCreation
    pub fn get_supports_file_share_creation(&self) -> Option<&bool> {
        self.supports_file_share_creation.as_ref()
    }

/// 

    /// * `continuously_available` -  (bool)
    /// * `description` -  (String)
    /// * `encrypt_data` -  (bool)
    /// * `file_sharing_protocol` -  (u16)
    /// * `name` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `source_volume` -  (MSFT_Volume)
    /// * `volume_relative_path` -  (String)

    /// * `created_file_share` -  (MSFT_FileShare)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_file_share(&self, name: &String, description: &String, source_volume: MSFT_Volume, volume_relative_path: &String, continuously_available: bool, encrypt_data: bool, file_sharing_protocol: u16, run_as_job: bool, created_file_share: &mut MSFT_FileShare, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "SourceVolume".to_string(), value: source_volume.into() });
        args.push(MethodParameter { name: "VolumeRelativePath".to_string(), value: volume_relative_path.into() });
        args.push(MethodParameter { name: "ContinuouslyAvailable".to_string(), value: continuously_available.into() });
        args.push(MethodParameter { name: "EncryptData".to_string(), value: encrypt_data.into() });
        args.push(MethodParameter { name: "FileSharingProtocol".to_string(), value: file_sharing_protocol.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateFileShare", &args)?;
        let created_file_share = result.get_value("CreatedFileShare")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

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

    /// * `friendly_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_friendly_name(&self, friendly_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });

        let result = self.invoke_method("SetFriendlyName", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

