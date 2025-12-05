// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTMemorySummary struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTMemorySummary {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Available")]
    pub available: Option<u64>,

/// 
    #[serde(rename = "Cached")]
    pub cached: Option<u64>,

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// 
    #[serde(rename = "CommitLimit")]
    pub commit_limit: Option<u64>,

/// 
    #[serde(rename = "Committed")]
    pub committed: Option<u64>,

/// 
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<u16>,

/// 
    #[serde(rename = "DynamicMemoryEnabled")]
    pub dynamic_memory_enabled: Option<bool>,

/// 
    #[serde(rename = "DynamicMemoryMax")]
    pub dynamic_memory_max: Option<u64>,

/// 
    #[serde(rename = "FormFactor")]
    pub form_factor: Option<u16>,

/// 
    #[serde(rename = "Free")]
    pub free: Option<u64>,

/// 
    #[serde(rename = "HardwareReserved")]
    pub hardware_reserved: Option<u64>,

/// 
    #[serde(rename = "Installed")]
    pub installed: Option<u64>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "InUse")]
    pub in_use: Option<u64>,

/// 
    #[serde(rename = "Modified")]
    pub modified: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NonPagedPool")]
    pub non_paged_pool: Option<u64>,

/// 
    #[serde(rename = "PagedPool")]
    pub paged_pool: Option<u64>,

/// 
    #[serde(rename = "PageSize")]
    pub page_size: Option<u32>,

/// 
    #[serde(rename = "Speed")]
    pub speed: Option<u32>,

/// 
    #[serde(rename = "Standby")]
    pub standby: Option<u64>,

/// 
    #[serde(rename = "Total")]
    pub total: Option<u64>,

/// 
    #[serde(rename = "TotalSlots")]
    pub total_slots: Option<u16>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,

/// 
    #[serde(rename = "UsedSlots")]
    pub used_slots: Option<u16>,

/// 
    #[serde(rename = "Utilization")]
    pub utilization: Vec<f32>,
}

