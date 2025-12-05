// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MaskingSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MaskingSet {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// FriendlyName is a user-friendly name of the masking set. It is specified during the creation of the masking set, and can be changed using the SetFriendlyName method.
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// This field specifies the operating system, version, driver, and other host environment factors that influence the behavior exposed by the storage subsystem.
    #[serde(rename = "HostType")]
    pub host_type: Option<MaskingSet_HostType>,

/// Name is a user-friendly system defined name for the masking set. Name is unique within the scope of the owning storage subsystem.
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_MaskingSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            friendly_name: None,
            host_type: None,
            name: None,
        }
    }


    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HostType
    pub fn set_host_type(&mut self, value: MaskingSet_HostType) {
        self.host_type = Some(value);
    }

    /// Gets the value of HostType
    pub fn get_host_type(&self) -> Option<&MaskingSet_HostType> {
        self.host_type.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

/// This method adds an initiator to the masking set. All virtual disks in the masking set will be accessible (shown) to these initiators.

    /// * `host_type` -  (MaskingSet_HostType)
    /// * `initiator_ids` - This parameter is an array of initiator addresses. For each address contained in this array, a corresponding initiator ID instance should be created and then associated with this masking set. (String[])

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_initiator_id(&self, initiator_ids: &Vec<String>, host_type: MaskingSet_HostType, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorIds".to_string(), value: initiator_ids.into() });
        args.push(MethodParameter { name: "HostType".to_string(), value: host_type.into() });

        let result = self.invoke_method("AddInitiatorId", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method removes one or more initiator ids from the masking set. Note that the initiator id instances themselves should not be deleted from the system.

    /// * `initiator_ids` -  (String[])

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_initiator_id(&self, initiator_ids: &Vec<String>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorIds".to_string(), value: initiator_ids.into() });

        let result = self.invoke_method("RemoveInitiatorId", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method adds one or more target ports to the masking set.

    /// * `target_port_addresses` -  (String[])

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_target_port(&self, target_port_addresses: &Vec<String>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });

        let result = self.invoke_method("AddTargetPort", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method removes one or more target ports from the masking set.

    /// * `target_port_addresses` -  (String[])

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_target_port(&self, target_port_addresses: &Vec<String>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });

        let result = self.invoke_method("RemoveTargetPort", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method adds a virtual disk to the masking set, allowing it to be shown to the initiators contained in the set.

    /// * `device_accesses` -  (MaskingSet_DeviceAccesses[])
    /// * `device_numbers` -  (String[])
    /// * `virtual_disk_names` -  (String[])

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_virtual_disk(&self, virtual_disk_names: &Vec<String>, device_numbers: &Vec<String>, device_accesses: &Vec<MaskingSet_DeviceAccesses>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualDiskNames".to_string(), value: virtual_disk_names.into() });
        args.push(MethodParameter { name: "DeviceNumbers".to_string(), value: device_numbers.into() });
        args.push(MethodParameter { name: "DeviceAccesses".to_string(), value: device_accesses.into() });

        let result = self.invoke_method("AddVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method removes a virtual disk from the masking set. Once removed, this virtual disk will no longer be shown to the initiators contained in this masking set.

    /// * `virtual_disk_names` -  (String[])

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_virtual_disk(&self, virtual_disk_names: &Vec<String>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualDiskNames".to_string(), value: virtual_disk_names.into() });

        let result = self.invoke_method("RemoveVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method deletes the masking set instance.

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the FriendlyName to be set.

    /// * `friendly_name` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_friendly_name(&self, friendly_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });

        let result = self.invoke_method("SetFriendlyName", &args)?;
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

}

