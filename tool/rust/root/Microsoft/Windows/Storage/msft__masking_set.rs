// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MaskingSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MaskingSet {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HostType")]
    pub host_type: Option<u16>,

/// 
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
    pub fn set_host_type(&mut self, value: u16) {
        self.host_type = Some(value);
    }

    /// Gets the value of HostType
    pub fn get_host_type(&self) -> Option<&u16> {
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

/// 

    /// * `host_type` -  (u16)
    /// * `initiator_ids` -  (String[])
    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_initiator_id(&self, initiator_ids: &Vec<String>, host_type: u16, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorIds".to_string(), value: initiator_ids.into() });
        args.push(MethodParameter { name: "HostType".to_string(), value: host_type.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("AddInitiatorId", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `initiator_ids` -  (String[])
    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_initiator_id(&self, initiator_ids: &Vec<String>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorIds".to_string(), value: initiator_ids.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("RemoveInitiatorId", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `target_port_addresses` -  (String[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_target_port(&self, target_port_addresses: &Vec<String>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("AddTargetPort", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `target_port_addresses` -  (String[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_target_port(&self, target_port_addresses: &Vec<String>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("RemoveTargetPort", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `device_accesses` -  (u16[])
    /// * `device_numbers` -  (String[])
    /// * `run_as_job` -  (bool)
    /// * `virtual_disk_names` -  (String[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_virtual_disk(&self, virtual_disk_names: &Vec<String>, device_numbers: &Vec<String>, device_accesses: &Vec<u16>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualDiskNames".to_string(), value: virtual_disk_names.into() });
        args.push(MethodParameter { name: "DeviceNumbers".to_string(), value: device_numbers.into() });
        args.push(MethodParameter { name: "DeviceAccesses".to_string(), value: device_accesses.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("AddVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `virtual_disk_names` -  (String[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_virtual_disk(&self, virtual_disk_names: &Vec<String>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualDiskNames".to_string(), value: virtual_disk_names.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("RemoveVirtualDisk", &args)?;
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

}

