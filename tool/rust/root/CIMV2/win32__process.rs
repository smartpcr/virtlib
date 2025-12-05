// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Process struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Process {
    #[serde(flatten)]
    pub base: CIM_Process,

/// 
    #[serde(rename = "CommandLine")]
    pub command_line: Option<String>,

/// 
    #[serde(rename = "ExecutablePath")]
    pub executable_path: Option<String>,

/// 
    #[serde(rename = "HandleCount")]
    pub handle_count: Option<u32>,

/// 
    #[serde(rename = "MaximumWorkingSetSize")]
    pub maximum_working_set_size: Option<u32>,

/// 
    #[serde(rename = "MinimumWorkingSetSize")]
    pub minimum_working_set_size: Option<u32>,

/// 
    #[serde(rename = "OtherOperationCount")]
    pub other_operation_count: Option<u64>,

/// 
    #[serde(rename = "OtherTransferCount")]
    pub other_transfer_count: Option<u64>,

/// 
    #[serde(rename = "PageFaults")]
    pub page_faults: Option<u32>,

/// 
    #[serde(rename = "PageFileUsage")]
    pub page_file_usage: Option<u32>,

/// 
    #[serde(rename = "ParentProcessId")]
    pub parent_process_id: Option<u32>,

/// 
    #[serde(rename = "PeakPageFileUsage")]
    pub peak_page_file_usage: Option<u32>,

/// 
    #[serde(rename = "PeakVirtualSize")]
    pub peak_virtual_size: Option<u64>,

/// 
    #[serde(rename = "PeakWorkingSetSize")]
    pub peak_working_set_size: Option<u32>,

/// 
    #[serde(rename = "PrivatePageCount")]
    pub private_page_count: Option<u64>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "QuotaNonPagedPoolUsage")]
    pub quota_non_paged_pool_usage: Option<u32>,

/// 
    #[serde(rename = "QuotaPagedPoolUsage")]
    pub quota_paged_pool_usage: Option<u32>,

/// 
    #[serde(rename = "QuotaPeakNonPagedPoolUsage")]
    pub quota_peak_non_paged_pool_usage: Option<u32>,

/// 
    #[serde(rename = "QuotaPeakPagedPoolUsage")]
    pub quota_peak_paged_pool_usage: Option<u32>,

/// 
    #[serde(rename = "ReadOperationCount")]
    pub read_operation_count: Option<u64>,

/// 
    #[serde(rename = "ReadTransferCount")]
    pub read_transfer_count: Option<u64>,

/// 
    #[serde(rename = "SessionId")]
    pub session_id: Option<u32>,

/// 
    #[serde(rename = "ThreadCount")]
    pub thread_count: Option<u32>,

/// 
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<u64>,

/// 
    #[serde(rename = "WindowsVersion")]
    pub windows_version: Option<String>,

/// 
    #[serde(rename = "WriteOperationCount")]
    pub write_operation_count: Option<u64>,

/// 
    #[serde(rename = "WriteTransferCount")]
    pub write_transfer_count: Option<u64>,
}

