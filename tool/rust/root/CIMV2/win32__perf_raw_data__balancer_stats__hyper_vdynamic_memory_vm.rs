// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_BalancerStats_HyperVDynamicMemoryVM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_BalancerStats_HyperVDynamicMemoryVM {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AddedMemory")]
    pub added_memory: Option<u64>,

/// 
    #[serde(rename = "AveragePressure")]
    pub average_pressure: Option<u32>,

/// 
    #[serde(rename = "CurrentPressure")]
    pub current_pressure: Option<u32>,

/// 
    #[serde(rename = "GuestAvailableMemory")]
    pub guest_available_memory: Option<u32>,

/// 
    #[serde(rename = "GuestVisiblePhysicalMemory")]
    pub guest_visible_physical_memory: Option<u32>,

/// 
    #[serde(rename = "MaximumPressure")]
    pub maximum_pressure: Option<u32>,

/// 
    #[serde(rename = "MemoryAddOperations")]
    pub memory_add_operations: Option<u64>,

/// 
    #[serde(rename = "MemoryRemoveOperations")]
    pub memory_remove_operations: Option<u64>,

/// 
    #[serde(rename = "MinimumPressure")]
    pub minimum_pressure: Option<u32>,

/// 
    #[serde(rename = "PhysicalMemory")]
    pub physical_memory: Option<u32>,

/// 
    #[serde(rename = "RemovedMemory")]
    pub removed_memory: Option<u64>,

/// 
    #[serde(rename = "SmartPagingWorkingSetSize")]
    pub smart_paging_working_set_size: Option<u32>,
}

impl Win32_PerfRawData_BalancerStats_HyperVDynamicMemoryVM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            added_memory: None,
            average_pressure: None,
            current_pressure: None,
            guest_available_memory: None,
            guest_visible_physical_memory: None,
            maximum_pressure: None,
            memory_add_operations: None,
            memory_remove_operations: None,
            minimum_pressure: None,
            physical_memory: None,
            removed_memory: None,
            smart_paging_working_set_size: None,
        }
    }


    /// Sets the value of AddedMemory
    pub fn set_added_memory(&mut self, value: u64) {
        self.added_memory = Some(value);
    }

    /// Gets the value of AddedMemory
    pub fn get_added_memory(&self) -> Option<&u64> {
        self.added_memory.as_ref()
    }

    /// Sets the value of AveragePressure
    pub fn set_average_pressure(&mut self, value: u32) {
        self.average_pressure = Some(value);
    }

    /// Gets the value of AveragePressure
    pub fn get_average_pressure(&self) -> Option<&u32> {
        self.average_pressure.as_ref()
    }

    /// Sets the value of CurrentPressure
    pub fn set_current_pressure(&mut self, value: u32) {
        self.current_pressure = Some(value);
    }

    /// Gets the value of CurrentPressure
    pub fn get_current_pressure(&self) -> Option<&u32> {
        self.current_pressure.as_ref()
    }

    /// Sets the value of GuestAvailableMemory
    pub fn set_guest_available_memory(&mut self, value: u32) {
        self.guest_available_memory = Some(value);
    }

    /// Gets the value of GuestAvailableMemory
    pub fn get_guest_available_memory(&self) -> Option<&u32> {
        self.guest_available_memory.as_ref()
    }

    /// Sets the value of GuestVisiblePhysicalMemory
    pub fn set_guest_visible_physical_memory(&mut self, value: u32) {
        self.guest_visible_physical_memory = Some(value);
    }

    /// Gets the value of GuestVisiblePhysicalMemory
    pub fn get_guest_visible_physical_memory(&self) -> Option<&u32> {
        self.guest_visible_physical_memory.as_ref()
    }

    /// Sets the value of MaximumPressure
    pub fn set_maximum_pressure(&mut self, value: u32) {
        self.maximum_pressure = Some(value);
    }

    /// Gets the value of MaximumPressure
    pub fn get_maximum_pressure(&self) -> Option<&u32> {
        self.maximum_pressure.as_ref()
    }

    /// Sets the value of MemoryAddOperations
    pub fn set_memory_add_operations(&mut self, value: u64) {
        self.memory_add_operations = Some(value);
    }

    /// Gets the value of MemoryAddOperations
    pub fn get_memory_add_operations(&self) -> Option<&u64> {
        self.memory_add_operations.as_ref()
    }

    /// Sets the value of MemoryRemoveOperations
    pub fn set_memory_remove_operations(&mut self, value: u64) {
        self.memory_remove_operations = Some(value);
    }

    /// Gets the value of MemoryRemoveOperations
    pub fn get_memory_remove_operations(&self) -> Option<&u64> {
        self.memory_remove_operations.as_ref()
    }

    /// Sets the value of MinimumPressure
    pub fn set_minimum_pressure(&mut self, value: u32) {
        self.minimum_pressure = Some(value);
    }

    /// Gets the value of MinimumPressure
    pub fn get_minimum_pressure(&self) -> Option<&u32> {
        self.minimum_pressure.as_ref()
    }

    /// Sets the value of PhysicalMemory
    pub fn set_physical_memory(&mut self, value: u32) {
        self.physical_memory = Some(value);
    }

    /// Gets the value of PhysicalMemory
    pub fn get_physical_memory(&self) -> Option<&u32> {
        self.physical_memory.as_ref()
    }

    /// Sets the value of RemovedMemory
    pub fn set_removed_memory(&mut self, value: u64) {
        self.removed_memory = Some(value);
    }

    /// Gets the value of RemovedMemory
    pub fn get_removed_memory(&self) -> Option<&u64> {
        self.removed_memory.as_ref()
    }

    /// Sets the value of SmartPagingWorkingSetSize
    pub fn set_smart_paging_working_set_size(&mut self, value: u32) {
        self.smart_paging_working_set_size = Some(value);
    }

    /// Gets the value of SmartPagingWorkingSetSize
    pub fn get_smart_paging_working_set_size(&self) -> Option<&u32> {
        self.smart_paging_working_set_size.as_ref()
    }
}

