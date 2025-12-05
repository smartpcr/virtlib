// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_NumaNodeTopology struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_NumaNodeTopology {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CountOfMemoryBlocks")]
    pub count_of_memory_blocks: Option<u64>,

/// 
    #[serde(rename = "CountOfProcessors")]
    pub count_of_processors: Option<u32>,

/// 
    #[serde(rename = "MemoryAccessTrackingPolicy")]
    pub memory_access_tracking_policy: Option<u8>,

/// 
    #[serde(rename = "MemoryAccessTrackingState")]
    pub memory_access_tracking_state: Option<u8>,

/// 
    #[serde(rename = "MemoryBackingType")]
    pub memory_backing_type: Option<u8>,

/// 
    #[serde(rename = "PhysicalNodeNumber")]
    pub physical_node_number: Option<u32>,

/// 
    #[serde(rename = "VirtualNodeNumber")]
    pub virtual_node_number: Option<u32>,

/// 
    #[serde(rename = "VirtualSocketNumber")]
    pub virtual_socket_number: Option<u32>,
}

impl Msvm_NumaNodeTopology {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            count_of_memory_blocks: None,
            count_of_processors: None,
            memory_access_tracking_policy: None,
            memory_access_tracking_state: None,
            memory_backing_type: None,
            physical_node_number: None,
            virtual_node_number: None,
            virtual_socket_number: None,
        }
    }


    /// Sets the value of CountOfMemoryBlocks
    pub fn set_count_of_memory_blocks(&mut self, value: u64) {
        self.count_of_memory_blocks = Some(value);
    }

    /// Gets the value of CountOfMemoryBlocks
    pub fn get_count_of_memory_blocks(&self) -> Option<&u64> {
        self.count_of_memory_blocks.as_ref()
    }

    /// Sets the value of CountOfProcessors
    pub fn set_count_of_processors(&mut self, value: u32) {
        self.count_of_processors = Some(value);
    }

    /// Gets the value of CountOfProcessors
    pub fn get_count_of_processors(&self) -> Option<&u32> {
        self.count_of_processors.as_ref()
    }

    /// Sets the value of MemoryAccessTrackingPolicy
    pub fn set_memory_access_tracking_policy(&mut self, value: u8) {
        self.memory_access_tracking_policy = Some(value);
    }

    /// Gets the value of MemoryAccessTrackingPolicy
    pub fn get_memory_access_tracking_policy(&self) -> Option<&u8> {
        self.memory_access_tracking_policy.as_ref()
    }

    /// Sets the value of MemoryAccessTrackingState
    pub fn set_memory_access_tracking_state(&mut self, value: u8) {
        self.memory_access_tracking_state = Some(value);
    }

    /// Gets the value of MemoryAccessTrackingState
    pub fn get_memory_access_tracking_state(&self) -> Option<&u8> {
        self.memory_access_tracking_state.as_ref()
    }

    /// Sets the value of MemoryBackingType
    pub fn set_memory_backing_type(&mut self, value: u8) {
        self.memory_backing_type = Some(value);
    }

    /// Gets the value of MemoryBackingType
    pub fn get_memory_backing_type(&self) -> Option<&u8> {
        self.memory_backing_type.as_ref()
    }

    /// Sets the value of PhysicalNodeNumber
    pub fn set_physical_node_number(&mut self, value: u32) {
        self.physical_node_number = Some(value);
    }

    /// Gets the value of PhysicalNodeNumber
    pub fn get_physical_node_number(&self) -> Option<&u32> {
        self.physical_node_number.as_ref()
    }

    /// Sets the value of VirtualNodeNumber
    pub fn set_virtual_node_number(&mut self, value: u32) {
        self.virtual_node_number = Some(value);
    }

    /// Gets the value of VirtualNodeNumber
    pub fn get_virtual_node_number(&self) -> Option<&u32> {
        self.virtual_node_number.as_ref()
    }

    /// Sets the value of VirtualSocketNumber
    pub fn set_virtual_socket_number(&mut self, value: u32) {
        self.virtual_socket_number = Some(value);
    }

    /// Gets the value of VirtualSocketNumber
    pub fn get_virtual_socket_number(&self) -> Option<&u32> {
        self.virtual_socket_number.as_ref()
    }
}

