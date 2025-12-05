// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfOS_System struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfOS_System {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AlignmentFixupsPersec")]
    pub alignment_fixups_persec: Option<u32>,

/// 
    #[serde(rename = "ContextSwitchesPersec")]
    pub context_switches_persec: Option<u32>,

/// 
    #[serde(rename = "ExceptionDispatchesPersec")]
    pub exception_dispatches_persec: Option<u32>,

/// 
    #[serde(rename = "FileControlBytesPersec")]
    pub file_control_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "FileControlOperationsPersec")]
    pub file_control_operations_persec: Option<u32>,

/// 
    #[serde(rename = "FileDataOperationsPersec")]
    pub file_data_operations_persec: Option<u32>,

/// 
    #[serde(rename = "FileReadBytesPersec")]
    pub file_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "FileReadOperationsPersec")]
    pub file_read_operations_persec: Option<u32>,

/// 
    #[serde(rename = "FileWriteBytesPersec")]
    pub file_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "FileWriteOperationsPersec")]
    pub file_write_operations_persec: Option<u32>,

/// 
    #[serde(rename = "FloatingEmulationsPersec")]
    pub floating_emulations_persec: Option<u32>,

/// 
    #[serde(rename = "PercentRegistryQuotaInUse")]
    pub percent_registry_quota_in_use: Option<u32>,

/// 
    #[serde(rename = "PercentRegistryQuotaInUse_Base")]
    pub percent_registry_quota_in_use__base: Option<u32>,

/// 
    #[serde(rename = "Processes")]
    pub processes: Option<u32>,

/// 
    #[serde(rename = "ProcessorQueueLength")]
    pub processor_queue_length: Option<u32>,

/// 
    #[serde(rename = "SystemCallsPersec")]
    pub system_calls_persec: Option<u32>,

/// 
    #[serde(rename = "SystemUpTime")]
    pub system_up_time: Option<u64>,

/// 
    #[serde(rename = "Threads")]
    pub threads: Option<u32>,
}

