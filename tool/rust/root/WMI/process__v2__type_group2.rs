// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_V2_TypeGroup2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_V2_TypeGroup2 {
    #[serde(flatten)]
    pub base: Process_V2,

/// 
    #[serde(rename = "HandleCount")]
    pub handle_count: Option<u32>,

/// 
    #[serde(rename = "PageFaultCount")]
    pub page_fault_count: Option<u32>,

/// 
    #[serde(rename = "PagefileUsage")]
    pub pagefile_usage: Option<serde_json::Value>,

/// 
    #[serde(rename = "PeakPagefileUsage")]
    pub peak_pagefile_usage: Option<serde_json::Value>,

/// 
    #[serde(rename = "PeakVirtualSize")]
    pub peak_virtual_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "PeakWorkingSetSize")]
    pub peak_working_set_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "PrivatePageCount")]
    pub private_page_count: Option<serde_json::Value>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "QuotaNonPagedPoolUsage")]
    pub quota_non_paged_pool_usage: Option<serde_json::Value>,

/// 
    #[serde(rename = "QuotaPagedPoolUsage")]
    pub quota_paged_pool_usage: Option<serde_json::Value>,

/// 
    #[serde(rename = "QuotaPeakNonPagedPoolUsage")]
    pub quota_peak_non_paged_pool_usage: Option<serde_json::Value>,

/// 
    #[serde(rename = "QuotaPeakPagedPoolUsage")]
    pub quota_peak_paged_pool_usage: Option<serde_json::Value>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// 
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "WorkingSetSize")]
    pub working_set_size: Option<serde_json::Value>,
}

impl Process_V2_TypeGroup2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V2::new(),
            handle_count: None,
            page_fault_count: None,
            pagefile_usage: None,
            peak_pagefile_usage: None,
            peak_virtual_size: None,
            peak_working_set_size: None,
            private_page_count: None,
            process_id: None,
            quota_non_paged_pool_usage: None,
            quota_paged_pool_usage: None,
            quota_peak_non_paged_pool_usage: None,
            quota_peak_paged_pool_usage: None,
            reserved: None,
            virtual_size: None,
            working_set_size: None,
        }
    }


    /// Sets the value of HandleCount
    pub fn set_handle_count(&mut self, value: u32) {
        self.handle_count = Some(value);
    }

    /// Gets the value of HandleCount
    pub fn get_handle_count(&self) -> Option<&u32> {
        self.handle_count.as_ref()
    }

    /// Sets the value of PageFaultCount
    pub fn set_page_fault_count(&mut self, value: u32) {
        self.page_fault_count = Some(value);
    }

    /// Gets the value of PageFaultCount
    pub fn get_page_fault_count(&self) -> Option<&u32> {
        self.page_fault_count.as_ref()
    }

    /// Sets the value of PagefileUsage
    pub fn set_pagefile_usage(&mut self, value: serde_json::Value) {
        self.pagefile_usage = Some(value);
    }

    /// Gets the value of PagefileUsage
    pub fn get_pagefile_usage(&self) -> Option<&serde_json::Value> {
        self.pagefile_usage.as_ref()
    }

    /// Sets the value of PeakPagefileUsage
    pub fn set_peak_pagefile_usage(&mut self, value: serde_json::Value) {
        self.peak_pagefile_usage = Some(value);
    }

    /// Gets the value of PeakPagefileUsage
    pub fn get_peak_pagefile_usage(&self) -> Option<&serde_json::Value> {
        self.peak_pagefile_usage.as_ref()
    }

    /// Sets the value of PeakVirtualSize
    pub fn set_peak_virtual_size(&mut self, value: serde_json::Value) {
        self.peak_virtual_size = Some(value);
    }

    /// Gets the value of PeakVirtualSize
    pub fn get_peak_virtual_size(&self) -> Option<&serde_json::Value> {
        self.peak_virtual_size.as_ref()
    }

    /// Sets the value of PeakWorkingSetSize
    pub fn set_peak_working_set_size(&mut self, value: serde_json::Value) {
        self.peak_working_set_size = Some(value);
    }

    /// Gets the value of PeakWorkingSetSize
    pub fn get_peak_working_set_size(&self) -> Option<&serde_json::Value> {
        self.peak_working_set_size.as_ref()
    }

    /// Sets the value of PrivatePageCount
    pub fn set_private_page_count(&mut self, value: serde_json::Value) {
        self.private_page_count = Some(value);
    }

    /// Gets the value of PrivatePageCount
    pub fn get_private_page_count(&self) -> Option<&serde_json::Value> {
        self.private_page_count.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of QuotaNonPagedPoolUsage
    pub fn set_quota_non_paged_pool_usage(&mut self, value: serde_json::Value) {
        self.quota_non_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaNonPagedPoolUsage
    pub fn get_quota_non_paged_pool_usage(&self) -> Option<&serde_json::Value> {
        self.quota_non_paged_pool_usage.as_ref()
    }

    /// Sets the value of QuotaPagedPoolUsage
    pub fn set_quota_paged_pool_usage(&mut self, value: serde_json::Value) {
        self.quota_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaPagedPoolUsage
    pub fn get_quota_paged_pool_usage(&self) -> Option<&serde_json::Value> {
        self.quota_paged_pool_usage.as_ref()
    }

    /// Sets the value of QuotaPeakNonPagedPoolUsage
    pub fn set_quota_peak_non_paged_pool_usage(&mut self, value: serde_json::Value) {
        self.quota_peak_non_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaPeakNonPagedPoolUsage
    pub fn get_quota_peak_non_paged_pool_usage(&self) -> Option<&serde_json::Value> {
        self.quota_peak_non_paged_pool_usage.as_ref()
    }

    /// Sets the value of QuotaPeakPagedPoolUsage
    pub fn set_quota_peak_paged_pool_usage(&mut self, value: serde_json::Value) {
        self.quota_peak_paged_pool_usage = Some(value);
    }

    /// Gets the value of QuotaPeakPagedPoolUsage
    pub fn get_quota_peak_paged_pool_usage(&self) -> Option<&serde_json::Value> {
        self.quota_peak_paged_pool_usage.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of VirtualSize
    pub fn set_virtual_size(&mut self, value: serde_json::Value) {
        self.virtual_size = Some(value);
    }

    /// Gets the value of VirtualSize
    pub fn get_virtual_size(&self) -> Option<&serde_json::Value> {
        self.virtual_size.as_ref()
    }

    /// Sets the value of WorkingSetSize
    pub fn set_working_set_size(&mut self, value: serde_json::Value) {
        self.working_set_size = Some(value);
    }

    /// Gets the value of WorkingSetSize
    pub fn get_working_set_size(&self) -> Option<&serde_json::Value> {
        self.working_set_size.as_ref()
    }
}

