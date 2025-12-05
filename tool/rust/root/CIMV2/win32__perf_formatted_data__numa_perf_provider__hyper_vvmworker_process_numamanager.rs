// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NumaPerfProvider_HyperVVMWorkerProcessNUMAManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NumaPerfProvider_HyperVVMWorkerProcessNUMAManager {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "InitialMemoryAssignedPerNodeMB")]
    pub initial_memory_assigned_per_node_mb: Option<u64>,

/// 
    #[serde(rename = "MappedPhysicalNUMANode")]
    pub mapped_physical_numanode: Option<u64>,

/// 
    #[serde(rename = "NUMASpanningAllowed")]
    pub numaspanning_allowed: Option<u32>,

/// 
    #[serde(rename = "VirtualProcessorCountPerNode")]
    pub virtual_processor_count_per_node: Option<u64>,
}

impl Win32_PerfFormattedData_NumaPerfProvider_HyperVVMWorkerProcessNUMAManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            initial_memory_assigned_per_node_mb: None,
            mapped_physical_numanode: None,
            numaspanning_allowed: None,
            virtual_processor_count_per_node: None,
        }
    }


    /// Sets the value of InitialMemoryAssignedPerNodeMB
    pub fn set_initial_memory_assigned_per_node_mb(&mut self, value: u64) {
        self.initial_memory_assigned_per_node_mb = Some(value);
    }

    /// Gets the value of InitialMemoryAssignedPerNodeMB
    pub fn get_initial_memory_assigned_per_node_mb(&self) -> Option<&u64> {
        self.initial_memory_assigned_per_node_mb.as_ref()
    }

    /// Sets the value of MappedPhysicalNUMANode
    pub fn set_mapped_physical_numanode(&mut self, value: u64) {
        self.mapped_physical_numanode = Some(value);
    }

    /// Gets the value of MappedPhysicalNUMANode
    pub fn get_mapped_physical_numanode(&self) -> Option<&u64> {
        self.mapped_physical_numanode.as_ref()
    }

    /// Sets the value of NUMASpanningAllowed
    pub fn set_numaspanning_allowed(&mut self, value: u32) {
        self.numaspanning_allowed = Some(value);
    }

    /// Gets the value of NUMASpanningAllowed
    pub fn get_numaspanning_allowed(&self) -> Option<&u32> {
        self.numaspanning_allowed.as_ref()
    }

    /// Sets the value of VirtualProcessorCountPerNode
    pub fn set_virtual_processor_count_per_node(&mut self, value: u64) {
        self.virtual_processor_count_per_node = Some(value);
    }

    /// Gets the value of VirtualProcessorCountPerNode
    pub fn get_virtual_processor_count_per_node(&self) -> Option<&u64> {
        self.virtual_processor_count_per_node.as_ref()
    }
}

