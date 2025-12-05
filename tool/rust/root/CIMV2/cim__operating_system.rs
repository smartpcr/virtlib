// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_OperatingSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_OperatingSystem {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "CurrentTimeZone")]
    pub current_time_zone: Option<i16>,

/// 
    #[serde(rename = "Distributed")]
    pub distributed: Option<bool>,

/// 
    #[serde(rename = "FreePhysicalMemory")]
    pub free_physical_memory: Option<u64>,

/// 
    #[serde(rename = "FreeSpaceInPagingFiles")]
    pub free_space_in_paging_files: Option<u64>,

/// 
    #[serde(rename = "FreeVirtualMemory")]
    pub free_virtual_memory: Option<u64>,

/// 
    #[serde(rename = "LastBootUpTime")]
    pub last_boot_up_time: Option<String>,

/// 
    #[serde(rename = "LocalDateTime")]
    pub local_date_time: Option<String>,

/// 
    #[serde(rename = "MaxNumberOfProcesses")]
    pub max_number_of_processes: Option<u32>,

/// 
    #[serde(rename = "MaxProcessMemorySize")]
    pub max_process_memory_size: Option<u64>,

/// 
    #[serde(rename = "NumberOfLicensedUsers")]
    pub number_of_licensed_users: Option<u32>,

/// 
    #[serde(rename = "NumberOfProcesses")]
    pub number_of_processes: Option<u32>,

/// 
    #[serde(rename = "NumberOfUsers")]
    pub number_of_users: Option<u32>,

/// 
    #[serde(rename = "OSType")]
    pub ostype: Option<u16>,

/// 
    #[serde(rename = "OtherTypeDescription")]
    pub other_type_description: Option<String>,

/// 
    #[serde(rename = "SizeStoredInPagingFiles")]
    pub size_stored_in_paging_files: Option<u64>,

/// 
    #[serde(rename = "TotalSwapSpaceSize")]
    pub total_swap_space_size: Option<u64>,

/// 
    #[serde(rename = "TotalVirtualMemorySize")]
    pub total_virtual_memory_size: Option<u64>,

