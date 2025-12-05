// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_StorageExtent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_StorageExtent {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "Access")]
    pub access: Option<u16>,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<u64>,

/// 
    #[serde(rename = "ErrorMethodology")]
    pub error_methodology: Option<String>,

/// 
    #[serde(rename = "NumberOfBlocks")]
    pub number_of_blocks: Option<u64>,

/// 
    #[serde(rename = "Purpose")]
    pub purpose: Option<String>,
}

impl CIM_StorageExtent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            access: None,
            block_size: None,
            error_methodology: None,
            number_of_blocks: None,
            purpose: None,
        }
    }


    /// Sets the value of Access
    pub fn set_access(&mut self, value: u16) {
        self.access = Some(value);
    }

    /// Gets the value of Access
    pub fn get_access(&self) -> Option<&u16> {
        self.access.as_ref()
    }

    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: u64) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&u64> {
        self.block_size.as_ref()
    }

    /// Sets the value of ErrorMethodology
    pub fn set_error_methodology(&mut self, value: String) {
        self.error_methodology = Some(value);
    }

    /// Gets the value of ErrorMethodology
    pub fn get_error_methodology(&self) -> Option<&String> {
        self.error_methodology.as_ref()
    }

    /// Sets the value of NumberOfBlocks
    pub fn set_number_of_blocks(&mut self, value: u64) {
        self.number_of_blocks = Some(value);
    }

    /// Gets the value of NumberOfBlocks
    pub fn get_number_of_blocks(&self) -> Option<&u64> {
        self.number_of_blocks.as_ref()
    }

    /// Sets the value of Purpose
    pub fn set_purpose(&mut self, value: String) {
        self.purpose = Some(value);
    }

    /// Gets the value of Purpose
    pub fn get_purpose(&self) -> Option<&String> {
        self.purpose.as_ref()
    }
}

