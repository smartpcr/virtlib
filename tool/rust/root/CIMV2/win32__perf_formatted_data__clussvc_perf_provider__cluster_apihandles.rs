// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ClussvcPerfProvider_ClusterAPIHandles struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ClussvcPerfProvider_ClusterAPIHandles {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BatchHandles")]
    pub batch_handles: Option<u64>,

/// 
    #[serde(rename = "BatchHandlesPersec")]
    pub batch_handles_persec: Option<u64>,

/// 
    #[serde(rename = "ClusterHandles")]
    pub cluster_handles: Option<u64>,

/// 
    #[serde(rename = "ClusterHandlesPersec")]
    pub cluster_handles_persec: Option<u64>,

/// 
    #[serde(rename = "GroupHandles")]
    pub group_handles: Option<u64>,

/// 
    #[serde(rename = "GroupHandlesPersec")]
    pub group_handles_persec: Option<u64>,

/// 
    #[serde(rename = "KeyHandles")]
    pub key_handles: Option<u64>,

/// 
    #[serde(rename = "KeyHandlesPersec")]
    pub key_handles_persec: Option<u64>,

/// 
    #[serde(rename = "NetworkHandles")]
    pub network_handles: Option<u64>,

/// 
    #[serde(rename = "NetworkHandlesPersec")]
    pub network_handles_persec: Option<u64>,

/// 
    #[serde(rename = "NetworkInterfaceHandles")]
    pub network_interface_handles: Option<u64>,

/// 
    #[serde(rename = "NetworkInterfaceHandlesPersec")]
    pub network_interface_handles_persec: Option<u64>,

/// 
    #[serde(rename = "NodeHandles")]
    pub node_handles: Option<u64>,

/// 
    #[serde(rename = "NodeHandlesPersec")]
    pub node_handles_persec: Option<u64>,

/// 
    #[serde(rename = "NotificationHandles")]
    pub notification_handles: Option<u64>,

/// 
    #[serde(rename = "NotificationHandlesPersec")]
    pub notification_handles_persec: Option<u64>,

/// 
    #[serde(rename = "ResourceHandles")]
    pub resource_handles: Option<u64>,

/// 
    #[serde(rename = "ResourceHandlesPersec")]
    pub resource_handles_persec: Option<u64>,
}

