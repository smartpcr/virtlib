// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalMemory {
    #[serde(flatten)]
    pub base: CIM_Chip,

/// 
    #[serde(rename = "BankLabel")]
    pub bank_label: Option<String>,

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// 
    #[serde(rename = "DataWidth")]
    pub data_width: Option<u16>,

/// 
    #[serde(rename = "InterleavePosition")]
    pub interleave_position: Option<u32>,

/// 
    #[serde(rename = "MemoryType")]
    pub memory_type: Option<u16>,

/// 
    #[serde(rename = "PositionInRow")]
    pub position_in_row: Option<u32>,

/// 
    #[serde(rename = "Speed")]
    pub speed: Option<u32>,

/// 
    #[serde(rename = "TotalWidth")]
    pub total_width: Option<u16>,
}

impl CIM_PhysicalMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Chip::new(),
            bank_label: None,
            capacity: None,
            data_width: None,
            interleave_position: None,
            memory_type: None,
            position_in_row: None,
            speed: None,
            total_width: None,
        }
    }


    /// Sets the value of BankLabel
    pub fn set_bank_label(&mut self, value: String) {
        self.bank_label = Some(value);
    }

    /// Gets the value of BankLabel
    pub fn get_bank_label(&self) -> Option<&String> {
        self.bank_label.as_ref()
    }

    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of DataWidth
    pub fn set_data_width(&mut self, value: u16) {
        self.data_width = Some(value);
    }

    /// Gets the value of DataWidth
    pub fn get_data_width(&self) -> Option<&u16> {
        self.data_width.as_ref()
    }

    /// Sets the value of InterleavePosition
    pub fn set_interleave_position(&mut self, value: u32) {
        self.interleave_position = Some(value);
    }

    /// Gets the value of InterleavePosition
    pub fn get_interleave_position(&self) -> Option<&u32> {
        self.interleave_position.as_ref()
    }

    /// Sets the value of MemoryType
    pub fn set_memory_type(&mut self, value: u16) {
        self.memory_type = Some(value);
    }

    /// Gets the value of MemoryType
    pub fn get_memory_type(&self) -> Option<&u16> {
        self.memory_type.as_ref()
    }

    /// Sets the value of PositionInRow
    pub fn set_position_in_row(&mut self, value: u32) {
        self.position_in_row = Some(value);
    }

    /// Gets the value of PositionInRow
    pub fn get_position_in_row(&self) -> Option<&u32> {
        self.position_in_row.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u32) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u32> {
        self.speed.as_ref()
    }

    /// Sets the value of TotalWidth
    pub fn set_total_width(&mut self, value: u16) {
        self.total_width = Some(value);
    }

    /// Gets the value of TotalWidth
    pub fn get_total_width(&self) -> Option<&u16> {
        self.total_width.as_ref()
    }
}

