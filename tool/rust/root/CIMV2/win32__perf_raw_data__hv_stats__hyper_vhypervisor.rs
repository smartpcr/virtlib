// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_HvStats_HyperVHypervisor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_HvStats_HyperVHypervisor {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "HypervisorStartupCost")]
    pub hypervisor_startup_cost: Option<u64>,

/// 
    #[serde(rename = "LogicalProcessors")]
    pub logical_processors: Option<u64>,

/// 
    #[serde(rename = "ModernStandbyEntries")]
    pub modern_standby_entries: Option<u64>,

/// 
    #[serde(rename = "MonitoredNotifications")]
    pub monitored_notifications: Option<u64>,

/// 
    #[serde(rename = "Partitions")]
    pub partitions: Option<u64>,

/// 
    #[serde(rename = "PlatformIdleTransitions")]
    pub platform_idle_transitions: Option<u64>,

/// 
    #[serde(rename = "TotalPages")]
    pub total_pages: Option<u64>,

/// 
    #[serde(rename = "VirtualProcessors")]
    pub virtual_processors: Option<u64>,
}

impl Win32_PerfRawData_HvStats_HyperVHypervisor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            hypervisor_startup_cost: None,
            logical_processors: None,
            modern_standby_entries: None,
            monitored_notifications: None,
            partitions: None,
            platform_idle_transitions: None,
            total_pages: None,
            virtual_processors: None,
        }
    }


    /// Sets the value of HypervisorStartupCost
    pub fn set_hypervisor_startup_cost(&mut self, value: u64) {
        self.hypervisor_startup_cost = Some(value);
    }

    /// Gets the value of HypervisorStartupCost
    pub fn get_hypervisor_startup_cost(&self) -> Option<&u64> {
        self.hypervisor_startup_cost.as_ref()
    }

    /// Sets the value of LogicalProcessors
    pub fn set_logical_processors(&mut self, value: u64) {
        self.logical_processors = Some(value);
    }

    /// Gets the value of LogicalProcessors
    pub fn get_logical_processors(&self) -> Option<&u64> {
        self.logical_processors.as_ref()
    }

    /// Sets the value of ModernStandbyEntries
    pub fn set_modern_standby_entries(&mut self, value: u64) {
        self.modern_standby_entries = Some(value);
    }

    /// Gets the value of ModernStandbyEntries
    pub fn get_modern_standby_entries(&self) -> Option<&u64> {
        self.modern_standby_entries.as_ref()
    }

    /// Sets the value of MonitoredNotifications
    pub fn set_monitored_notifications(&mut self, value: u64) {
        self.monitored_notifications = Some(value);
    }

    /// Gets the value of MonitoredNotifications
    pub fn get_monitored_notifications(&self) -> Option<&u64> {
        self.monitored_notifications.as_ref()
    }

    /// Sets the value of Partitions
    pub fn set_partitions(&mut self, value: u64) {
        self.partitions = Some(value);
    }

    /// Gets the value of Partitions
    pub fn get_partitions(&self) -> Option<&u64> {
        self.partitions.as_ref()
    }

    /// Sets the value of PlatformIdleTransitions
    pub fn set_platform_idle_transitions(&mut self, value: u64) {
        self.platform_idle_transitions = Some(value);
    }

    /// Gets the value of PlatformIdleTransitions
    pub fn get_platform_idle_transitions(&self) -> Option<&u64> {
        self.platform_idle_transitions.as_ref()
    }

    /// Sets the value of TotalPages
    pub fn set_total_pages(&mut self, value: u64) {
        self.total_pages = Some(value);
    }

    /// Gets the value of TotalPages
    pub fn get_total_pages(&self) -> Option<&u64> {
        self.total_pages.as_ref()
    }

    /// Sets the value of VirtualProcessors
    pub fn set_virtual_processors(&mut self, value: u64) {
        self.virtual_processors = Some(value);
    }

    /// Gets the value of VirtualProcessors
    pub fn get_virtual_processors(&self) -> Option<&u64> {
        self.virtual_processors.as_ref()
    }
}