impl Win32_PerfFormattedData_ClussvcPerfProvider_ClusterAPIHandles {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            batch_handles: None,
            batch_handles_persec: None,
            cluster_handles: None,
            cluster_handles_persec: None,
            group_handles: None,
            group_handles_persec: None,
            key_handles: None,
            key_handles_persec: None,
            network_handles: None,
            network_handles_persec: None,
            network_interface_handles: None,
            network_interface_handles_persec: None,
            node_handles: None,
            node_handles_persec: None,
            notification_handles: None,
            notification_handles_persec: None,
            resource_handles: None,
            resource_handles_persec: None,
        }
    }


    /// Sets the value of BatchHandles
    pub fn set_batch_handles(&mut self, value: u64) {
        self.batch_handles = Some(value);
    }

    /// Gets the value of BatchHandles
    pub fn get_batch_handles(&self) -> Option<&u64> {
        self.batch_handles.as_ref()
    }

    /// Sets the value of BatchHandlesPersec
    pub fn set_batch_handles_persec(&mut self, value: u64) {
        self.batch_handles_persec = Some(value);
    }

    /// Gets the value of BatchHandlesPersec
    pub fn get_batch_handles_persec(&self) -> Option<&u64> {
        self.batch_handles_persec.as_ref()
    }

    /// Sets the value of ClusterHandles
    pub fn set_cluster_handles(&mut self, value: u64) {
        self.cluster_handles = Some(value);
    }

    /// Gets the value of ClusterHandles
    pub fn get_cluster_handles(&self) -> Option<&u64> {
        self.cluster_handles.as_ref()
    }

    /// Sets the value of ClusterHandlesPersec
    pub fn set_cluster_handles_persec(&mut self, value: u64) {
        self.cluster_handles_persec = Some(value);
    }

    /// Gets the value of ClusterHandlesPersec
    pub fn get_cluster_handles_persec(&self) -> Option<&u64> {
        self.cluster_handles_persec.as_ref()
    }

    /// Sets the value of GroupHandles
    pub fn set_group_handles(&mut self, value: u64) {
        self.group_handles = Some(value);
    }

    /// Gets the value of GroupHandles
    pub fn get_group_handles(&self) -> Option<&u64> {
        self.group_handles.as_ref()
    }

    /// Sets the value of GroupHandlesPersec
    pub fn set_group_handles_persec(&mut self, value: u64) {
        self.group_handles_persec = Some(value);
    }

    /// Gets the value of GroupHandlesPersec
    pub fn get_group_handles_persec(&self) -> Option<&u64> {
        self.group_handles_persec.as_ref()
    }

    /// Sets the value of KeyHandles
    pub fn set_key_handles(&mut self, value: u64) {
        self.key_handles = Some(value);
    }

    /// Gets the value of KeyHandles
    pub fn get_key_handles(&self) -> Option<&u64> {
        self.key_handles.as_ref()
    }

    /// Sets the value of KeyHandlesPersec
    pub fn set_key_handles_persec(&mut self, value: u64) {
        self.key_handles_persec = Some(value);
    }

    /// Gets the value of KeyHandlesPersec
    pub fn get_key_handles_persec(&self) -> Option<&u64> {
        self.key_handles_persec.as_ref()
    }

    /// Sets the value of NetworkHandles
    pub fn set_network_handles(&mut self, value: u64) {
        self.network_handles = Some(value);
    }

    /// Gets the value of NetworkHandles
    pub fn get_network_handles(&self) -> Option<&u64> {
        self.network_handles.as_ref()
    }

    /// Sets the value of NetworkHandlesPersec
    pub fn set_network_handles_persec(&mut self, value: u64) {
        self.network_handles_persec = Some(value);
    }

    /// Gets the value of NetworkHandlesPersec
    pub fn get_network_handles_persec(&self) -> Option<&u64> {
        self.network_handles_persec.as_ref()
    }

    /// Sets the value of NetworkInterfaceHandles
    pub fn set_network_interface_handles(&mut self, value: u64) {
        self.network_interface_handles = Some(value);
    }

    /// Gets the value of NetworkInterfaceHandles
    pub fn get_network_interface_handles(&self) -> Option<&u64> {
        self.network_interface_handles.as_ref()
    }

    /// Sets the value of NetworkInterfaceHandlesPersec
    pub fn set_network_interface_handles_persec(&mut self, value: u64) {
        self.network_interface_handles_persec = Some(value);
    }

    /// Gets the value of NetworkInterfaceHandlesPersec
    pub fn get_network_interface_handles_persec(&self) -> Option<&u64> {
        self.network_interface_handles_persec.as_ref()
    }

    /// Sets the value of NodeHandles
    pub fn set_node_handles(&mut self, value: u64) {
        self.node_handles = Some(value);
    }

    /// Gets the value of NodeHandles
    pub fn get_node_handles(&self) -> Option<&u64> {
        self.node_handles.as_ref()
    }

    /// Sets the value of NodeHandlesPersec
    pub fn set_node_handles_persec(&mut self, value: u64) {
        self.node_handles_persec = Some(value);
    }

    /// Gets the value of NodeHandlesPersec
    pub fn get_node_handles_persec(&self) -> Option<&u64> {
        self.node_handles_persec.as_ref()
    }

    /// Sets the value of NotificationHandles
    pub fn set_notification_handles(&mut self, value: u64) {
        self.notification_handles = Some(value);
    }

    /// Gets the value of NotificationHandles
    pub fn get_notification_handles(&self) -> Option<&u64> {
        self.notification_handles.as_ref()
    }

    /// Sets the value of NotificationHandlesPersec
    pub fn set_notification_handles_persec(&mut self, value: u64) {
        self.notification_handles_persec = Some(value);
    }

    /// Gets the value of NotificationHandlesPersec
    pub fn get_notification_handles_persec(&self) -> Option<&u64> {
        self.notification_handles_persec.as_ref()
    }

    /// Sets the value of ResourceHandles
    pub fn set_resource_handles(&mut self, value: u64) {
        self.resource_handles = Some(value);
    }

    /// Gets the value of ResourceHandles
    pub fn get_resource_handles(&self) -> Option<&u64> {
        self.resource_handles.as_ref()
    }

    /// Sets the value of ResourceHandlesPersec
    pub fn set_resource_handles_persec(&mut self, value: u64) {
        self.resource_handles_persec = Some(value);
    }

    /// Gets the value of ResourceHandlesPersec
    pub fn get_resource_handles_persec(&self) -> Option<&u64> {
        self.resource_handles_persec.as_ref()
    }
}

