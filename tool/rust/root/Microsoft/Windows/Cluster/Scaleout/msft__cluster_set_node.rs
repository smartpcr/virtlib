// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetNode {

/// 
    #[serde(rename = "AvailableMemory")]
    pub available_memory: Option<u64>,

/// 
    #[serde(rename = "AvailableMemoryAfterReclaimation")]
    pub available_memory_after_reclaimation: Option<u64>,

/// 
    #[serde(rename = "AverageCpuUsage")]
    pub average_cpu_usage: Option<u32>,

/// 
    #[serde(rename = "FreeCpuReserve")]
    pub free_cpu_reserve: Option<u64>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,

/// 
    #[serde(rename = "LocalDiskFreeSpaceInMB")]
    pub local_disk_free_space_in_mb: Option<u32>,

/// 
    #[serde(rename = "LocalDiskTotalSpaceInMB")]
    pub local_disk_total_space_in_mb: Option<u32>,

/// 
    #[serde(rename = "MaxCpuReserve")]
    pub max_cpu_reserve: Option<u64>,

/// 
    #[serde(rename = "Member")]
    pub member: Option<String>,

/// 
    #[serde(rename = "MemberId")]
    pub member_id: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NodeId")]
    pub node_id: Option<u32>,

/// 
    #[serde(rename = "NodeLPCount")]
    pub node_lpcount: Option<u32>,

/// 
    #[serde(rename = "ReserveCpu")]
    pub reserve_cpu: Option<u64>,

/// 
    #[serde(rename = "ReservedLocalDiskUsage")]
    pub reserved_local_disk_usage: Option<u32>,

/// 
    #[serde(rename = "ReservedMemory")]
    pub reserved_memory: Option<u64>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "TotalMemory")]
    pub total_memory: Option<u64>,
}

impl MSFT_ClusterSetNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            available_memory: None,
            available_memory_after_reclaimation: None,
            average_cpu_usage: None,
            free_cpu_reserve: None,
            id: None,
            local_disk_free_space_in_mb: None,
            local_disk_total_space_in_mb: None,
            max_cpu_reserve: None,
            member: None,
            member_id: None,
            name: None,
            node_id: None,
            node_lpcount: None,
            reserve_cpu: None,
            reserved_local_disk_usage: None,
            reserved_memory: None,
            state: None,
            total_memory: None,
        }
    }


    /// Sets the value of AvailableMemory
    pub fn set_available_memory(&mut self, value: u64) {
        self.available_memory = Some(value);
    }

    /// Gets the value of AvailableMemory
    pub fn get_available_memory(&self) -> Option<&u64> {
        self.available_memory.as_ref()
    }

    /// Sets the value of AvailableMemoryAfterReclaimation
    pub fn set_available_memory_after_reclaimation(&mut self, value: u64) {
        self.available_memory_after_reclaimation = Some(value);
    }

    /// Gets the value of AvailableMemoryAfterReclaimation
    pub fn get_available_memory_after_reclaimation(&self) -> Option<&u64> {
        self.available_memory_after_reclaimation.as_ref()
    }

    /// Sets the value of AverageCpuUsage
    pub fn set_average_cpu_usage(&mut self, value: u32) {
        self.average_cpu_usage = Some(value);
    }

    /// Gets the value of AverageCpuUsage
    pub fn get_average_cpu_usage(&self) -> Option<&u32> {
        self.average_cpu_usage.as_ref()
    }

    /// Sets the value of FreeCpuReserve
    pub fn set_free_cpu_reserve(&mut self, value: u64) {
        self.free_cpu_reserve = Some(value);
    }

    /// Gets the value of FreeCpuReserve
    pub fn get_free_cpu_reserve(&self) -> Option<&u64> {
        self.free_cpu_reserve.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    /// Sets the value of LocalDiskFreeSpaceInMB
    pub fn set_local_disk_free_space_in_mb(&mut self, value: u32) {
        self.local_disk_free_space_in_mb = Some(value);
    }

    /// Gets the value of LocalDiskFreeSpaceInMB
    pub fn get_local_disk_free_space_in_mb(&self) -> Option<&u32> {
        self.local_disk_free_space_in_mb.as_ref()
    }

    /// Sets the value of LocalDiskTotalSpaceInMB
    pub fn set_local_disk_total_space_in_mb(&mut self, value: u32) {
        self.local_disk_total_space_in_mb = Some(value);
    }

    /// Gets the value of LocalDiskTotalSpaceInMB
    pub fn get_local_disk_total_space_in_mb(&self) -> Option<&u32> {
        self.local_disk_total_space_in_mb.as_ref()
    }

    /// Sets the value of MaxCpuReserve
    pub fn set_max_cpu_reserve(&mut self, value: u64) {
        self.max_cpu_reserve = Some(value);
    }

    /// Gets the value of MaxCpuReserve
    pub fn get_max_cpu_reserve(&self) -> Option<&u64> {
        self.max_cpu_reserve.as_ref()
    }

    /// Sets the value of Member
    pub fn set_member(&mut self, value: String) {
        self.member = Some(value);
    }

    /// Gets the value of Member
    pub fn get_member(&self) -> Option<&String> {
        self.member.as_ref()
    }

    /// Sets the value of MemberId
    pub fn set_member_id(&mut self, value: u64) {
        self.member_id = Some(value);
    }

    /// Gets the value of MemberId
    pub fn get_member_id(&self) -> Option<&u64> {
        self.member_id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NodeId
    pub fn set_node_id(&mut self, value: u32) {
        self.node_id = Some(value);
    }

    /// Gets the value of NodeId
    pub fn get_node_id(&self) -> Option<&u32> {
        self.node_id.as_ref()
    }

    /// Sets the value of NodeLPCount
    pub fn set_node_lpcount(&mut self, value: u32) {
        self.node_lpcount = Some(value);
    }

    /// Gets the value of NodeLPCount
    pub fn get_node_lpcount(&self) -> Option<&u32> {
        self.node_lpcount.as_ref()
    }

    /// Sets the value of ReserveCpu
    pub fn set_reserve_cpu(&mut self, value: u64) {
        self.reserve_cpu = Some(value);
    }

    /// Gets the value of ReserveCpu
    pub fn get_reserve_cpu(&self) -> Option<&u64> {
        self.reserve_cpu.as_ref()
    }

    /// Sets the value of ReservedLocalDiskUsage
    pub fn set_reserved_local_disk_usage(&mut self, value: u32) {
        self.reserved_local_disk_usage = Some(value);
    }

    /// Gets the value of ReservedLocalDiskUsage
    pub fn get_reserved_local_disk_usage(&self) -> Option<&u32> {
        self.reserved_local_disk_usage.as_ref()
    }

    /// Sets the value of ReservedMemory
    pub fn set_reserved_memory(&mut self, value: u64) {
        self.reserved_memory = Some(value);
    }

    /// Gets the value of ReservedMemory
    pub fn get_reserved_memory(&self) -> Option<&u64> {
        self.reserved_memory.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of TotalMemory
    pub fn set_total_memory(&mut self, value: u64) {
        self.total_memory = Some(value);
    }

    /// Gets the value of TotalMemory
    pub fn get_total_memory(&self) -> Option<&u64> {
        self.total_memory.as_ref()
    }
}

