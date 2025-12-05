// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Processor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Processor {
    #[serde(flatten)]
    pub base: CIM_Processor,

/// 
    #[serde(rename = "LoadPercentageHistory")]
    pub load_percentage_history: Vec<u16>,
}

impl Msvm_Processor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Processor::new(),
            load_percentage_history: Vec::new(),
        }
    }


    /// Sets the value of LoadPercentageHistory
    pub fn set_load_percentage_history(&mut self, value: Vec<u16>) {
        self.load_percentage_history = value;
    }

    /// Gets the value of LoadPercentageHistory
    pub fn get_load_percentage_history(&self) -> &Vec<u16> {
        &self.load_percentage_history
    }
}

impl Msvm_Processor {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_NumaNode object(s)
    pub fn get_related__numa_node(&self) -> Result<Msvm_NumaNode, WmiError> {
        self.get_related("Msvm_NumaNode")
    }

    /// Gets the related Msvm_ProcessorPool object(s)
    pub fn get_related__processor_pool(&self) -> Result<Msvm_ProcessorPool, WmiError> {
        self.get_related("Msvm_ProcessorPool")
    }

}