/// 
    #[serde(rename = "TotalVisibleMemorySize")]
    pub total_visible_memory_size: Option<u64>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl CIM_OperatingSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            cscreation_class_name: None,
            csname: None,
            current_time_zone: None,
            distributed: None,
            free_physical_memory: None,
            free_space_in_paging_files: None,
            free_virtual_memory: None,
            last_boot_up_time: None,
            local_date_time: None,
            max_number_of_processes: None,
            max_process_memory_size: None,
            number_of_licensed_users: None,
            number_of_processes: None,
            number_of_users: None,
            ostype: None,
            other_type_description: None,
            size_stored_in_paging_files: None,
            total_swap_space_size: None,
            total_virtual_memory_size: None,
            total_visible_memory_size: None,
            version: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of CSCreationClassName
    pub fn set_cscreation_class_name(&mut self, value: String) {
        self.cscreation_class_name = Some(value);
    }

    /// Gets the value of CSCreationClassName
    pub fn get_cscreation_class_name(&self) -> Option<&String> {
        self.cscreation_class_name.as_ref()
    }

    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of CurrentTimeZone
    pub fn set_current_time_zone(&mut self, value: i16) {
        self.current_time_zone = Some(value);
    }

    /// Gets the value of CurrentTimeZone
    pub fn get_current_time_zone(&self) -> Option<&i16> {
        self.current_time_zone.as_ref()
    }

    /// Sets the value of Distributed
    pub fn set_distributed(&mut self, value: bool) {
        self.distributed = Some(value);
    }

    /// Gets the value of Distributed
    pub fn get_distributed(&self) -> Option<&bool> {
        self.distributed.as_ref()
    }

    /// Sets the value of FreePhysicalMemory
    pub fn set_free_physical_memory(&mut self, value: u64) {
        self.free_physical_memory = Some(value);
    }

    /// Gets the value of FreePhysicalMemory
    pub fn get_free_physical_memory(&self) -> Option<&u64> {
        self.free_physical_memory.as_ref()
    }

    /// Sets the value of FreeSpaceInPagingFiles
    pub fn set_free_space_in_paging_files(&mut self, value: u64) {
        self.free_space_in_paging_files = Some(value);
    }

    /// Gets the value of FreeSpaceInPagingFiles
    pub fn get_free_space_in_paging_files(&self) -> Option<&u64> {
        self.free_space_in_paging_files.as_ref()
    }

    /// Sets the value of FreeVirtualMemory
    pub fn set_free_virtual_memory(&mut self, value: u64) {
        self.free_virtual_memory = Some(value);
    }

    /// Gets the value of FreeVirtualMemory
    pub fn get_free_virtual_memory(&self) -> Option<&u64> {
        self.free_virtual_memory.as_ref()
    }

    /// Sets the value of LastBootUpTime
    pub fn set_last_boot_up_time(&mut self, value: String) {
        self.last_boot_up_time = Some(value);
    }

    /// Gets the value of LastBootUpTime
    pub fn get_last_boot_up_time(&self) -> Option<&String> {
        self.last_boot_up_time.as_ref()
    }

    /// Sets the value of LocalDateTime
    pub fn set_local_date_time(&mut self, value: String) {
        self.local_date_time = Some(value);
    }

    /// Gets the value of LocalDateTime
    pub fn get_local_date_time(&self) -> Option<&String> {
        self.local_date_time.as_ref()
    }

    /// Sets the value of MaxNumberOfProcesses
    pub fn set_max_number_of_processes(&mut self, value: u32) {
        self.max_number_of_processes = Some(value);
    }

    /// Gets the value of MaxNumberOfProcesses
    pub fn get_max_number_of_processes(&self) -> Option<&u32> {
        self.max_number_of_processes.as_ref()
    }

    /// Sets the value of MaxProcessMemorySize
    pub fn set_max_process_memory_size(&mut self, value: u64) {
        self.max_process_memory_size = Some(value);
    }

    /// Gets the value of MaxProcessMemorySize
    pub fn get_max_process_memory_size(&self) -> Option<&u64> {
        self.max_process_memory_size.as_ref()
    }

    /// Sets the value of NumberOfLicensedUsers
    pub fn set_number_of_licensed_users(&mut self, value: u32) {
        self.number_of_licensed_users = Some(value);
    }

    /// Gets the value of NumberOfLicensedUsers
    pub fn get_number_of_licensed_users(&self) -> Option<&u32> {
        self.number_of_licensed_users.as_ref()
    }

    /// Sets the value of NumberOfProcesses
    pub fn set_number_of_processes(&mut self, value: u32) {
        self.number_of_processes = Some(value);
    }

    /// Gets the value of NumberOfProcesses
    pub fn get_number_of_processes(&self) -> Option<&u32> {
        self.number_of_processes.as_ref()
    }

    /// Sets the value of NumberOfUsers
    pub fn set_number_of_users(&mut self, value: u32) {
        self.number_of_users = Some(value);
    }

    /// Gets the value of NumberOfUsers
    pub fn get_number_of_users(&self) -> Option<&u32> {
        self.number_of_users.as_ref()
    }

    /// Sets the value of OSType
    pub fn set_ostype(&mut self, value: u16) {
        self.ostype = Some(value);
    }

    /// Gets the value of OSType
    pub fn get_ostype(&self) -> Option<&u16> {
        self.ostype.as_ref()
    }

    /// Sets the value of OtherTypeDescription
    pub fn set_other_type_description(&mut self, value: String) {
        self.other_type_description = Some(value);
    }

    /// Gets the value of OtherTypeDescription
    pub fn get_other_type_description(&self) -> Option<&String> {
        self.other_type_description.as_ref()
    }

    /// Sets the value of SizeStoredInPagingFiles
    pub fn set_size_stored_in_paging_files(&mut self, value: u64) {
        self.size_stored_in_paging_files = Some(value);
    }

    /// Gets the value of SizeStoredInPagingFiles
    pub fn get_size_stored_in_paging_files(&self) -> Option<&u64> {
        self.size_stored_in_paging_files.as_ref()
    }

    /// Sets the value of TotalSwapSpaceSize
    pub fn set_total_swap_space_size(&mut self, value: u64) {
        self.total_swap_space_size = Some(value);
    }

    /// Gets the value of TotalSwapSpaceSize
    pub fn get_total_swap_space_size(&self) -> Option<&u64> {
        self.total_swap_space_size.as_ref()
    }

    /// Sets the value of TotalVirtualMemorySize
    pub fn set_total_virtual_memory_size(&mut self, value: u64) {
        self.total_virtual_memory_size = Some(value);
    }

    /// Gets the value of TotalVirtualMemorySize
    pub fn get_total_virtual_memory_size(&self) -> Option<&u64> {
        self.total_virtual_memory_size.as_ref()
    }

    /// Sets the value of TotalVisibleMemorySize
    pub fn set_total_visible_memory_size(&mut self, value: u64) {
        self.total_visible_memory_size = Some(value);
    }

    /// Gets the value of TotalVisibleMemorySize
    pub fn get_total_visible_memory_size(&self) -> Option<&u64> {
        self.total_visible_memory_size.as_ref()
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

    /// * `return_value` -  (u32)
    pub fn reboot(&self) -> Result<(), WmiError> {
        self.invoke_method("Reboot", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn shutdown(&self) -> Result<(), WmiError> {
        self.invoke_method("Shutdown", &[])

    }

}