impl Win32_Process {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Process::new(),
            command_line: None,
            executable_path: None,
            handle_count: None,
            maximum_working_set_size: None,
            minimum_working_set_size: None,
            other_operation_count: None,
            other_transfer_count: None,
            page_faults: None,
            page_file_usage: None,
            parent_process_id: None,
            peak_page_file_usage: None,
            peak_virtual_size: None,
            peak_working_set_size: None,
            private_page_count: None,
            process_id: None,
            quota_non_paged_pool_usage: None,
            quota_paged_pool_usage: None,
            quota_peak_non_paged_pool_usage: None,
            quota_peak_paged_pool_usage: None,
            read_operation_count: None,
            read_transfer_count: None,
            session_id: None,
            thread_count: None,
            virtual_size: None,
            windows_version: None,
            write_operation_count: None,
            write_transfer_count: None,
        }
    }


    /// Sets the value of CommandLine
    pub fn set_command_line(&mut self, value: String) {
        self.command_line = Some(value);
    }

    /// Gets the value of CommandLine
    pub fn get_command_line(&self) -> Option<&String> {
        self.command_line.as_ref()
    }

    /// Sets the value of ExecutablePath
    pub fn set_executable_path(&mut self, value: String) {
        self.executable_path = Some(value);
    }

    /// Gets the value of ExecutablePath
    pub fn get_executable_path(&self) -> Option<&String> {
        self.executable_path.as_ref()
    }

    /// Sets the value of HandleCount
    pub fn set_handle_count(&mut self, value: u32) {
        self.handle_count = Some(value);
    }

    /// Gets the value of HandleCount
    pub fn get_handle_count(&self) -> Option<&u32> {
        self.handle_count.as_ref()
    }

    /// Sets the value of MaximumWorkingSetSize
    pub fn set_maximum_working_set_size(&mut self, value: u32) {
        self.maximum_working_set_size = Some(value);
    }

    /// Gets the value of MaximumWorkingSetSize
    pub fn get_maximum_working_set_size(&self) -> Option<&u32> {
        self.maximum_working_set_size.as_ref()
    }

    /// Sets the value of MinimumWorkingSetSize
    pub fn set_minimum_working_set_size(&mut self, value: u32) {
        self.minimum_working_set_size = Some(value);
    }

    /// Gets the value of MinimumWorkingSetSize
    pub fn get_minimum_working_set_size(&self) -> Option<&u32> {
        self.minimum_working_set_size.as_ref()
    }

    /// Sets the value of OtherOperationCount
    pub fn set_other_operation_count(&mut self, value: u64) {
        self.other_operation_count = Some(value);
    }

    /// Gets the value of OtherOperationCount
    pub fn get_other_operation_count(&self) -> Option<&u64> {
        self.other_operation_count.as_ref()
    }

    /// Sets the value of OtherTransferCount
    pub fn set_other_transfer_count(&mut self, value: u64) {
        self.other_transfer_count = Some(value);
    }

    /// Gets the value of OtherTransferCount
    pub fn get_other_transfer_count(&self) -> Option<&u64> {
        self.other_transfer_count.as_ref()
    }

    /// Sets the value of PageFaults
    pub fn set_page_faults(&mut self, value: u32) {
        self.page_faults = Some(value);
    }

    /// Gets the value of PageFaults
    pub fn get_page_faults(&self) -> Option<&u32> {
        self.page_faults.as_ref()
    }

    /// Sets the value of PageFileUsage
    pub fn set_page_file_usage(&mut self, value: u32) {
        self.page_file_usage = Some(value);
    }

    /// Gets the value of PageFileUsage
    pub fn get_page_file_usage(&self) -> Option<&u32> {
        self.page_file_usage.as_ref()
    }

    /// Sets the value of ParentProcessId
    pub fn set_parent_process_id(&mut self, value: u32) {
        self.parent_process_id = Some(value);
    }

    /// Gets the value of ParentProcessId
    pub fn get_parent_process_id(&self) -> Option<&u32> {
        self.parent_process_id.as_ref()
    }

    /// Sets the value of PeakPageFileUsage
    pub fn set_peak_page_file_usage(&mut self, value: u32) {
        self.peak_page_file_usage = Some(value);
    }

    /// Gets the value of PeakPageFileUsage
    pub fn get_peak_page_file_usage(&self) -> Option<&u32> {
        self.peak_page_file_usage.as_ref()
    }

    /// Sets the value of PeakVirtualSize
    pub fn set_peak_virtual_size(&mut self, value: u64) {
        self.peak_virtual_size = Some(value);
    }

    /// Gets the value of PeakVirtualSize
    pub fn get_peak_virtual_size(&self) -> Option<&u64> {
        self.peak_virtual_size.as_ref()
    }

    /// Sets the value of PeakWorkingSetSize
    pub fn set_peak_working_set_size(&mut self, value: u32) {
        self.peak_working_set_size = Some(value);
    }

    /// Gets the value of PeakWorkingSetSize
    pub fn get_peak_working_set_size(&self) -> Option<&u32> {
        self.peak_working_set_size.as_ref()
    }

    /// Sets the value of PrivatePageCount
    pub fn set_private_page_count(&mut self, value: u64) {
        self.private_page_count = Some(value);
    }

    /// Gets the value of PrivatePageCount
    pub fn get_private_page_count(&self) -> Option<&u64> {
        self.private_page_count.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of QuotaNonPagedPoolUsage
    pub fn set_quota_non_paged_pool_usage(&mut self, value: u32) {
        self.quota_non_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaNonPagedPoolUsage
    pub fn get_quota_non_paged_pool_usage(&self) -> Option<&u32> {
        self.quota_non_paged_pool_usage.as_ref()
    }

    /// Sets the value of QuotaPagedPoolUsage
    pub fn set_quota_paged_pool_usage(&mut self, value: u32) {
        self.quota_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaPagedPoolUsage
    pub fn get_quota_paged_pool_usage(&self) -> Option<&u32> {
        self.quota_paged_pool_usage.as_ref()
    }

    /// Sets the value of QuotaPeakNonPagedPoolUsage
    pub fn set_quota_peak_non_paged_pool_usage(&mut self, value: u32) {
        self.quota_peak_non_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaPeakNonPagedPoolUsage
    pub fn get_quota_peak_non_paged_pool_usage(&self) -> Option<&u32> {
        self.quota_peak_non_paged_pool_usage.as_ref()
    }

    /// Sets the value of QuotaPeakPagedPoolUsage
    pub fn set_quota_peak_paged_pool_usage(&mut self, value: u32) {
        self.quota_peak_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaPeakPagedPoolUsage
    pub fn get_quota_peak_paged_pool_usage(&self) -> Option<&u32> {
        self.quota_peak_paged_pool_usage.as_ref()
    }

    /// Sets the value of ReadOperationCount
    pub fn set_read_operation_count(&mut self, value: u64) {
        self.read_operation_count = Some(value);
    }

    /// Gets the value of ReadOperationCount
    pub fn get_read_operation_count(&self) -> Option<&u64> {
        self.read_operation_count.as_ref()
    }

    /// Sets the value of ReadTransferCount
    pub fn set_read_transfer_count(&mut self, value: u64) {
        self.read_transfer_count = Some(value);
    }

    /// Gets the value of ReadTransferCount
    pub fn get_read_transfer_count(&self) -> Option<&u64> {
        self.read_transfer_count.as_ref()
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: u32) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&u32> {
        self.session_id.as_ref()
    }

    /// Sets the value of ThreadCount
    pub fn set_thread_count(&mut self, value: u32) {
        self.thread_count = Some(value);
    }

    /// Gets the value of ThreadCount
    pub fn get_thread_count(&self) -> Option<&u32> {
        self.thread_count.as_ref()
    }

    /// Sets the value of VirtualSize
    pub fn set_virtual_size(&mut self, value: u64) {
        self.virtual_size = Some(value);
    }

    /// Gets the value of VirtualSize
    pub fn get_virtual_size(&self) -> Option<&u64> {
        self.virtual_size.as_ref()
    }

    /// Sets the value of WindowsVersion
    pub fn set_windows_version(&mut self, value: String) {
        self.windows_version = Some(value);
    }

    /// Gets the value of WindowsVersion
    pub fn get_windows_version(&self) -> Option<&String> {
        self.windows_version.as_ref()
    }

    /// Sets the value of WriteOperationCount
    pub fn set_write_operation_count(&mut self, value: u64) {
        self.write_operation_count = Some(value);
    }

    /// Gets the value of WriteOperationCount
    pub fn get_write_operation_count(&self) -> Option<&u64> {
        self.write_operation_count.as_ref()
    }

    /// Sets the value of WriteTransferCount
    pub fn set_write_transfer_count(&mut self, value: u64) {
        self.write_transfer_count = Some(value);
    }

    /// Gets the value of WriteTransferCount
    pub fn get_write_transfer_count(&self) -> Option<&u64> {
        self.write_transfer_count.as_ref()
    }

/// 

    /// * `command_line` -  (String)
    /// * `current_directory` -  (String)
    /// * `process_startup_information` -  (Win32_ProcessStartup)

    /// * `process_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn create(&self, command_line: &String, current_directory: &String, process_startup_information: Win32_ProcessStartup, process_id: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CommandLine".to_string(), value: command_line.into() });
        args.push(MethodParameter { name: "CurrentDirectory".to_string(), value: current_directory.into() });
        args.push(MethodParameter { name: "ProcessStartupInformation".to_string(), value: process_startup_information.into() });

        let result = self.invoke_method("Create", &args)?;
        let process_id = result.get_value("ProcessId")?;
        Ok(result.return_value)

    }


/// 

    /// * `reason` -  (u32)

    /// * `return_value` -  (u32)
    pub fn terminate(&self, reason: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });
        self.invoke_method("Terminate", &args)

    }


/// 

    /// * `domain` -  (String)
    /// * `return_value` -  (u32)
    /// * `user` -  (String)
    pub fn get_owner(&self, user: &mut String, domain: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetOwner", &[])?;
        let domain = result.get_value("Domain")?;
        let user = result.get_value("User")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `sid` -  (String)
    pub fn get_owner_sid(&self, sid: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetOwnerSid", &[])?;
        let sid = result.get_value("Sid")?;
        Ok(result.return_value)

    }


/// 

    /// * `priority` -  (i32)

    /// * `return_value` -  (u32)
    pub fn set_priority(&self, priority: i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Priority".to_string(), value: priority.into() });
        self.invoke_method("SetPriority", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn attach_debugger(&self) -> Result<(), WmiError> {
        self.invoke_method("AttachDebugger", &[])

    }


/// 

    /// * `available_virtual_size` -  (u64)
    /// * `return_value` -  (u32)
    pub fn get_available_virtual_size(&self, available_virtual_size: &mut u64) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAvailableVirtualSize", &[])?;
        let available_virtual_size = result.get_value("AvailableVirtualSize")?;
        Ok(result.return_value)

    }

}

