// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_ProcessV2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_ProcessV2 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CreatingProcessID")]
    pub creating_process_id: Option<u32>,

/// 
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<u64>,

/// 
    #[serde(rename = "HandleCount")]
    pub handle_count: Option<u32>,

/// 
    #[serde(rename = "IODataBytesPersec")]
    pub iodata_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IODataOperationsPersec")]
    pub iodata_operations_persec: Option<u64>,

/// 
    #[serde(rename = "IOOtherBytesPersec")]
    pub ioother_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOOtherOperationsPersec")]
    pub ioother_operations_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesPersec")]
    pub ioread_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadOperationsPersec")]
    pub ioread_operations_persec: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytesPersec")]
    pub iowrite_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOWriteOperationsPersec")]
    pub iowrite_operations_persec: Option<u64>,

/// 
    #[serde(rename = "PageFaultsPersec")]
    pub page_faults_persec: Option<u32>,

/// 
    #[serde(rename = "PageFileBytes")]
    pub page_file_bytes: Option<u64>,

/// 
    #[serde(rename = "PageFileBytesPeak")]
    pub page_file_bytes_peak: Option<u64>,

/// 
    #[serde(rename = "PercentPrivilegedTime")]
    pub percent_privileged_time: Option<u64>,

/// 
    #[serde(rename = "PercentProcessorTime")]
    pub percent_processor_time: Option<u64>,

/// 
    #[serde(rename = "PercentUserTime")]
    pub percent_user_time: Option<u64>,

/// 
    #[serde(rename = "PoolNonpagedBytes")]
    pub pool_nonpaged_bytes: Option<u64>,

/// 
    #[serde(rename = "PoolPagedBytes")]
    pub pool_paged_bytes: Option<u64>,

/// 
    #[serde(rename = "PriorityBase")]
    pub priority_base: Option<u32>,

/// 
    #[serde(rename = "PrivateBytes")]
    pub private_bytes: Option<u64>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ThreadCount")]
    pub thread_count: Option<u32>,

/// 
    #[serde(rename = "VirtualBytes")]
    pub virtual_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualBytesPeak")]
    pub virtual_bytes_peak: Option<u64>,

/// 
    #[serde(rename = "WorkingSet")]
    pub working_set: Option<u64>,

/// 
    #[serde(rename = "WorkingSetPeak")]
    pub working_set_peak: Option<u64>,

/// 
    #[serde(rename = "WorkingSetPrivate")]
    pub working_set_private: Option<u64>,
}

