// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_NumaNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_NumaNode {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// CreationClassName indicates the name of the class or the subclass used in the creation of an instance. When used with the other key properties of this class, this property allows all instances of this class and its subclasses to be uniquely identified.
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CurrentlyAssignedVirtualProcessors")]
    pub currently_assigned_virtual_processors: Option<u32>,

/// 
    #[serde(rename = "CurrentlyConsumableMemoryBlocks")]
    pub currently_consumable_memory_blocks: Option<u64>,

/// 
    #[serde(rename = "NodeID")]
    pub node_id: Option<String>,

/// 
    #[serde(rename = "NumberOfLogicalProcessors")]
    pub number_of_logical_processors: Option<u32>,

/// 
    #[serde(rename = "NumberOfProcessorCores")]
    pub number_of_processor_cores: Option<u32>,

/// The scoping System's CreationClassName.
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// The scoping System's Name.
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl Msvm_NumaNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            creation_class_name: None,
            currently_assigned_virtual_processors: None,
            currently_consumable_memory_blocks: None,
            node_id: None,
            number_of_logical_processors: None,
            number_of_processor_cores: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of CurrentlyAssignedVirtualProcessors
    pub fn set_currently_assigned_virtual_processors(&mut self, value: u32) {
        self.currently_assigned_virtual_processors = Some(value);
    }

    /// Gets the value of CurrentlyAssignedVirtualProcessors
    pub fn get_currently_assigned_virtual_processors(&self) -> Option<&u32> {
        self.currently_assigned_virtual_processors.as_ref()
    }

    /// Sets the value of CurrentlyConsumableMemoryBlocks
    pub fn set_currently_consumable_memory_blocks(&mut self, value: u64) {
        self.currently_consumable_memory_blocks = Some(value);
    }

    /// Gets the value of CurrentlyConsumableMemoryBlocks
    pub fn get_currently_consumable_memory_blocks(&self) -> Option<&u64> {
        self.currently_consumable_memory_blocks.as_ref()
    }

    /// Sets the value of NodeID
    pub fn set_node_id(&mut self, value: String) {
        self.node_id = Some(value);
    }

    /// Gets the value of NodeID
    pub fn get_node_id(&self) -> Option<&String> {
        self.node_id.as_ref()
    }

    /// Sets the value of NumberOfLogicalProcessors
    pub fn set_number_of_logical_processors(&mut self, value: u32) {
        self.number_of_logical_processors = Some(value);
    }

    /// Gets the value of NumberOfLogicalProcessors
    pub fn get_number_of_logical_processors(&self) -> Option<&u32> {
        self.number_of_logical_processors.as_ref()
    }

    /// Sets the value of NumberOfProcessorCores
    pub fn set_number_of_processor_cores(&mut self, value: u32) {
        self.number_of_processor_cores = Some(value);
    }

    /// Gets the value of NumberOfProcessorCores
    pub fn get_number_of_processor_cores(&self) -> Option<&u32> {
        self.number_of_processor_cores.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

impl Msvm_NumaNode {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_Memory object(s)
    pub fn get_related__memory(&self) -> Result<Msvm_Memory, WmiError> {
        self.get_related("Msvm_Memory")
    }

    /// Gets the related Msvm_Processor object(s)
    pub fn get_related__processor(&self) -> Result<Vec<Msvm_Processor>, WmiError> {
        self.get_all_related("Msvm_Processor")
    }

}

