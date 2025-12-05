// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidPartition {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "PhysicalPagesAllocated")]
    pub physical_pages_allocated: Option<u64>,

/// 
    #[serde(rename = "PreferredNUMANodeIndex")]
    pub preferred_numanode_index: Option<u64>,

/// 
    #[serde(rename = "RemotePhysicalPages")]
    pub remote_physical_pages: Option<u64>,
}

impl Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            physical_pages_allocated: None,
            preferred_numanode_index: None,
            remote_physical_pages: None,
        }
    }


    /// Sets the value of PhysicalPagesAllocated
    pub fn set_physical_pages_allocated(&mut self, value: u64) {
        self.physical_pages_allocated = Some(value);
    }

    /// Gets the value of PhysicalPagesAllocated
    pub fn get_physical_pages_allocated(&self) -> Option<&u64> {
        self.physical_pages_allocated.as_ref()
    }

    /// Sets the value of PreferredNUMANodeIndex
    pub fn set_preferred_numanode_index(&mut self, value: u64) {
        self.preferred_numanode_index = Some(value);
    }

    /// Gets the value of PreferredNUMANodeIndex
    pub fn get_preferred_numanode_index(&self) -> Option<&u64> {
        self.preferred_numanode_index.as_ref()
    }

    /// Sets the value of RemotePhysicalPages
    pub fn set_remote_physical_pages(&mut self, value: u64) {
        self.remote_physical_pages = Some(value);
    }

    /// Gets the value of RemotePhysicalPages
    pub fn get_remote_physical_pages(&self) -> Option<&u64> {
        self.remote_physical_pages.as_ref()
    }
}