impl Win32_PerfRawData_Counters_ProcessV2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            creating_process_id: None,
            elapsed_time: None,
            handle_count: None,
            iodata_bytes_persec: None,
            iodata_operations_persec: None,
            ioother_bytes_persec: None,
            ioother_operations_persec: None,
            ioread_bytes_persec: None,
            ioread_operations_persec: None,
            iowrite_bytes_persec: None,
            iowrite_operations_persec: None,
            page_faults_persec: None,
            page_file_bytes: None,
            page_file_bytes_peak: None,
            percent_privileged_time: None,
            percent_processor_time: None,
            percent_user_time: None,
            pool_nonpaged_bytes: None,
            pool_paged_bytes: None,
            priority_base: None,
            private_bytes: None,
            process_id: None,
            thread_count: None,
            virtual_bytes: None,
            virtual_bytes_peak: None,
            working_set: None,
            working_set_peak: None,
            working_set_private: None,
        }
    }


    /// Sets the value of CreatingProcessID
    pub fn set_creating_process_id(&mut self, value: u32) {
        self.creating_process_id = Some(value);
    }

    /// Gets the value of CreatingProcessID
    pub fn get_creating_process_id(&self) -> Option<&u32> {
        self.creating_process_id.as_ref()
    }

    /// Sets the value of ElapsedTime
    pub fn set_elapsed_time(&mut self, value: u64) {
        self.elapsed_time = Some(value);
    }

    /// Gets the value of ElapsedTime
    pub fn get_elapsed_time(&self) -> Option<&u64> {
        self.elapsed_time.as_ref()
    }

    /// Sets the value of HandleCount
    pub fn set_handle_count(&mut self, value: u32) {
        self.handle_count = Some(value);
    }

    /// Gets the value of HandleCount
    pub fn get_handle_count(&self) -> Option<&u32> {
        self.handle_count.as_ref()
    }

    /// Sets the value of IODataBytesPersec
    pub fn set_iodata_bytes_persec(&mut self, value: u64) {
        self.iodata_bytes_persec = Some(value);
    }

    /// Gets the value of IODataBytesPersec
    pub fn get_iodata_bytes_persec(&self) -> Option<&u64> {
        self.iodata_bytes_persec.as_ref()
    }

    /// Sets the value of IODataOperationsPersec
    pub fn set_iodata_operations_persec(&mut self, value: u64) {
        self.iodata_operations_persec = Some(value);
    }

    /// Gets the value of IODataOperationsPersec
    pub fn get_iodata_operations_persec(&self) -> Option<&u64> {
        self.iodata_operations_persec.as_ref()
    }

    /// Sets the value of IOOtherBytesPersec
    pub fn set_ioother_bytes_persec(&mut self, value: u64) {
        self.ioother_bytes_persec = Some(value);
    }

    /// Gets the value of IOOtherBytesPersec
    pub fn get_ioother_bytes_persec(&self) -> Option<&u64> {
        self.ioother_bytes_persec.as_ref()
    }

    /// Sets the value of IOOtherOperationsPersec
    pub fn set_ioother_operations_persec(&mut self, value: u64) {
        self.ioother_operations_persec = Some(value);
    }

    /// Gets the value of IOOtherOperationsPersec
    pub fn get_ioother_operations_persec(&self) -> Option<&u64> {
        self.ioother_operations_persec.as_ref()
    }

    /// Sets the value of IOReadBytesPersec
    pub fn set_ioread_bytes_persec(&mut self, value: u64) {
        self.ioread_bytes_persec = Some(value);
    }

    /// Gets the value of IOReadBytesPersec
    pub fn get_ioread_bytes_persec(&self) -> Option<&u64> {
        self.ioread_bytes_persec.as_ref()
    }

    /// Sets the value of IOReadOperationsPersec
    pub fn set_ioread_operations_persec(&mut self, value: u64) {
        self.ioread_operations_persec = Some(value);
    }

    /// Gets the value of IOReadOperationsPersec
    pub fn get_ioread_operations_persec(&self) -> Option<&u64> {
        self.ioread_operations_persec.as_ref()
    }

    /// Sets the value of IOWriteBytesPersec
    pub fn set_iowrite_bytes_persec(&mut self, value: u64) {
        self.iowrite_bytes_persec = Some(value);
    }

    /// Gets the value of IOWriteBytesPersec
    pub fn get_iowrite_bytes_persec(&self) -> Option<&u64> {
        self.iowrite_bytes_persec.as_ref()
    }

    /// Sets the value of IOWriteOperationsPersec
    pub fn set_iowrite_operations_persec(&mut self, value: u64) {
        self.iowrite_operations_persec = Some(value);
    }

    /// Gets the value of IOWriteOperationsPersec
    pub fn get_iowrite_operations_persec(&self) -> Option<&u64> {
        self.iowrite_operations_persec.as_ref()
    }

    /// Sets the value of PageFaultsPersec
    pub fn set_page_faults_persec(&mut self, value: u32) {
        self.page_faults_persec = Some(value);
    }

    /// Gets the value of PageFaultsPersec
    pub fn get_page_faults_persec(&self) -> Option<&u32> {
        self.page_faults_persec.as_ref()
    }

    /// Sets the value of PageFileBytes
    pub fn set_page_file_bytes(&mut self, value: u64) {
        self.page_file_bytes = Some(value);
    }

    /// Gets the value of PageFileBytes
    pub fn get_page_file_bytes(&self) -> Option<&u64> {
        self.page_file_bytes.as_ref()
    }

    /// Sets the value of PageFileBytesPeak
    pub fn set_page_file_bytes_peak(&mut self, value: u64) {
        self.page_file_bytes_peak = Some(value);
    }

    /// Gets the value of PageFileBytesPeak
    pub fn get_page_file_bytes_peak(&self) -> Option<&u64> {
        self.page_file_bytes_peak.as_ref()
    }

    /// Sets the value of PercentPrivilegedTime
    pub fn set_percent_privileged_time(&mut self, value: u64) {
        self.percent_privileged_time = Some(value);
    }

    /// Gets the value of PercentPrivilegedTime
    pub fn get_percent_privileged_time(&self) -> Option<&u64> {
        self.percent_privileged_time.as_ref()
    }

    /// Sets the value of PercentProcessorTime
    pub fn set_percent_processor_time(&mut self, value: u64) {
        self.percent_processor_time = Some(value);
    }

    /// Gets the value of PercentProcessorTime
    pub fn get_percent_processor_time(&self) -> Option<&u64> {
        self.percent_processor_time.as_ref()
    }

    /// Sets the value of PercentUserTime
    pub fn set_percent_user_time(&mut self, value: u64) {
        self.percent_user_time = Some(value);
    }

    /// Gets the value of PercentUserTime
    pub fn get_percent_user_time(&self) -> Option<&u64> {
        self.percent_user_time.as_ref()
    }

    /// Sets the value of PoolNonpagedBytes
    pub fn set_pool_nonpaged_bytes(&mut self, value: u64) {
        self.pool_nonpaged_bytes = Some(value);
    }

    /// Gets the value of PoolNonpagedBytes
    pub fn get_pool_nonpaged_bytes(&self) -> Option<&u64> {
        self.pool_nonpaged_bytes.as_ref()
    }

    /// Sets the value of PoolPagedBytes
    pub fn set_pool_paged_bytes(&mut self, value: u64) {
        self.pool_paged_bytes = Some(value);
    }

    /// Gets the value of PoolPagedBytes
    pub fn get_pool_paged_bytes(&self) -> Option<&u64> {
        self.pool_paged_bytes.as_ref()
    }

    /// Sets the value of PriorityBase
    pub fn set_priority_base(&mut self, value: u32) {
        self.priority_base = Some(value);
    }

    /// Gets the value of PriorityBase
    pub fn get_priority_base(&self) -> Option<&u32> {
        self.priority_base.as_ref()
    }

    /// Sets the value of PrivateBytes
    pub fn set_private_bytes(&mut self, value: u64) {
        self.private_bytes = Some(value);
    }

    /// Gets the value of PrivateBytes
    pub fn get_private_bytes(&self) -> Option<&u64> {
        self.private_bytes.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ThreadCount
    pub fn set_thread_count(&mut self, value: u32) {
        self.thread_count = Some(value);
    }

    /// Gets the value of ThreadCount
    pub fn get_thread_count(&self) -> Option<&u32> {
        self.thread_count.as_ref()
    }

    /// Sets the value of VirtualBytes
    pub fn set_virtual_bytes(&mut self, value: u64) {
        self.virtual_bytes = Some(value);
    }

    /// Gets the value of VirtualBytes
    pub fn get_virtual_bytes(&self) -> Option<&u64> {
        self.virtual_bytes.as_ref()
    }

    /// Sets the value of VirtualBytesPeak
    pub fn set_virtual_bytes_peak(&mut self, value: u64) {
        self.virtual_bytes_peak = Some(value);
    }

    /// Gets the value of VirtualBytesPeak
    pub fn get_virtual_bytes_peak(&self) -> Option<&u64> {
        self.virtual_bytes_peak.as_ref()
    }

    /// Sets the value of WorkingSet
    pub fn set_working_set(&mut self, value: u64) {
        self.working_set = Some(value);
    }

    /// Gets the value of WorkingSet
    pub fn get_working_set(&self) -> Option<&u64> {
        self.working_set.as_ref()
    }

    /// Sets the value of WorkingSetPeak
    pub fn set_working_set_peak(&mut self, value: u64) {
        self.working_set_peak = Some(value);
    }

    /// Gets the value of WorkingSetPeak
    pub fn get_working_set_peak(&self) -> Option<&u64> {
        self.working_set_peak.as_ref()
    }

    /// Sets the value of WorkingSetPrivate
    pub fn set_working_set_private(&mut self, value: u64) {
        self.working_set_private = Some(value);
    }

    /// Gets the value of WorkingSetPrivate
    pub fn get_working_set_private(&self) -> Option<&u64> {
        self.working_set_private.as_ref()
    }
}

