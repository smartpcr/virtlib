// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NamedJobObjectActgInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NamedJobObjectActgInfo {
    #[serde(flatten)]
    pub base: CIM_StatisticalInformation,

/// 
    #[serde(rename = "ActiveProcesses")]
    pub active_processes: Option<u32>,

/// 
    #[serde(rename = "OtherOperationCount")]
    pub other_operation_count: Option<u64>,

/// 
    #[serde(rename = "OtherTransferCount")]
    pub other_transfer_count: Option<u64>,

/// 
    #[serde(rename = "PeakJobMemoryUsed")]
    pub peak_job_memory_used: Option<u32>,

/// 
    #[serde(rename = "PeakProcessMemoryUsed")]
    pub peak_process_memory_used: Option<u32>,

/// 
    #[serde(rename = "ReadOperationCount")]
    pub read_operation_count: Option<u64>,

/// 
    #[serde(rename = "ReadTransferCount")]
    pub read_transfer_count: Option<u64>,

/// 
    #[serde(rename = "ThisPeriodTotalKernelTime")]
    pub this_period_total_kernel_time: Option<u64>,

/// 
    #[serde(rename = "ThisPeriodTotalUserTime")]
    pub this_period_total_user_time: Option<u64>,

/// 
    #[serde(rename = "TotalKernelTime")]
    pub total_kernel_time: Option<u64>,

/// 
    #[serde(rename = "TotalPageFaultCount")]
    pub total_page_fault_count: Option<u32>,

/// 
    #[serde(rename = "TotalProcesses")]
    pub total_processes: Option<u32>,

/// 
    #[serde(rename = "TotalTerminatedProcesses")]
    pub total_terminated_processes: Option<u32>,

/// 
    #[serde(rename = "TotalUserTime")]
    pub total_user_time: Option<u64>,

/// 
    #[serde(rename = "WriteOperationCount")]
    pub write_operation_count: Option<u64>,

/// 
    #[serde(rename = "WriteTransferCount")]
    pub write_transfer_count: Option<u64>,
}

impl Win32_NamedJobObjectActgInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StatisticalInformation::new(),
            active_processes: None,
            other_operation_count: None,
            other_transfer_count: None,
            peak_job_memory_used: None,
            peak_process_memory_used: None,
            read_operation_count: None,
            read_transfer_count: None,
            this_period_total_kernel_time: None,
            this_period_total_user_time: None,
            total_kernel_time: None,
            total_page_fault_count: None,
            total_processes: None,
            total_terminated_processes: None,
            total_user_time: None,
            write_operation_count: None,
            write_transfer_count: None,
        }
    }


    /// Sets the value of ActiveProcesses
    pub fn set_active_processes(&mut self, value: u32) {
        self.active_processes = Some(value);
    }

    /// Gets the value of ActiveProcesses
    pub fn get_active_processes(&self) -> Option<&u32> {
        self.active_processes.as_ref()
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

    /// Sets the value of PeakJobMemoryUsed
    pub fn set_peak_job_memory_used(&mut self, value: u32) {
        self.peak_job_memory_used = Some(value);
    }

    /// Gets the value of PeakJobMemoryUsed
    pub fn get_peak_job_memory_used(&self) -> Option<&u32> {
        self.peak_job_memory_used.as_ref()
    }

    /// Sets the value of PeakProcessMemoryUsed
    pub fn set_peak_process_memory_used(&mut self, value: u32) {
        self.peak_process_memory_used = Some(value);
    }

    /// Gets the value of PeakProcessMemoryUsed
    pub fn get_peak_process_memory_used(&self) -> Option<&u32> {
        self.peak_process_memory_used.as_ref()
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

    /// Sets the value of ThisPeriodTotalKernelTime
    pub fn set_this_period_total_kernel_time(&mut self, value: u64) {
        self.this_period_total_kernel_time = Some(value);
    }

    /// Gets the value of ThisPeriodTotalKernelTime
    pub fn get_this_period_total_kernel_time(&self) -> Option<&u64> {
        self.this_period_total_kernel_time.as_ref()
    }

    /// Sets the value of ThisPeriodTotalUserTime
    pub fn set_this_period_total_user_time(&mut self, value: u64) {
        self.this_period_total_user_time = Some(value);
    }

    /// Gets the value of ThisPeriodTotalUserTime
    pub fn get_this_period_total_user_time(&self) -> Option<&u64> {
        self.this_period_total_user_time.as_ref()
    }

    /// Sets the value of TotalKernelTime
    pub fn set_total_kernel_time(&mut self, value: u64) {
        self.total_kernel_time = Some(value);
    }

    /// Gets the value of TotalKernelTime
    pub fn get_total_kernel_time(&self) -> Option<&u64> {
        self.total_kernel_time.as_ref()
    }

    /// Sets the value of TotalPageFaultCount
    pub fn set_total_page_fault_count(&mut self, value: u32) {
        self.total_page_fault_count = Some(value);
    }

    /// Gets the value of TotalPageFaultCount
    pub fn get_total_page_fault_count(&self) -> Option<&u32> {
        self.total_page_fault_count.as_ref()
    }

    /// Sets the value of TotalProcesses
    pub fn set_total_processes(&mut self, value: u32) {
        self.total_processes = Some(value);
    }

    /// Gets the value of TotalProcesses
    pub fn get_total_processes(&self) -> Option<&u32> {
        self.total_processes.as_ref()
    }

    /// Sets the value of TotalTerminatedProcesses
    pub fn set_total_terminated_processes(&mut self, value: u32) {
        self.total_terminated_processes = Some(value);
    }

    /// Gets the value of TotalTerminatedProcesses
    pub fn get_total_terminated_processes(&self) -> Option<&u32> {
        self.total_terminated_processes.as_ref()
    }

    /// Sets the value of TotalUserTime
    pub fn set_total_user_time(&mut self, value: u64) {
        self.total_user_time = Some(value);
    }

    /// Gets the value of TotalUserTime
    pub fn get_total_user_time(&self) -> Option<&u64> {
        self.total_user_time.as_ref()
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
}

