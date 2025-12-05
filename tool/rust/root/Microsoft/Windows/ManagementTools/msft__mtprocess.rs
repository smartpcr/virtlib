// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTProcess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTProcess {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "BasePriority")]
    pub base_priority: Option<u32>,

/// 
    #[serde(rename = "CommandLine")]
    pub command_line: Option<String>,

/// 
    #[serde(rename = "CommitCharge")]
    pub commit_charge: Option<u64>,

/// 
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: Option<f32>,

/// 
    #[serde(rename = "CpuTime")]
    pub cpu_time: Option<u64>,

/// 
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<u64>,

/// 
    #[serde(rename = "CyclePercent")]
    pub cycle_percent: Option<f32>,

/// 
    #[serde(rename = "CycleTime")]
    pub cycle_time: Option<u64>,

/// 
    #[serde(rename = "DataExecutionPrevention")]
    pub data_execution_prevention: Option<bool>,

/// 
    #[serde(rename = "DeltaPageFaults")]
    pub delta_page_faults: Option<i32>,

/// 
    #[serde(rename = "DeltaWorkingSetSize")]
    pub delta_working_set_size: Option<i64>,

/// 
    #[serde(rename = "Elevated")]
    pub elevated: Option<bool>,

/// 
    #[serde(rename = "ExecutablePath")]
    pub executable_path: Option<String>,

/// 
    #[serde(rename = "GdiObjects")]
    pub gdi_objects: Option<u32>,

/// 
    #[serde(rename = "HandleCount")]
    pub handle_count: Option<u32>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "IsImmersive")]
    pub is_immersive: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NonPagedPool")]
    pub non_paged_pool: Option<u64>,

/// 
    #[serde(rename = "OperatingSystemContext")]
    pub operating_system_context: Option<u16>,

/// 
    #[serde(rename = "OtherOperationCount")]
    pub other_operation_count: Option<u64>,

/// 
    #[serde(rename = "OtherTransferCount")]
    pub other_transfer_count: Option<u64>,

/// 
    #[serde(rename = "PagedPool")]
    pub paged_pool: Option<u64>,

/// 
    #[serde(rename = "PageFaults")]
    pub page_faults: Option<u32>,

/// 
    #[serde(rename = "PeakWorkingSetSize")]
    pub peak_working_set_size: Option<u64>,

/// 
    #[serde(rename = "Platform")]
    pub platform: Option<u16>,

/// 
    #[serde(rename = "PrivateWorkingSetSize")]
    pub private_working_set_size: Option<u64>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ProcessStatus")]
    pub process_status: Option<u16>,

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
    #[serde(rename = "SharedWorkingSetSize")]
    pub shared_working_set_size: Option<u64>,

/// 
    #[serde(rename = "ThreadCount")]
    pub thread_count: Option<u32>,

/// 
    #[serde(rename = "UACVirtualization")]
    pub uacvirtualization: Option<u16>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// 
    #[serde(rename = "UserObjects")]
    pub user_objects: Option<u32>,

/// 
    #[serde(rename = "WorkingSetSize")]
    pub working_set_size: Option<u64>,

/// 
    #[serde(rename = "WriteOperationCount")]
    pub write_operation_count: Option<u64>,

/// 
    #[serde(rename = "WriteTransferCount")]
    pub write_transfer_count: Option<u64>,
}

