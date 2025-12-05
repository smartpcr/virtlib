// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterVPortSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterVPortSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "FilterList")]
    pub filter_list: Vec<MSFT_NetAdapter_VmqFilter>,

/// 
    #[serde(rename = "FunctionID")]
    pub function_id: Option<u16>,

/// 
    #[serde(rename = "InterruptModeration")]
    pub interrupt_moderation: Option<u32>,

/// 
    #[serde(rename = "NumFilters")]
    pub num_filters: Option<u32>,

/// 
    #[serde(rename = "NumQueuePairs")]
    pub num_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "ProcessorAffinityMask")]
    pub processor_affinity_mask: Option<u64>,

/// 
    #[serde(rename = "ProcessorGroup")]
    pub processor_group: Option<u16>,

/// 
    #[serde(rename = "SwitchID")]
    pub switch_id: Option<u32>,

/// 
    #[serde(rename = "VPortID")]
    pub vport_id: Option<u32>,

/// 
    #[serde(rename = "VPortName")]
    pub vport_name: Option<String>,

/// 
    #[serde(rename = "VPortState")]
    pub vport_state: Option<u32>,
}

impl MSFT_NetAdapterVPortSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            filter_list: Vec::new(),
            function_id: None,
            interrupt_moderation: None,
            num_filters: None,
            num_queue_pairs: None,
            processor_affinity_mask: None,
            processor_group: None,
            switch_id: None,
            vport_id: None,
            vport_name: None,
            vport_state: None,
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

    /// Sets the value of FunctionID
    pub fn set_function_id(&mut self, value: u16) {
        self.function_id = Some(value);
    }

    /// Gets the value of FunctionID
    pub fn get_function_id(&self) -> Option<&u16> {
        self.function_id.as_ref()
    }

    /// Sets the value of InterruptModeration
    pub fn set_interrupt_moderation(&mut self, value: u32) {
        self.interrupt_moderation = Some(value);
    }

    /// Gets the value of InterruptModeration
    pub fn get_interrupt_moderation(&self) -> Option<&u32> {
        self.interrupt_moderation.as_ref()
    }

    /// Sets the value of NumFilters
    pub fn set_num_filters(&mut self, value: u32) {
        self.num_filters = Some(value);
    }

    /// Gets the value of NumFilters
    pub fn get_num_filters(&self) -> Option<&u32> {
        self.num_filters.as_ref()
    }

    /// Sets the value of NumQueuePairs
    pub fn set_num_queue_pairs(&mut self, value: u32) {
        self.num_queue_pairs = Some(value);
    }

    /// Gets the value of NumQueuePairs
    pub fn get_num_queue_pairs(&self) -> Option<&u32> {
        self.num_queue_pairs.as_ref()
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

    /// Sets the value of SwitchID
    pub fn set_switch_id(&mut self, value: u32) {
        self.switch_id = Some(value);
    }

    /// Gets the value of SwitchID
    pub fn get_switch_id(&self) -> Option<&u32> {
        self.switch_id.as_ref()
    }

    /// Sets the value of VPortID
    pub fn set_vport_id(&mut self, value: u32) {
        self.vport_id = Some(value);
    }

    /// Gets the value of VPortID
    pub fn get_vport_id(&self) -> Option<&u32> {
        self.vport_id.as_ref()
    }

    /// Sets the value of VPortName
    pub fn set_vport_name(&mut self, value: String) {
        self.vport_name = Some(value);
    }

    /// Gets the value of VPortName
    pub fn get_vport_name(&self) -> Option<&String> {
        self.vport_name.as_ref()
    }

    /// Sets the value of VPortState
    pub fn set_vport_state(&mut self, value: u32) {
        self.vport_state = Some(value);
    }

    /// Gets the value of VPortState
    pub fn get_vport_state(&self) -> Option<&u32> {
        self.vport_state.as_ref()
    }
}

