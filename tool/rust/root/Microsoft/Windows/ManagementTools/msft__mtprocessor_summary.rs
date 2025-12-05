// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTProcessorSummary struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTProcessorSummary {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AverageSpeed")]
    pub average_speed: Option<f32>,

/// 
    #[serde(rename = "Cores")]
    pub cores: Option<u32>,

/// 
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<u16>,

/// 
    #[serde(rename = "Handles")]
    pub handles: Option<u32>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "L1Cache")]
    pub l1_cache: Option<u32>,

/// 
    #[serde(rename = "L2Cache")]
    pub l2_cache: Option<u32>,

/// 
    #[serde(rename = "L3Cache")]
    pub l3_cache: Option<u32>,

/// 
    #[serde(rename = "LogicalProcessors")]
    pub logical_processors: Option<u32>,

/// 
    #[serde(rename = "MaximumSpeed")]
    pub maximum_speed: Option<f32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NumaNodes")]
    pub numa_nodes: Option<u16>,

/// 
    #[serde(rename = "Privileged")]
    pub privileged: Vec<f32>,

/// 
    #[serde(rename = "Processes")]
    pub processes: Option<u32>,

/// 
    #[serde(rename = "Sockets")]
    pub sockets: Option<u32>,

/// 
    #[serde(rename = "Threads")]
    pub threads: Option<u32>,

/// 
    #[serde(rename = "Uptime")]
    pub uptime: Option<u64>,

/// 
    #[serde(rename = "Utilization")]
    pub utilization: Vec<f32>,

/// 
    #[serde(rename = "Virtualization")]
    pub virtualization: Option<u16>,
}

impl MSFT_MTProcessorSummary {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            average_speed: None,
            cores: None,
            current_index: None,
            handles: None,
            interval_seconds: None,
            l1_cache: None,
            l2_cache: None,
            l3_cache: None,
            logical_processors: None,
            maximum_speed: None,
            name: None,
            numa_nodes: None,
            privileged: Vec::new(),
            processes: None,
            sockets: None,
            threads: None,
            uptime: None,
            utilization: Vec::new(),
            virtualization: None,
        }
    }


    /// Sets the value of AverageSpeed
    pub fn set_average_speed(&mut self, value: f32) {
        self.average_speed = Some(value);
    }

    /// Gets the value of AverageSpeed
    pub fn get_average_speed(&self) -> Option<&f32> {
        self.average_speed.as_ref()
    }

    /// Sets the value of Cores
    pub fn set_cores(&mut self, value: u32) {
        self.cores = Some(value);
    }

    /// Gets the value of Cores
    pub fn get_cores(&self) -> Option<&u32> {
        self.cores.as_ref()
    }

    /// Sets the value of CurrentIndex
    pub fn set_current_index(&mut self, value: u16) {
        self.current_index = Some(value);
    }

    /// Gets the value of CurrentIndex
    pub fn get_current_index(&self) -> Option<&u16> {
        self.current_index.as_ref()
    }

    /// Sets the value of Handles
    pub fn set_handles(&mut self, value: u32) {
        self.handles = Some(value);
    }

    /// Gets the value of Handles
    pub fn get_handles(&self) -> Option<&u32> {
        self.handles.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of L1Cache
    pub fn set_l1_cache(&mut self, value: u32) {
        self.l1_cache = Some(value);
    }

    /// Gets the value of L1Cache
    pub fn get_l1_cache(&self) -> Option<&u32> {
        self.l1_cache.as_ref()
    }

    /// Sets the value of L2Cache
    pub fn set_l2_cache(&mut self, value: u32) {
        self.l2_cache = Some(value);
    }

    /// Gets the value of L2Cache
    pub fn get_l2_cache(&self) -> Option<&u32> {
        self.l2_cache.as_ref()
    }

    /// Sets the value of L3Cache
    pub fn set_l3_cache(&mut self, value: u32) {
        self.l3_cache = Some(value);
    }

    /// Gets the value of L3Cache
    pub fn get_l3_cache(&self) -> Option<&u32> {
        self.l3_cache.as_ref()
    }

    /// Sets the value of LogicalProcessors
    pub fn set_logical_processors(&mut self, value: u32) {
        self.logical_processors = Some(value);
    }

    /// Gets the value of LogicalProcessors
    pub fn get_logical_processors(&self) -> Option<&u32> {
        self.logical_processors.as_ref()
    }

    /// Sets the value of MaximumSpeed
    pub fn set_maximum_speed(&mut self, value: f32) {
        self.maximum_speed = Some(value);
    }

    /// Gets the value of MaximumSpeed
    pub fn get_maximum_speed(&self) -> Option<&f32> {
        self.maximum_speed.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NumaNodes
    pub fn set_numa_nodes(&mut self, value: u16) {
        self.numa_nodes = Some(value);
    }

    /// Gets the value of NumaNodes
    pub fn get_numa_nodes(&self) -> Option<&u16> {
        self.numa_nodes.as_ref()
    }

    /// Sets the value of Privileged
    pub fn set_privileged(&mut self, value: Vec<f32>) {
        self.privileged = value;
    }

    /// Gets the value of Privileged
    pub fn get_privileged(&self) -> &Vec<f32> {
        &self.privileged
    }

    /// Sets the value of Processes
    pub fn set_processes(&mut self, value: u32) {
        self.processes = Some(value);
    }

    /// Gets the value of Processes
    pub fn get_processes(&self) -> Option<&u32> {
        self.processes.as_ref()
    }

    /// Sets the value of Sockets
    pub fn set_sockets(&mut self, value: u32) {
        self.sockets = Some(value);
    }

    /// Gets the value of Sockets
    pub fn get_sockets(&self) -> Option<&u32> {
        self.sockets.as_ref()
    }

    /// Sets the value of Threads
    pub fn set_threads(&mut self, value: u32) {
        self.threads = Some(value);
    }

    /// Gets the value of Threads
    pub fn get_threads(&self) -> Option<&u32> {
        self.threads.as_ref()
    }

    /// Sets the value of Uptime
    pub fn set_uptime(&mut self, value: u64) {
        self.uptime = Some(value);
    }

    /// Gets the value of Uptime
    pub fn get_uptime(&self) -> Option<&u64> {
        self.uptime.as_ref()
    }

    /// Sets the value of Utilization
    pub fn set_utilization(&mut self, value: Vec<f32>) {
        self.utilization = value;
    }

    /// Gets the value of Utilization
    pub fn get_utilization(&self) -> &Vec<f32> {
        &self.utilization
    }

    /// Sets the value of Virtualization
    pub fn set_virtualization(&mut self, value: u16) {
        self.virtualization = Some(value);
    }

    /// Gets the value of Virtualization
    pub fn get_virtualization(&self) -> Option<&u16> {
        self.virtualization.as_ref()
    }
}