impl MSFT_MTProcess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            base_priority: None,
            command_line: None,
            commit_charge: None,
            cpu_percent: None,
            cpu_time: None,
            creation_date: None,
            creation_time: None,
            cycle_percent: None,
            cycle_time: None,
            data_execution_prevention: None,
            delta_page_faults: None,
            delta_working_set_size: None,
            elevated: None,
            executable_path: None,
            gdi_objects: None,
            handle_count: None,
            interval_seconds: None,
            is_immersive: None,
            name: None,
            non_paged_pool: None,
            operating_system_context: None,
            other_operation_count: None,
            other_transfer_count: None,
            paged_pool: None,
            page_faults: None,
            peak_working_set_size: None,
            platform: None,
            private_working_set_size: None,
            process_id: None,
            process_status: None,
            read_operation_count: None,
            read_transfer_count: None,
            session_id: None,
            shared_working_set_size: None,
            thread_count: None,
            uacvirtualization: None,
            user_name: None,
            user_objects: None,
            working_set_size: None,
            write_operation_count: None,
            write_transfer_count: None,
        }
    }


    /// Sets the value of BasePriority
    pub fn set_base_priority(&mut self, value: u32) {
        self.base_priority = Some(value);
    }

    /// Gets the value of BasePriority
    pub fn get_base_priority(&self) -> Option<&u32> {
        self.base_priority.as_ref()
    }

    /// Sets the value of CommandLine
    pub fn set_command_line(&mut self, value: String) {
        self.command_line = Some(value);
    }

    /// Gets the value of CommandLine
    pub fn get_command_line(&self) -> Option<&String> {
        self.command_line.as_ref()
    }

    /// Sets the value of CommitCharge
    pub fn set_commit_charge(&mut self, value: u64) {
        self.commit_charge = Some(value);
    }

    /// Gets the value of CommitCharge
    pub fn get_commit_charge(&self) -> Option<&u64> {
        self.commit_charge.as_ref()
    }

    /// Sets the value of CpuPercent
    pub fn set_cpu_percent(&mut self, value: f32) {
        self.cpu_percent = Some(value);
    }

    /// Gets the value of CpuPercent
    pub fn get_cpu_percent(&self) -> Option<&f32> {
        self.cpu_percent.as_ref()
    }

    /// Sets the value of CpuTime
    pub fn set_cpu_time(&mut self, value: u64) {
        self.cpu_time = Some(value);
    }

    /// Gets the value of CpuTime
    pub fn get_cpu_time(&self) -> Option<&u64> {
        self.cpu_time.as_ref()
    }

    /// Sets the value of CreationDate
    pub fn set_creation_date(&mut self, value: String) {
        self.creation_date = Some(value);
    }

    /// Gets the value of CreationDate
    pub fn get_creation_date(&self) -> Option<&String> {
        self.creation_date.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: u64) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&u64> {
        self.creation_time.as_ref()
    }

    /// Sets the value of CyclePercent
    pub fn set_cycle_percent(&mut self, value: f32) {
        self.cycle_percent = Some(value);
    }

    /// Gets the value of CyclePercent
    pub fn get_cycle_percent(&self) -> Option<&f32> {
        self.cycle_percent.as_ref()
    }

    /// Sets the value of CycleTime
    pub fn set_cycle_time(&mut self, value: u64) {
        self.cycle_time = Some(value);
    }

    /// Gets the value of CycleTime
    pub fn get_cycle_time(&self) -> Option<&u64> {
        self.cycle_time.as_ref()
    }

    /// Sets the value of DataExecutionPrevention
    pub fn set_data_execution_prevention(&mut self, value: bool) {
        self.data_execution_prevention = Some(value);
    }

    /// Gets the value of DataExecutionPrevention
    pub fn get_data_execution_prevention(&self) -> Option<&bool> {
        self.data_execution_prevention.as_ref()
    }

    /// Sets the value of DeltaPageFaults
    pub fn set_delta_page_faults(&mut self, value: i32) {
        self.delta_page_faults = Some(value);
    }

    /// Gets the value of DeltaPageFaults
    pub fn get_delta_page_faults(&self) -> Option<&i32> {
        self.delta_page_faults.as_ref()
    }

    /// Sets the value of DeltaWorkingSetSize
    pub fn set_delta_working_set_size(&mut self, value: i64) {
        self.delta_working_set_size = Some(value);
    }

    /// Gets the value of DeltaWorkingSetSize
    pub fn get_delta_working_set_size(&self) -> Option<&i64> {
        self.delta_working_set_size.as_ref()
    }

    /// Sets the value of Elevated
    pub fn set_elevated(&mut self, value: bool) {
        self.elevated = Some(value);
    }

    /// Gets the value of Elevated
    pub fn get_elevated(&self) -> Option<&bool> {
        self.elevated.as_ref()
    }

    /// Sets the value of ExecutablePath
    pub fn set_executable_path(&mut self, value: String) {
        self.executable_path = Some(value);
    }

    /// Gets the value of ExecutablePath
    pub fn get_executable_path(&self) -> Option<&String> {
        self.executable_path.as_ref()
    }

    /// Sets the value of GdiObjects
    pub fn set_gdi_objects(&mut self, value: u32) {
        self.gdi_objects = Some(value);
    }

    /// Gets the value of GdiObjects
    pub fn get_gdi_objects(&self) -> Option<&u32> {
        self.gdi_objects.as_ref()
    }

    /// Sets the value of HandleCount
    pub fn set_handle_count(&mut self, value: u32) {
        self.handle_count = Some(value);
    }

    /// Gets the value of HandleCount
    pub fn get_handle_count(&self) -> Option<&u32> {
        self.handle_count.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of IsImmersive
    pub fn set_is_immersive(&mut self, value: bool) {
        self.is_immersive = Some(value);
    }

    /// Gets the value of IsImmersive
    pub fn get_is_immersive(&self) -> Option<&bool> {
        self.is_immersive.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NonPagedPool
    pub fn set_non_paged_pool(&mut self, value: u64) {
        self.non_paged_pool = Some(value);
    }

    /// Gets the value of NonPagedPool
    pub fn get_non_paged_pool(&self) -> Option<&u64> {
        self.non_paged_pool.as_ref()
    }

    /// Sets the value of OperatingSystemContext
    pub fn set_operating_system_context(&mut self, value: u16) {
        self.operating_system_context = Some(value);
    }

    /// Gets the value of OperatingSystemContext
    pub fn get_operating_system_context(&self) -> Option<&u16> {
        self.operating_system_context.as_ref()
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

    /// Sets the value of PagedPool
    pub fn set_paged_pool(&mut self, value: u64) {
        self.paged_pool = Some(value);
    }

    /// Gets the value of PagedPool
    pub fn get_paged_pool(&self) -> Option<&u64> {
        self.paged_pool.as_ref()
    }

    /// Sets the value of PageFaults
    pub fn set_page_faults(&mut self, value: u32) {
        self.page_faults = Some(value);
    }

    /// Gets the value of PageFaults
    pub fn get_page_faults(&self) -> Option<&u32> {
        self.page_faults.as_ref()
    }

    /// Sets the value of PeakWorkingSetSize
    pub fn set_peak_working_set_size(&mut self, value: u64) {
        self.peak_working_set_size = Some(value);
    }

    /// Gets the value of PeakWorkingSetSize
    pub fn get_peak_working_set_size(&self) -> Option<&u64> {
        self.peak_working_set_size.as_ref()
    }

    /// Sets the value of Platform
    pub fn set_platform(&mut self, value: u16) {
        self.platform = Some(value);
    }

    /// Gets the value of Platform
    pub fn get_platform(&self) -> Option<&u16> {
        self.platform.as_ref()
    }

    /// Sets the value of PrivateWorkingSetSize
    pub fn set_private_working_set_size(&mut self, value: u64) {
        self.private_working_set_size = Some(value);
    }

    /// Gets the value of PrivateWorkingSetSize
    pub fn get_private_working_set_size(&self) -> Option<&u64> {
        self.private_working_set_size.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ProcessStatus
    pub fn set_process_status(&mut self, value: u16) {
        self.process_status = Some(value);
    }

    /// Gets the value of ProcessStatus
    pub fn get_process_status(&self) -> Option<&u16> {
        self.process_status.as_ref()
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

    /// Sets the value of SharedWorkingSetSize
    pub fn set_shared_working_set_size(&mut self, value: u64) {
        self.shared_working_set_size = Some(value);
    }

    /// Gets the value of SharedWorkingSetSize
    pub fn get_shared_working_set_size(&self) -> Option<&u64> {
        self.shared_working_set_size.as_ref()
    }

    /// Sets the value of ThreadCount
    pub fn set_thread_count(&mut self, value: u32) {
        self.thread_count = Some(value);
    }

    /// Gets the value of ThreadCount
    pub fn get_thread_count(&self) -> Option<&u32> {
        self.thread_count.as_ref()
    }

    /// Sets the value of UACVirtualization
    pub fn set_uacvirtualization(&mut self, value: u16) {
        self.uacvirtualization = Some(value);
    }

    /// Gets the value of UACVirtualization
    pub fn get_uacvirtualization(&self) -> Option<&u16> {
        self.uacvirtualization.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of UserObjects
    pub fn set_user_objects(&mut self, value: u32) {
        self.user_objects = Some(value);
    }

    /// Gets the value of UserObjects
    pub fn get_user_objects(&self) -> Option<&u32> {
        self.user_objects.as_ref()
    }

    /// Sets the value of WorkingSetSize
    pub fn set_working_set_size(&mut self, value: u64) {
        self.working_set_size = Some(value);
    }

    /// Gets the value of WorkingSetSize
    pub fn get_working_set_size(&self) -> Option<&u64> {
        self.working_set_size.as_ref()
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

    /// * `dump_file_path` -  (String)
    /// * `return_value` -  (u32)
    pub fn create_dump(&self, dump_file_path: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("CreateDump", &[])?;
        let dump_file_path = result.get_value("DumpFilePath")?;
        Ok(result.return_value)

    }


/// 

    /// * `command_line` -  (String)
    /// * `wait_milliseconds` -  (u32)

    /// * `actual_command_line` -  (String)
    /// * `process_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn create_process(&self, command_line: &String, wait_milliseconds: u32, process_id: &mut u32, actual_command_line: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CommandLine".to_string(), value: command_line.into() });
        args.push(MethodParameter { name: "WaitMilliseconds".to_string(), value: wait_milliseconds.into() });

        let result = self.invoke_method("CreateProcess", &args)?;
        let actual_command_line = result.get_value("ActualCommandLine")?;
        let process_id = result.get_value("ProcessId")?;
        Ok(result.return_value)

    }

}

