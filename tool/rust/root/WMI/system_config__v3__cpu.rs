// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V3_CPU struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V3_CPU {
    #[serde(flatten)]
    pub base: SystemConfig_V3,

/// 
    #[serde(rename = "AllocationGranularity")]
    pub allocation_granularity: Option<u32>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<char>,

/// 
    #[serde(rename = "DomainName")]
    pub domain_name: Vec<char>,

/// 
    #[serde(rename = "HighestUserAddress")]
    pub highest_user_address: Option<u32>,

/// 
    #[serde(rename = "HyperThreadingFlag")]
    pub hyper_threading_flag: Option<u32>,

/// 
    #[serde(rename = "MemorySpeed")]
    pub memory_speed: Option<u32>,

/// 
    #[serde(rename = "MemSize")]
    pub mem_size: Option<u32>,

/// 
    #[serde(rename = "MHz")]
    pub mhz: Option<u32>,

/// 
    #[serde(rename = "NumberOfProcessors")]
    pub number_of_processors: Option<u32>,

/// 
    #[serde(rename = "NxEnabled")]
    pub nx_enabled: Option<u8>,

/// 
    #[serde(rename = "PaeEnabled")]
    pub pae_enabled: Option<u8>,

/// 
    #[serde(rename = "PageSize")]
    pub page_size: Option<u32>,

/// 
    #[serde(rename = "ProcessorArchitecture")]
    pub processor_architecture: Option<u16>,

/// 
    #[serde(rename = "ProcessorLevel")]
    pub processor_level: Option<u16>,

/// 
    #[serde(rename = "ProcessorRevision")]
    pub processor_revision: Option<u16>,
}

impl SystemConfig_V3_CPU {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V3::new(),
            allocation_granularity: None,
            computer_name: Vec::new(),
            domain_name: Vec::new(),
            highest_user_address: None,
            hyper_threading_flag: None,
            memory_speed: None,
            mem_size: None,
            mhz: None,
            number_of_processors: None,
            nx_enabled: None,
            pae_enabled: None,
            page_size: None,
            processor_architecture: None,
            processor_level: None,
            processor_revision: None,
        }
    }


    /// Sets the value of AllocationGranularity
    pub fn set_allocation_granularity(&mut self, value: u32) {
        self.allocation_granularity = Some(value);
    }

    /// Gets the value of AllocationGranularity
    pub fn get_allocation_granularity(&self) -> Option<&u32> {
        self.allocation_granularity.as_ref()
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: Vec<char>) {
        self.computer_name = value;
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> &Vec<char> {
        &self.computer_name
    }

    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: Vec<char>) {
        self.domain_name = value;
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> &Vec<char> {
        &self.domain_name
    }

    /// Sets the value of HighestUserAddress
    pub fn set_highest_user_address(&mut self, value: u32) {
        self.highest_user_address = Some(value);
    }

    /// Gets the value of HighestUserAddress
    pub fn get_highest_user_address(&self) -> Option<&u32> {
        self.highest_user_address.as_ref()
    }

    /// Sets the value of HyperThreadingFlag
    pub fn set_hyper_threading_flag(&mut self, value: u32) {
        self.hyper_threading_flag = Some(value);
    }

    /// Gets the value of HyperThreadingFlag
    pub fn get_hyper_threading_flag(&self) -> Option<&u32> {
        self.hyper_threading_flag.as_ref()
    }

    /// Sets the value of MemorySpeed
    pub fn set_memory_speed(&mut self, value: u32) {
        self.memory_speed = Some(value);
    }

    /// Gets the value of MemorySpeed
    pub fn get_memory_speed(&self) -> Option<&u32> {
        self.memory_speed.as_ref()
    }

    /// Sets the value of MemSize
    pub fn set_mem_size(&mut self, value: u32) {
        self.mem_size = Some(value);
    }

    /// Gets the value of MemSize
    pub fn get_mem_size(&self) -> Option<&u32> {
        self.mem_size.as_ref()
    }

    /// Sets the value of MHz
    pub fn set_mhz(&mut self, value: u32) {
        self.mhz = Some(value);
    }

    /// Gets the value of MHz
    pub fn get_mhz(&self) -> Option<&u32> {
        self.mhz.as_ref()
    }

    /// Sets the value of NumberOfProcessors
    pub fn set_number_of_processors(&mut self, value: u32) {
        self.number_of_processors = Some(value);
    }

    /// Gets the value of NumberOfProcessors
    pub fn get_number_of_processors(&self) -> Option<&u32> {
        self.number_of_processors.as_ref()
    }

    /// Sets the value of NxEnabled
    pub fn set_nx_enabled(&mut self, value: u8) {
        self.nx_enabled = Some(value);
    }

    /// Gets the value of NxEnabled
    pub fn get_nx_enabled(&self) -> Option<&u8> {
        self.nx_enabled.as_ref()
    }

    /// Sets the value of PaeEnabled
    pub fn set_pae_enabled(&mut self, value: u8) {
        self.pae_enabled = Some(value);
    }

    /// Gets the value of PaeEnabled
    pub fn get_pae_enabled(&self) -> Option<&u8> {
        self.pae_enabled.as_ref()
    }

    /// Sets the value of PageSize
    pub fn set_page_size(&mut self, value: u32) {
        self.page_size = Some(value);
    }

    /// Gets the value of PageSize
    pub fn get_page_size(&self) -> Option<&u32> {
        self.page_size.as_ref()
    }

    /// Sets the value of ProcessorArchitecture
    pub fn set_processor_architecture(&mut self, value: u16) {
        self.processor_architecture = Some(value);
    }

    /// Gets the value of ProcessorArchitecture
    pub fn get_processor_architecture(&self) -> Option<&u16> {
        self.processor_architecture.as_ref()
    }

    /// Sets the value of ProcessorLevel
    pub fn set_processor_level(&mut self, value: u16) {
        self.processor_level = Some(value);
    }

    /// Gets the value of ProcessorLevel
    pub fn get_processor_level(&self) -> Option<&u16> {
        self.processor_level.as_ref()
    }

    /// Sets the value of ProcessorRevision
    pub fn set_processor_revision(&mut self, value: u16) {
        self.processor_revision = Some(value);
    }

    /// Gets the value of ProcessorRevision
    pub fn get_processor_revision(&self) -> Option<&u16> {
        self.processor_revision.as_ref()
    }
}

