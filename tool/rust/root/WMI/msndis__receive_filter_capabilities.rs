// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveFilterCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveFilterCapabilities {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "EnabledFilterTypes")]
    pub enabled_filter_types: Option<u32>,

/// 
    #[serde(rename = "EnabledQueueTypes")]
    pub enabled_queue_types: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "MaxLookaheadSplitSize")]
    pub max_lookahead_split_size: Option<u32>,

/// 
    #[serde(rename = "MaxMacHeaderFilters")]
    pub max_mac_header_filters: Option<u32>,

/// 
    #[serde(rename = "MaxQueueGroups")]
    pub max_queue_groups: Option<u32>,

/// 
    #[serde(rename = "MaxQueuesPerQueueGroup")]
    pub max_queues_per_queue_group: Option<u32>,

/// 
    #[serde(rename = "MinLookaheadSplitSize")]
    pub min_lookahead_split_size: Option<u32>,

/// 
    #[serde(rename = "NumQueues")]
    pub num_queues: Option<u32>,

/// 
    #[serde(rename = "SupportedFilterTests")]
    pub supported_filter_tests: Option<u32>,

/// 
    #[serde(rename = "SupportedHeaders")]
    pub supported_headers: Option<u32>,

/// 
    #[serde(rename = "SupportedMacHeaderFields")]
    pub supported_mac_header_fields: Option<u32>,

/// 
    #[serde(rename = "SupportedQueueProperties")]
    pub supported_queue_properties: Option<u32>,
}

impl MSNdis_ReceiveFilterCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            enabled_filter_types: None,
            enabled_queue_types: None,
            flags: None,
            header: None,
            max_lookahead_split_size: None,
            max_mac_header_filters: None,
            max_queue_groups: None,
            max_queues_per_queue_group: None,
            min_lookahead_split_size: None,
            num_queues: None,
            supported_filter_tests: None,
            supported_headers: None,
            supported_mac_header_fields: None,
            supported_queue_properties: None,
        }
    }


    /// Sets the value of EnabledFilterTypes
    pub fn set_enabled_filter_types(&mut self, value: u32) {
        self.enabled_filter_types = Some(value);
    }

    /// Gets the value of EnabledFilterTypes
    pub fn get_enabled_filter_types(&self) -> Option<&u32> {
        self.enabled_filter_types.as_ref()
    }

    /// Sets the value of EnabledQueueTypes
    pub fn set_enabled_queue_types(&mut self, value: u32) {
        self.enabled_queue_types = Some(value);
    }

    /// Gets the value of EnabledQueueTypes
    pub fn get_enabled_queue_types(&self) -> Option<&u32> {
        self.enabled_queue_types.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of MaxLookaheadSplitSize
    pub fn set_max_lookahead_split_size(&mut self, value: u32) {
        self.max_lookahead_split_size = Some(value);
    }

    /// Gets the value of MaxLookaheadSplitSize
    pub fn get_max_lookahead_split_size(&self) -> Option<&u32> {
        self.max_lookahead_split_size.as_ref()
    }

    /// Sets the value of MaxMacHeaderFilters
    pub fn set_max_mac_header_filters(&mut self, value: u32) {
        self.max_mac_header_filters = Some(value);
    }

    /// Gets the value of MaxMacHeaderFilters
    pub fn get_max_mac_header_filters(&self) -> Option<&u32> {
        self.max_mac_header_filters.as_ref()
    }

    /// Sets the value of MaxQueueGroups
    pub fn set_max_queue_groups(&mut self, value: u32) {
        self.max_queue_groups = Some(value);
    }

    /// Gets the value of MaxQueueGroups
    pub fn get_max_queue_groups(&self) -> Option<&u32> {
        self.max_queue_groups.as_ref()
    }

    /// Sets the value of MaxQueuesPerQueueGroup
    pub fn set_max_queues_per_queue_group(&mut self, value: u32) {
        self.max_queues_per_queue_group = Some(value);
    }

    /// Gets the value of MaxQueuesPerQueueGroup
    pub fn get_max_queues_per_queue_group(&self) -> Option<&u32> {
        self.max_queues_per_queue_group.as_ref()
    }

    /// Sets the value of MinLookaheadSplitSize
    pub fn set_min_lookahead_split_size(&mut self, value: u32) {
        self.min_lookahead_split_size = Some(value);
    }

    /// Gets the value of MinLookaheadSplitSize
    pub fn get_min_lookahead_split_size(&self) -> Option<&u32> {
        self.min_lookahead_split_size.as_ref()
    }

    /// Sets the value of NumQueues
    pub fn set_num_queues(&mut self, value: u32) {
        self.num_queues = Some(value);
    }

    /// Gets the value of NumQueues
    pub fn get_num_queues(&self) -> Option<&u32> {
        self.num_queues.as_ref()
    }

    /// Sets the value of SupportedFilterTests
    pub fn set_supported_filter_tests(&mut self, value: u32) {
        self.supported_filter_tests = Some(value);
    }

    /// Gets the value of SupportedFilterTests
    pub fn get_supported_filter_tests(&self) -> Option<&u32> {
        self.supported_filter_tests.as_ref()
    }

    /// Sets the value of SupportedHeaders
    pub fn set_supported_headers(&mut self, value: u32) {
        self.supported_headers = Some(value);
    }

    /// Gets the value of SupportedHeaders
    pub fn get_supported_headers(&self) -> Option<&u32> {
        self.supported_headers.as_ref()
    }

    /// Sets the value of SupportedMacHeaderFields
    pub fn set_supported_mac_header_fields(&mut self, value: u32) {
        self.supported_mac_header_fields = Some(value);
    }

    /// Gets the value of SupportedMacHeaderFields
    pub fn get_supported_mac_header_fields(&self) -> Option<&u32> {
        self.supported_mac_header_fields.as_ref()
    }

    /// Sets the value of SupportedQueueProperties
    pub fn set_supported_queue_properties(&mut self, value: u32) {
        self.supported_queue_properties = Some(value);
    }

    /// Gets the value of SupportedQueueProperties
    pub fn get_supported_queue_properties(&self) -> Option<&u32> {
        self.supported_queue_properties.as_ref()
    }
}

