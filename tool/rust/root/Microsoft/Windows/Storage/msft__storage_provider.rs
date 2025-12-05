// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageProvider {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "CimServerName")]
    pub cim_server_name: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RemoteSubsystemCacheMode")]
    pub remote_subsystem_cache_mode: Option<u16>,

/// 
    #[serde(rename = "SupportedRemoteSubsystemCacheModes")]
    pub supported_remote_subsystem_cache_modes: Vec<u16>,

/// 
    #[serde(rename = "SupportsSubsystemRegistration")]
    pub supports_subsystem_registration: Option<bool>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,

/// 
    #[serde(rename = "URI")]
    pub uri: Option<String>,

/// 
    #[serde(rename = "URI_IP")]
    pub uri__ip: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MSFT_StorageProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            cim_server_name: None,
            manufacturer: None,
            name: None,
            remote_subsystem_cache_mode: None,
            supported_remote_subsystem_cache_modes: Vec::new(),
            supports_subsystem_registration: None,
            type: None,
            uri: None,
            uri__ip: None,
            version: None,
        }
    }


    /// Sets the value of CimServerName
    pub fn set_cim_server_name(&mut self, value: String) {
        self.cim_server_name = Some(value);
    }

    /// Gets the value of CimServerName
    pub fn get_cim_server_name(&self) -> Option<&String> {
        self.cim_server_name.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of RemoteSubsystemCacheMode
    pub fn set_remote_subsystem_cache_mode(&mut self, value: u16) {
        self.remote_subsystem_cache_mode = Some(value);
    }

    /// Gets the value of RemoteSubsystemCacheMode
    pub fn get_remote_subsystem_cache_mode(&self) -> Option<&u16> {
        self.remote_subsystem_cache_mode.as_ref()
    }

    /// Sets the value of SupportedRemoteSubsystemCacheModes
    pub fn set_supported_remote_subsystem_cache_modes(&mut self, value: Vec<u16>) {
        self.supported_remote_subsystem_cache_modes = value;
    }

    /// Gets the value of SupportedRemoteSubsystemCacheModes
    pub fn get_supported_remote_subsystem_cache_modes(&self) -> &Vec<u16> {
        &self.supported_remote_subsystem_cache_modes
    }

    /// Sets the value of SupportsSubsystemRegistration
    pub fn set_supports_subsystem_registration(&mut self, value: bool) {
        self.supports_subsystem_registration = Some(value);
    }

    /// Gets the value of SupportsSubsystemRegistration
    pub fn get_supports_subsystem_registration(&self) -> Option<&bool> {
        self.supports_subsystem_registration.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

    /// Sets the value of URI
    pub fn set_uri(&mut self, value: String) {
        self.uri = Some(value);
    }

    /// Gets the value of URI
    pub fn get_uri(&self) -> Option<&String> {
        self.uri.as_ref()
    }

    /// Sets the value of URI_IP
    pub fn set_uri__ip(&mut self, value: String) {
        self.uri__ip = Some(value);
    }

    /// Gets the value of URI_IP
    pub fn get_uri__ip(&self) -> Option<&String> {
        self.uri__ip.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

/// 

    /// * `discovery_level` -  (u16)
    /// * `root_object` -  (MSFT_StorageObject)
    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn discover(&self, discovery_level: u16, root_object: MSFT_StorageObject, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiscoveryLevel".to_string(), value: discovery_level.into() });
        args.push(MethodParameter { name: "RootObject".to_string(), value: root_object.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Discover", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `security_descriptor` -  (String)
    pub fn get_security_descriptor(&self, security_descriptor: &mut String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let security_descriptor = result.get_value("SecurityDescriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `security_descriptor` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, security_descriptor: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });

        let result = self.invoke_method("SetSecurityDescriptor", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `credential` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `registered_subsystem` -  (MSFT_StorageSubSystem)
    /// * `return_value` -  (u32)
    pub fn register_subsystem(&self, computer_name: &String, credential: &String, registered_subsystem: &mut MSFT_StorageSubSystem, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "Credential".to_string(), value: credential.into() });

        let result = self.invoke_method("RegisterSubsystem", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let registered_subsystem = result.get_value("RegisteredSubsystem")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)
    /// * `storage_sub_system_unique_id` -  (String)
    /// * `subsystem` -  (MSFT_StorageSubSystem)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn unregister_subsystem(&self, subsystem: MSFT_StorageSubSystem, storage_sub_system_unique_id: &String, force: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Subsystem".to_string(), value: subsystem.into() });
        args.push(MethodParameter { name: "StorageSubSystemUniqueId".to_string(), value: storage_sub_system_unique_id.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("UnregisterSubsystem", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `remote_subsystem_cache_mode` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, remote_subsystem_cache_mode: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RemoteSubsystemCacheMode".to_string(), value: remote_subsystem_cache_mode.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