impl MSFT_MTMemorySummary {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            available: None,
            cached: None,
            capacity: None,
            commit_limit: None,
            committed: None,
            current_index: None,
            dynamic_memory_enabled: None,
            dynamic_memory_max: None,
            form_factor: None,
            free: None,
            hardware_reserved: None,
            installed: None,
            interval_seconds: None,
            in_use: None,
            modified: None,
            name: None,
            non_paged_pool: None,
            paged_pool: None,
            page_size: None,
            speed: None,
            standby: None,
            total: None,
            total_slots: None,
            type: None,
            used_slots: None,
            utilization: Vec::new(),
        }
    }


    /// Sets the value of Available
    pub fn set_available(&mut self, value: u64) {
        self.available = Some(value);
    }

    /// Gets the value of Available
    pub fn get_available(&self) -> Option<&u64> {
        self.available.as_ref()
    }

    /// Sets the value of Cached
    pub fn set_cached(&mut self, value: u64) {
        self.cached = Some(value);
    }

    /// Gets the value of Cached
    pub fn get_cached(&self) -> Option<&u64> {
        self.cached.as_ref()
    }

    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of CommitLimit
    pub fn set_commit_limit(&mut self, value: u64) {
        self.commit_limit = Some(value);
    }

    /// Gets the value of CommitLimit
    pub fn get_commit_limit(&self) -> Option<&u64> {
        self.commit_limit.as_ref()
    }

    /// Sets the value of Committed
    pub fn set_committed(&mut self, value: u64) {
        self.committed = Some(value);
    }

    /// Gets the value of Committed
    pub fn get_committed(&self) -> Option<&u64> {
        self.committed.as_ref()
    }

    /// Sets the value of CurrentIndex
    pub fn set_current_index(&mut self, value: u16) {
        self.current_index = Some(value);
    }

    /// Gets the value of CurrentIndex
    pub fn get_current_index(&self) -> Option<&u16> {
        self.current_index.as_ref()
    }

    /// Sets the value of DynamicMemoryEnabled
    pub fn set_dynamic_memory_enabled(&mut self, value: bool) {
        self.dynamic_memory_enabled = Some(value);
    }

    /// Gets the value of DynamicMemoryEnabled
    pub fn get_dynamic_memory_enabled(&self) -> Option<&bool> {
        self.dynamic_memory_enabled.as_ref()
    }

    /// Sets the value of DynamicMemoryMax
    pub fn set_dynamic_memory_max(&mut self, value: u64) {
        self.dynamic_memory_max = Some(value);
    }

    /// Gets the value of DynamicMemoryMax
    pub fn get_dynamic_memory_max(&self) -> Option<&u64> {
        self.dynamic_memory_max.as_ref()
    }

    /// Sets the value of FormFactor
    pub fn set_form_factor(&mut self, value: u16) {
        self.form_factor = Some(value);
    }

    /// Gets the value of FormFactor
    pub fn get_form_factor(&self) -> Option<&u16> {
        self.form_factor.as_ref()
    }

    /// Sets the value of Free
    pub fn set_free(&mut self, value: u64) {
        self.free = Some(value);
    }

    /// Gets the value of Free
    pub fn get_free(&self) -> Option<&u64> {
        self.free.as_ref()
    }

    /// Sets the value of HardwareReserved
    pub fn set_hardware_reserved(&mut self, value: u64) {
        self.hardware_reserved = Some(value);
    }

    /// Gets the value of HardwareReserved
    pub fn get_hardware_reserved(&self) -> Option<&u64> {
        self.hardware_reserved.as_ref()
    }

    /// Sets the value of Installed
    pub fn set_installed(&mut self, value: u64) {
        self.installed = Some(value);
    }

    /// Gets the value of Installed
    pub fn get_installed(&self) -> Option<&u64> {
        self.installed.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of InUse
    pub fn set_in_use(&mut self, value: u64) {
        self.in_use = Some(value);
    }

    /// Gets the value of InUse
    pub fn get_in_use(&self) -> Option<&u64> {
        self.in_use.as_ref()
    }

    /// Sets the value of Modified
    pub fn set_modified(&mut self, value: u64) {
        self.modified = Some(value);
    }

    /// Gets the value of Modified
    pub fn get_modified(&self) -> Option<&u64> {
        self.modified.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NonPagedPool
    pub fn set_non_paged_pool(&mut self, value: u64) {
        self.non_paged_pool = Some(value);
    }

    /// Gets the value of NonPagedPool
    pub fn get_non_paged_pool(&self) -> Option<&u64> {
        self.non_paged_pool.as_ref()
    }

    /// Sets the value of PagedPool
    pub fn set_paged_pool(&mut self, value: u64) {
        self.paged_pool = Some(value);
    }

    /// Gets the value of PagedPool
    pub fn get_paged_pool(&self) -> Option<&u64> {
        self.paged_pool.as_ref()
    }

    /// Sets the value of PageSize
    pub fn set_page_size(&mut self, value: u32) {
        self.page_size = Some(value);
    }

    /// Gets the value of PageSize
    pub fn get_page_size(&self) -> Option<&u32> {
        self.page_size.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u32) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u32> {
        self.speed.as_ref()
    }

    /// Sets the value of Standby
    pub fn set_standby(&mut self, value: u64) {
        self.standby = Some(value);
    }

    /// Gets the value of Standby
    pub fn get_standby(&self) -> Option<&u64> {
        self.standby.as_ref()
    }

    /// Sets the value of Total
    pub fn set_total(&mut self, value: u64) {
        self.total = Some(value);
    }

    /// Gets the value of Total
    pub fn get_total(&self) -> Option<&u64> {
        self.total.as_ref()
    }

    /// Sets the value of TotalSlots
    pub fn set_total_slots(&mut self, value: u16) {
        self.total_slots = Some(value);
    }

    /// Gets the value of TotalSlots
    pub fn get_total_slots(&self) -> Option<&u16> {
        self.total_slots.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

    /// Sets the value of UsedSlots
    pub fn set_used_slots(&mut self, value: u16) {
        self.used_slots = Some(value);
    }

    /// Gets the value of UsedSlots
    pub fn get_used_slots(&self) -> Option<&u16> {
        self.used_slots.as_ref()
    }

    /// Sets the value of Utilization
    pub fn set_utilization(&mut self, value: Vec<f32>) {
        self.utilization = value;
    }

    /// Gets the value of Utilization
    pub fn get_utilization(&self) -> &Vec<f32> {
        &self.utilization
    }
}

