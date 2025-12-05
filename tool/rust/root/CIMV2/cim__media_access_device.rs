// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MediaAccessDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MediaAccessDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<u16>,

/// 
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// 
    #[serde(rename = "CompressionMethod")]
    pub compression_method: Option<String>,

/// 
    #[serde(rename = "DefaultBlockSize")]
    pub default_block_size: Option<u64>,

/// 
    #[serde(rename = "ErrorMethodology")]
    pub error_methodology: Option<String>,

/// 
    #[serde(rename = "MaxBlockSize")]
    pub max_block_size: Option<u64>,

/// 
    #[serde(rename = "MaxMediaSize")]
    pub max_media_size: Option<u64>,

/// 
    #[serde(rename = "MinBlockSize")]
    pub min_block_size: Option<u64>,

/// 
    #[serde(rename = "NeedsCleaning")]
    pub needs_cleaning: Option<bool>,

/// 
    #[serde(rename = "NumberOfMediaSupported")]
    pub number_of_media_supported: Option<u32>,
}

impl CIM_MediaAccessDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            compression_method: None,
            default_block_size: None,
            error_methodology: None,
            max_block_size: None,
            max_media_size: None,
            min_block_size: None,
            needs_cleaning: None,
            number_of_media_supported: None,
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: Vec<u16>) {
        self.capabilities = value;
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> &Vec<u16> {
        &self.capabilities
    }

    /// Sets the value of CapabilityDescriptions
    pub fn set_capability_descriptions(&mut self, value: Vec<String>) {
        self.capability_descriptions = value;
    }

    /// Gets the value of CapabilityDescriptions
    pub fn get_capability_descriptions(&self) -> &Vec<String> {
        &self.capability_descriptions
    }

    /// Sets the value of CompressionMethod
    pub fn set_compression_method(&mut self, value: String) {
        self.compression_method = Some(value);
    }

    /// Gets the value of CompressionMethod
    pub fn get_compression_method(&self) -> Option<&String> {
        self.compression_method.as_ref()
    }

    /// Sets the value of DefaultBlockSize
    pub fn set_default_block_size(&mut self, value: u64) {
        self.default_block_size = Some(value);
    }

    /// Gets the value of DefaultBlockSize
    pub fn get_default_block_size(&self) -> Option<&u64> {
        self.default_block_size.as_ref()
    }

    /// Sets the value of ErrorMethodology
    pub fn set_error_methodology(&mut self, value: String) {
        self.error_methodology = Some(value);
    }

    /// Gets the value of ErrorMethodology
    pub fn get_error_methodology(&self) -> Option<&String> {
        self.error_methodology.as_ref()
    }

    /// Sets the value of MaxBlockSize
    pub fn set_max_block_size(&mut self, value: u64) {
        self.max_block_size = Some(value);
    }

    /// Gets the value of MaxBlockSize
    pub fn get_max_block_size(&self) -> Option<&u64> {
        self.max_block_size.as_ref()
    }

    /// Sets the value of MaxMediaSize
    pub fn set_max_media_size(&mut self, value: u64) {
        self.max_media_size = Some(value);
    }

    /// Gets the value of MaxMediaSize
    pub fn get_max_media_size(&self) -> Option<&u64> {
        self.max_media_size.as_ref()
    }

    /// Sets the value of MinBlockSize
    pub fn set_min_block_size(&mut self, value: u64) {
        self.min_block_size = Some(value);
    }

    /// Gets the value of MinBlockSize
    pub fn get_min_block_size(&self) -> Option<&u64> {
        self.min_block_size.as_ref()
    }

    /// Sets the value of NeedsCleaning
    pub fn set_needs_cleaning(&mut self, value: bool) {
        self.needs_cleaning = Some(value);
    }

    /// Gets the value of NeedsCleaning
    pub fn get_needs_cleaning(&self) -> Option<&bool> {
        self.needs_cleaning.as_ref()
    }

    /// Sets the value of NumberOfMediaSupported
    pub fn set_number_of_media_supported(&mut self, value: u32) {
        self.number_of_media_supported = Some(value);
    }

    /// Gets the value of NumberOfMediaSupported
    pub fn get_number_of_media_supported(&self) -> Option<&u32> {
        self.number_of_media_supported.as_ref()
    }
}

