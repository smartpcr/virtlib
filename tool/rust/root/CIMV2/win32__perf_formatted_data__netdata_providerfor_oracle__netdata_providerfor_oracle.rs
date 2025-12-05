// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NETDataProviderforOracle_NETDataProviderforOracle struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NETDataProviderforOracle_NETDataProviderforOracle {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "HardConnectsPerSecond")]
    pub hard_connects_per_second: Option<u32>,

/// 
    #[serde(rename = "HardDisconnectsPerSecond")]
    pub hard_disconnects_per_second: Option<u32>,

/// 
    #[serde(rename = "NumberOfActiveConnectionPoolGroups")]
    pub number_of_active_connection_pool_groups: Option<u32>,

/// 
    #[serde(rename = "NumberOfActiveConnectionPools")]
    pub number_of_active_connection_pools: Option<u32>,

/// 
    #[serde(rename = "NumberOfActiveConnections")]
    pub number_of_active_connections: Option<u32>,

/// 
    #[serde(rename = "NumberOfFreeConnections")]
    pub number_of_free_connections: Option<u32>,

/// 
    #[serde(rename = "NumberOfInactiveConnectionPoolGroups")]
    pub number_of_inactive_connection_pool_groups: Option<u32>,

/// 
    #[serde(rename = "NumberOfInactiveConnectionPools")]
    pub number_of_inactive_connection_pools: Option<u32>,

/// 
    #[serde(rename = "NumberOfNonPooledConnections")]
    pub number_of_non_pooled_connections: Option<u32>,

/// 
    #[serde(rename = "NumberOfPooledConnections")]
    pub number_of_pooled_connections: Option<u32>,

/// 
    #[serde(rename = "NumberOfReclaimedConnections")]
    pub number_of_reclaimed_connections: Option<u32>,

/// 
    #[serde(rename = "NumberOfStasisConnections")]
    pub number_of_stasis_connections: Option<u32>,

/// 
    #[serde(rename = "SoftConnectsPerSecond")]
    pub soft_connects_per_second: Option<u32>,

/// 
    #[serde(rename = "SoftDisconnectsPerSecond")]
    pub soft_disconnects_per_second: Option<u32>,
}

