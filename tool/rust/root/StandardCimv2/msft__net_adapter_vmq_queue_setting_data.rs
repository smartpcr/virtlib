// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterVmqQueueSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterVmqQueueSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "FilterList")]
    pub filter_list: Vec<MSFT_NetAdapter_VmqFilter>,

/// 
    #[serde(rename = "NumFilters")]
    pub num_filters: Option<u32>,

/// 
    #[serde(rename = "ProcessorAffinityMask")]
    pub processor_affinity_mask: Option<u64>,

/// 
    #[serde(rename = "ProcessorGroup")]
    pub processor_group: Option<u16>,

/// 
    #[serde(rename = "QueueID")]
    pub queue_id: Option<u32>,

/// 
    #[serde(rename = "QueueName")]
    pub queue_name: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "VmFriendlyName")]
    pub vm_friendly_name: Option<String>,

/// 
    #[serde(rename = "VmID")]
    pub vm_id: Option<String>,
}

impl MSFT_NetAdapterVmqQueueSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            filter_list: Vec::new(),
            num_filters: None,
            processor_affinity_mask: None,
            processor_group: None,
            queue_id: None,
            queue_name: None,
            state: None,
            vm_friendly_name: None,
            vm_id: None,
        }
    }


    /// Sets the value of FilterList
    pub fn set_filter_list(&mut self, value: Vec<MSFT_NetAdapter_VmqFilter>) {
        self.filter_list = value;
    }

    /// Gets the value of FilterList
    pub fn get_filter_list(&self) -> &Vec<MSFT_NetAdapter_VmqFilter> {
        &self.filter_list
    }

    /// Sets the value of NumFilters
    pub fn set_num_filters(&mut self, value: u32) {
        self.num_filters = Some(value);
    }

    /// Gets the value of NumFilters
    pub fn get_num_filters(&self) -> Option<&u32> {
        self.num_filters.as_ref()
    }

    /// Sets the value of ProcessorAffinityMask
    pub fn set_processor_affinity_mask(&mut self, value: u64) {
        self.processor_affinity_mask = Some(value);
    }

    /// Gets the value of ProcessorAffinityMask
    pub fn get_processor_affinity_mask(&self) -> Option<&u64> {
        self.processor_affinity_mask.as_ref()
    }

    /// Sets the value of ProcessorGroup
    pub fn set_processor_group(&mut self, value: u16) {
        self.processor_group = Some(value);
    }

    /// Gets the value of ProcessorGroup
    pub fn get_processor_group(&self) -> Option<&u16> {
        self.processor_group.as_ref()
    }

    /// Sets the value of QueueID
    pub fn set_queue_id(&mut self, value: u32) {
        self.queue_id = Some(value);
    }

    /// Gets the value of QueueID
    pub fn get_queue_id(&self) -> Option<&u32> {
        self.queue_id.as_ref()
    }

    /// Sets the value of QueueName
    pub fn set_queue_name(&mut self, value: String) {
        self.queue_name = Some(value);
    }

    /// Gets the value of QueueName
    pub fn get_queue_name(&self) -> Option<&String> {
        self.queue_name.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of VmFriendlyName
    pub fn set_vm_friendly_name(&mut self, value: String) {
        self.vm_friendly_name = Some(value);
    }

    /// Gets the value of VmFriendlyName
    pub fn get_vm_friendly_name(&self) -> Option<&String> {
        self.vm_friendly_name.as_ref()
    }

    /// Sets the value of VmID
    pub fn set_vm_id(&mut self, value: String) {
        self.vm_id = Some(value);
    }

    /// Gets the value of VmID
    pub fn get_vm_id(&self) -> Option<&String> {
        self.vm_id.as_ref()
    }
}

