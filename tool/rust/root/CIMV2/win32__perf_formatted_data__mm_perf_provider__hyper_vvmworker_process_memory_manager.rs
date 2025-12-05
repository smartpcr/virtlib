// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_MmPerfProvider_HyperVVMWorkerProcessMemoryManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_MmPerfProvider_HyperVVMWorkerProcessMemoryManager {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "MemoryBlockCount")]
    pub memory_block_count: Option<u64>,
}

impl Win32_PerfFormattedData_MmPerfProvider_HyperVVMWorkerProcessMemoryManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            memory_block_count: None,
        }
    }


    /// Sets the value of MemoryBlockCount
    pub fn set_memory_block_count(&mut self, value: u64) {
        self.memory_block_count = Some(value);
    }

    /// Gets the value of MemoryBlockCount
    pub fn get_memory_block_count(&self) -> Option<&u64> {
        self.memory_block_count.as_ref()
    }
}