impl Win32_PerfFormattedData_NETDataProviderforOracle_NETDataProviderforOracle {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            hard_connects_per_second: None,
            hard_disconnects_per_second: None,
            number_of_active_connection_pool_groups: None,
            number_of_active_connection_pools: None,
            number_of_active_connections: None,
            number_of_free_connections: None,
            number_of_inactive_connection_pool_groups: None,
            number_of_inactive_connection_pools: None,
            number_of_non_pooled_connections: None,
            number_of_pooled_connections: None,
            number_of_reclaimed_connections: None,
            number_of_stasis_connections: None,
            soft_connects_per_second: None,
            soft_disconnects_per_second: None,
        }
    }


    /// Sets the value of HardConnectsPerSecond
    pub fn set_hard_connects_per_second(&mut self, value: u32) {
        self.hard_connects_per_second = Some(value);
    }

    /// Gets the value of HardConnectsPerSecond
    pub fn get_hard_connects_per_second(&self) -> Option<&u32> {
        self.hard_connects_per_second.as_ref()
    }

    /// Sets the value of HardDisconnectsPerSecond
    pub fn set_hard_disconnects_per_second(&mut self, value: u32) {
        self.hard_disconnects_per_second = Some(value);
    }

    /// Gets the value of HardDisconnectsPerSecond
    pub fn get_hard_disconnects_per_second(&self) -> Option<&u32> {
        self.hard_disconnects_per_second.as_ref()
    }

    /// Sets the value of NumberOfActiveConnectionPoolGroups
    pub fn set_number_of_active_connection_pool_groups(&mut self, value: u32) {
        self.number_of_active_connection_pool_groups = Some(value);
    }

    /// Gets the value of NumberOfActiveConnectionPoolGroups
    pub fn get_number_of_active_connection_pool_groups(&self) -> Option<&u32> {
        self.number_of_active_connection_pool_groups.as_ref()
    }

    /// Sets the value of NumberOfActiveConnectionPools
    pub fn set_number_of_active_connection_pools(&mut self, value: u32) {
        self.number_of_active_connection_pools = Some(value);
    }

    /// Gets the value of NumberOfActiveConnectionPools
    pub fn get_number_of_active_connection_pools(&self) -> Option<&u32> {
        self.number_of_active_connection_pools.as_ref()
    }

    /// Sets the value of NumberOfActiveConnections
    pub fn set_number_of_active_connections(&mut self, value: u32) {
        self.number_of_active_connections = Some(value);
    }

    /// Gets the value of NumberOfActiveConnections
    pub fn get_number_of_active_connections(&self) -> Option<&u32> {
        self.number_of_active_connections.as_ref()
    }

    /// Sets the value of NumberOfFreeConnections
    pub fn set_number_of_free_connections(&mut self, value: u32) {
        self.number_of_free_connections = Some(value);
    }

    /// Gets the value of NumberOfFreeConnections
    pub fn get_number_of_free_connections(&self) -> Option<&u32> {
        self.number_of_free_connections.as_ref()
    }

    /// Sets the value of NumberOfInactiveConnectionPoolGroups
    pub fn set_number_of_inactive_connection_pool_groups(&mut self, value: u32) {
        self.number_of_inactive_connection_pool_groups = Some(value);
    }

    /// Gets the value of NumberOfInactiveConnectionPoolGroups
    pub fn get_number_of_inactive_connection_pool_groups(&self) -> Option<&u32> {
        self.number_of_inactive_connection_pool_groups.as_ref()
    }

    /// Sets the value of NumberOfInactiveConnectionPools
    pub fn set_number_of_inactive_connection_pools(&mut self, value: u32) {
        self.number_of_inactive_connection_pools = Some(value);
    }

    /// Gets the value of NumberOfInactiveConnectionPools
    pub fn get_number_of_inactive_connection_pools(&self) -> Option<&u32> {
        self.number_of_inactive_connection_pools.as_ref()
    }

    /// Sets the value of NumberOfNonPooledConnections
    pub fn set_number_of_non_pooled_connections(&mut self, value: u32) {
        self.number_of_non_pooled_connections = Some(value);
    }

    /// Gets the value of NumberOfNonPooledConnections
    pub fn get_number_of_non_pooled_connections(&self) -> Option<&u32> {
        self.number_of_non_pooled_connections.as_ref()
    }

    /// Sets the value of NumberOfPooledConnections
    pub fn set_number_of_pooled_connections(&mut self, value: u32) {
        self.number_of_pooled_connections = Some(value);
    }

    /// Gets the value of NumberOfPooledConnections
    pub fn get_number_of_pooled_connections(&self) -> Option<&u32> {
        self.number_of_pooled_connections.as_ref()
    }

    /// Sets the value of NumberOfReclaimedConnections
    pub fn set_number_of_reclaimed_connections(&mut self, value: u32) {
        self.number_of_reclaimed_connections = Some(value);
    }

    /// Gets the value of NumberOfReclaimedConnections
    pub fn get_number_of_reclaimed_connections(&self) -> Option<&u32> {
        self.number_of_reclaimed_connections.as_ref()
    }

    /// Sets the value of NumberOfStasisConnections
    pub fn set_number_of_stasis_connections(&mut self, value: u32) {
        self.number_of_stasis_connections = Some(value);
    }

    /// Gets the value of NumberOfStasisConnections
    pub fn get_number_of_stasis_connections(&self) -> Option<&u32> {
        self.number_of_stasis_connections.as_ref()
    }

    /// Sets the value of SoftConnectsPerSecond
    pub fn set_soft_connects_per_second(&mut self, value: u32) {
        self.soft_connects_per_second = Some(value);
    }

    /// Gets the value of SoftConnectsPerSecond
    pub fn get_soft_connects_per_second(&self) -> Option<&u32> {
        self.soft_connects_per_second.as_ref()
    }

    /// Sets the value of SoftDisconnectsPerSecond
    pub fn set_soft_disconnects_per_second(&mut self, value: u32) {
        self.soft_disconnects_per_second = Some(value);
    }

    /// Gets the value of SoftDisconnectsPerSecond
    pub fn get_soft_disconnects_per_second(&self) -> Option<&u32> {
        self.soft_disconnects_per_second.as_ref()
    }
}