impl Win32_PerfRawData_PerfOS_System {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            alignment_fixups_persec: None,
            context_switches_persec: None,
            exception_dispatches_persec: None,
            file_control_bytes_persec: None,
            file_control_operations_persec: None,
            file_data_operations_persec: None,
            file_read_bytes_persec: None,
            file_read_operations_persec: None,
            file_write_bytes_persec: None,
            file_write_operations_persec: None,
            floating_emulations_persec: None,
            percent_registry_quota_in_use: None,
            percent_registry_quota_in_use__base: None,
            processes: None,
            processor_queue_length: None,
            system_calls_persec: None,
            system_up_time: None,
            threads: None,
        }
    }


    /// Sets the value of AlignmentFixupsPersec
    pub fn set_alignment_fixups_persec(&mut self, value: u32) {
        self.alignment_fixups_persec = Some(value);
    }

    /// Gets the value of AlignmentFixupsPersec
    pub fn get_alignment_fixups_persec(&self) -> Option<&u32> {
        self.alignment_fixups_persec.as_ref()
    }

    /// Sets the value of ContextSwitchesPersec
    pub fn set_context_switches_persec(&mut self, value: u32) {
        self.context_switches_persec = Some(value);
    }

    /// Gets the value of ContextSwitchesPersec
    pub fn get_context_switches_persec(&self) -> Option<&u32> {
        self.context_switches_persec.as_ref()
    }

    /// Sets the value of ExceptionDispatchesPersec
    pub fn set_exception_dispatches_persec(&mut self, value: u32) {
        self.exception_dispatches_persec = Some(value);
    }

    /// Gets the value of ExceptionDispatchesPersec
    pub fn get_exception_dispatches_persec(&self) -> Option<&u32> {
        self.exception_dispatches_persec.as_ref()
    }

    /// Sets the value of FileControlBytesPersec
    pub fn set_file_control_bytes_persec(&mut self, value: u64) {
        self.file_control_bytes_persec = Some(value);
    }

    /// Gets the value of FileControlBytesPersec
    pub fn get_file_control_bytes_persec(&self) -> Option<&u64> {
        self.file_control_bytes_persec.as_ref()
    }

    /// Sets the value of FileControlOperationsPersec
    pub fn set_file_control_operations_persec(&mut self, value: u32) {
        self.file_control_operations_persec = Some(value);
    }

    /// Gets the value of FileControlOperationsPersec
    pub fn get_file_control_operations_persec(&self) -> Option<&u32> {
        self.file_control_operations_persec.as_ref()
    }

    /// Sets the value of FileDataOperationsPersec
    pub fn set_file_data_operations_persec(&mut self, value: u32) {
        self.file_data_operations_persec = Some(value);
    }

    /// Gets the value of FileDataOperationsPersec
    pub fn get_file_data_operations_persec(&self) -> Option<&u32> {
        self.file_data_operations_persec.as_ref()
    }

    /// Sets the value of FileReadBytesPersec
    pub fn set_file_read_bytes_persec(&mut self, value: u64) {
        self.file_read_bytes_persec = Some(value);
    }

    /// Gets the value of FileReadBytesPersec
    pub fn get_file_read_bytes_persec(&self) -> Option<&u64> {
        self.file_read_bytes_persec.as_ref()
    }

    /// Sets the value of FileReadOperationsPersec
    pub fn set_file_read_operations_persec(&mut self, value: u32) {
        self.file_read_operations_persec = Some(value);
    }

    /// Gets the value of FileReadOperationsPersec
    pub fn get_file_read_operations_persec(&self) -> Option<&u32> {
        self.file_read_operations_persec.as_ref()
    }

    /// Sets the value of FileWriteBytesPersec
    pub fn set_file_write_bytes_persec(&mut self, value: u64) {
        self.file_write_bytes_persec = Some(value);
    }

    /// Gets the value of FileWriteBytesPersec
    pub fn get_file_write_bytes_persec(&self) -> Option<&u64> {
        self.file_write_bytes_persec.as_ref()
    }

    /// Sets the value of FileWriteOperationsPersec
    pub fn set_file_write_operations_persec(&mut self, value: u32) {
        self.file_write_operations_persec = Some(value);
    }

    /// Gets the value of FileWriteOperationsPersec
    pub fn get_file_write_operations_persec(&self) -> Option<&u32> {
        self.file_write_operations_persec.as_ref()
    }

    /// Sets the value of FloatingEmulationsPersec
    pub fn set_floating_emulations_persec(&mut self, value: u32) {
        self.floating_emulations_persec = Some(value);
    }

    /// Gets the value of FloatingEmulationsPersec
    pub fn get_floating_emulations_persec(&self) -> Option<&u32> {
        self.floating_emulations_persec.as_ref()
    }

    /// Sets the value of PercentRegistryQuotaInUse
    pub fn set_percent_registry_quota_in_use(&mut self, value: u32) {
        self.percent_registry_quota_in_use = Some(value);
    }

    /// Gets the value of PercentRegistryQuotaInUse
    pub fn get_percent_registry_quota_in_use(&self) -> Option<&u32> {
        self.percent_registry_quota_in_use.as_ref()
    }

    /// Sets the value of PercentRegistryQuotaInUse_Base
    pub fn set_percent_registry_quota_in_use__base(&mut self, value: u32) {
        self.percent_registry_quota_in_use__base = Some(value);
    }

    /// Gets the value of PercentRegistryQuotaInUse_Base
    pub fn get_percent_registry_quota_in_use__base(&self) -> Option<&u32> {
        self.percent_registry_quota_in_use__base.as_ref()
    }

    /// Sets the value of Processes
    pub fn set_processes(&mut self, value: u32) {
        self.processes = Some(value);
    }

    /// Gets the value of Processes
    pub fn get_processes(&self) -> Option<&u32> {
        self.processes.as_ref()
    }

    /// Sets the value of ProcessorQueueLength
    pub fn set_processor_queue_length(&mut self, value: u32) {
        self.processor_queue_length = Some(value);
    }

    /// Gets the value of ProcessorQueueLength
    pub fn get_processor_queue_length(&self) -> Option<&u32> {
        self.processor_queue_length.as_ref()
    }

    /// Sets the value of SystemCallsPersec
    pub fn set_system_calls_persec(&mut self, value: u32) {
        self.system_calls_persec = Some(value);
    }

    /// Gets the value of SystemCallsPersec
    pub fn get_system_calls_persec(&self) -> Option<&u32> {
        self.system_calls_persec.as_ref()
    }

    /// Sets the value of SystemUpTime
    pub fn set_system_up_time(&mut self, value: u64) {
        self.system_up_time = Some(value);
    }

    /// Gets the value of SystemUpTime
    pub fn get_system_up_time(&self) -> Option<&u64> {
        self.system_up_time.as_ref()
    }

    /// Sets the value of Threads
    pub fn set_threads(&mut self, value: u32) {
        self.threads = Some(value);
    }

    /// Gets the value of Threads
    pub fn get_threads(&self) -> Option<&u32> {
        self.threads.as_ref()
    }
}

