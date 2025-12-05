// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageProvider {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// A string indicating the manufacturer of the SMP software.
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// A user-friendly name of the storage provider.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RemoteSubsystemCacheMode")]
    pub remote_subsystem_cache_mode: Option<StorageProvider_RemoteSubsystemCacheMode>,

/// Denotes the caching modes this provider supports. The modes are 'Disabled' and 'Manual-Discovery'.
    #[serde(rename = "SupportedRemoteSubsystemCacheModes")]
    pub supported_remote_subsystem_cache_modes: Vec<StorageProvider_SupportedRemoteSubsystemCacheModes>,

/// Denotes whether this provider supports remote registration and management.
    #[serde(rename = "SupportsSubsystemRegistration")]
    pub supports_subsystem_registration: Option<bool>,

/// Denotes whether the provider is a stand-alone SMP provider or an SMIS provider that uses the SMIS proxy SMP.
    #[serde(rename = "Type")]
    pub type: Option<StorageProvider_Type>,

/// A version string used by the SMP manufacturer to differentiate between software versions.
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MSFT_StorageProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            manufacturer: None,
            name: None,
            remote_subsystem_cache_mode: None,
            supported_remote_subsystem_cache_modes: Vec::new(),
            supports_subsystem_registration: None,
            type: None,
            version: None,
        }
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
    pub fn set_remote_subsystem_cache_mode(&mut self, value: StorageProvider_RemoteSubsystemCacheMode) {
        self.remote_subsystem_cache_mode = Some(value);
    }

    /// Gets the value of RemoteSubsystemCacheMode
    pub fn get_remote_subsystem_cache_mode(&self) -> Option<&StorageProvider_RemoteSubsystemCacheMode> {
        self.remote_subsystem_cache_mode.as_ref()
    }

    /// Sets the value of SupportedRemoteSubsystemCacheModes
    pub fn set_supported_remote_subsystem_cache_modes(&mut self, value: Vec<StorageProvider_SupportedRemoteSubsystemCacheModes>) {
        self.supported_remote_subsystem_cache_modes = value;
    }

    /// Gets the value of SupportedRemoteSubsystemCacheModes
    pub fn get_supported_remote_subsystem_cache_modes(&self) -> &Vec<StorageProvider_SupportedRemoteSubsystemCacheModes> {
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
    pub fn set_type(&mut self, value: StorageProvider_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&StorageProvider_Type> {
        self.type.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

/// This method is used when a user wants to explicitly discover or re-enumerate objects owned by the storage provider. A call to this method could result in a full or partial cache invalidation and over-the-wire calls to the storage subsystem(s) in order to discover new or updated objects. As this is a time consuming task, it should be used sparingly by the user.
/// The scope of the discovery operation is controlled by the DiscoveryLevel and RootObject parameters. DiscoveryLevel controls the depth of the object discovery. RootObject defines the starting point from which discovery will happen.

    /// * `discovery_level` - This field denotes the level (or depth) of discovery that should be performed. This parameter can only be specified if the root object is a storage provider, storage subsystem, or NULL. When specified, the storage provider will discover objects starting from Level 0 and continuing until the specified level is reached. Associations between objects (within the discovered levels) will also be discovered. The discovery levels are defined as follows:  0 - 'Level 0': The storage provider and storage subsystem objects will be discovered.  1 - 'Level 1': Storage pools, resiliency settings, target ports, target portals, and initiator ids will be discovered. 2 - 'Level 2': Virtual disks and masking sets will be discovered. 3 - 'Level 3': Physical disks will be discovered. (StorageProvider_DiscoveryLevel)
    /// * `root_object` - If this parameter is set, discovery will begin from this object. When DiscoveryLevel is NULL, well-defined actions will be taken depending on the type of object specified by RootObject:  Storage subsystem: All associated objects will be discovered.  Storage pool: The pool, along with any associated resiliency settings, virtual disks, and physical disks will be discovered.  Masking set: The masking set, along with any associated target ports, initiator ids, and virtual disks will be discovered.  For all other objects: Only that object will be discovered / refreshed. (MSFT_StorageObject)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn discover(&self, discovery_level: StorageProvider_DiscoveryLevel, root_object: MSFT_StorageObject, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiscoveryLevel".to_string(), value: discovery_level.into() });
        args.push(MethodParameter { name: "RootObject".to_string(), value: root_object.into() });

        let result = self.invoke_method("Discover", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method returns the security descriptor that controls access to this specific object instance.

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `security_descriptor` - A Security Descriptor Definition Language (SDDL) formed string describing the access control list of the object. (String)
    pub fn get_security_descriptor(&self, security_descriptor: &mut String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let security_descriptor = result.get_value("SecurityDescriptor")?;
        Ok(result.return_value)

    }


/// This method allows a user with sufficient privileges to set the security descriptor that control access to this specific object instance. If the call is not made in the context of a user specified in the security descriptor's access control list, this method will fail with 40001 - 'Access Denied'. If an empty security descriptor is passed to this function, the behavior is left to the specific implementation so long as there is some user context (typically domain administrators) that can access and administer the object.

    /// * `security_descriptor` - A Security Descriptor Definition Language (SDDL) formed string describing the desired access control list for this object. (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, security_descriptor: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });

        let result = self.invoke_method("SetSecurityDescriptor", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method registers a subsystem to be managed by this provider. Note that the subsystem must be compatible with the provider software.

    /// * `computer_name` -  (String)
    /// * `credential` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
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


/// This method unregisters a subsystem. The provider will no longer manage this subsystem.

    /// * `force` -  (bool)
    /// * `storage_sub_system_unique_id` -  (String)
    /// * `subsystem` -  (MSFT_StorageSubSystem)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
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


/// This method sets the attributes of the provider.

    /// * `remote_subsystem_cache_mode` - If set to 3, caching for all the registered remote subsystem is enabled. If set to 2, caching for all the registered remote subsystem is disabled. This API only effects the remote subsystem registered and local Subsystem requests are not cached and reported live always. (StorageProvider_RemoteSubsystemCacheMode)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, remote_subsystem_cache_mode: StorageProvider_RemoteSubsystemCacheMode, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RemoteSubsystemCacheMode".to_string(), value: remote_subsystem_cache_mode.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

