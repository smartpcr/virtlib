// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterAPICalls struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterAPICalls {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ClusterAPICallsPersec")]
    pub cluster_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "GroupAPICallsPersec")]
    pub group_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "KeyAPICallsPersec")]
    pub key_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "NetworkAPICallsPersec")]
    pub network_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "NetworkInterfaceAPICallsPersec")]
    pub network_interface_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "NodeAPICallsPersec")]
    pub node_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "NotificationAPICallsPersec")]
    pub notification_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "NotificationBatchAPICallsPersec")]
    pub notification_batch_apicalls_persec: Option<u64>,

/// 
    #[serde(rename = "ResourceAPICallsPersec")]
    pub resource_apicalls_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterAPICalls {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            cluster_apicalls_persec: None,
            group_apicalls_persec: None,
            key_apicalls_persec: None,
            network_apicalls_persec: None,
            network_interface_apicalls_persec: None,
            node_apicalls_persec: None,
            notification_apicalls_persec: None,
            notification_batch_apicalls_persec: None,
            resource_apicalls_persec: None,
        }
    }


    /// Sets the value of ClusterAPICallsPersec
    pub fn set_cluster_apicalls_persec(&mut self, value: u64) {
        self.cluster_apicalls_persec = Some(value);
    }

    /// Gets the value of ClusterAPICallsPersec
    pub fn get_cluster_apicalls_persec(&self) -> Option<&u64> {
        self.cluster_apicalls_persec.as_ref()
    }

    /// Sets the value of GroupAPICallsPersec
    pub fn set_group_apicalls_persec(&mut self, value: u64) {
        self.group_apicalls_persec = Some(value);
    }

    /// Gets the value of GroupAPICallsPersec
    pub fn get_group_apicalls_persec(&self) -> Option<&u64> {
        self.group_apicalls_persec.as_ref()
    }

    /// Sets the value of KeyAPICallsPersec
    pub fn set_key_apicalls_persec(&mut self, value: u64) {
        self.key_apicalls_persec = Some(value);
    }

    /// Gets the value of KeyAPICallsPersec
    pub fn get_key_apicalls_persec(&self) -> Option<&u64> {
        self.key_apicalls_persec.as_ref()
    }

    /// Sets the value of NetworkAPICallsPersec
    pub fn set_network_apicalls_persec(&mut self, value: u64) {
        self.network_apicalls_persec = Some(value);
    }

    /// Gets the value of NetworkAPICallsPersec
    pub fn get_network_apicalls_persec(&self) -> Option<&u64> {
        self.network_apicalls_persec.as_ref()
    }

    /// Sets the value of NetworkInterfaceAPICallsPersec
    pub fn set_network_interface_apicalls_persec(&mut self, value: u64) {
        self.network_interface_apicalls_persec = Some(value);
    }

    /// Gets the value of NetworkInterfaceAPICallsPersec
    pub fn get_network_interface_apicalls_persec(&self) -> Option<&u64> {
        self.network_interface_apicalls_persec.as_ref()
    }

    /// Sets the value of NodeAPICallsPersec
    pub fn set_node_apicalls_persec(&mut self, value: u64) {
        self.node_apicalls_persec = Some(value);
    }

    /// Gets the value of NodeAPICallsPersec
    pub fn get_node_apicalls_persec(&self) -> Option<&u64> {
        self.node_apicalls_persec.as_ref()
    }

    /// Sets the value of NotificationAPICallsPersec
    pub fn set_notification_apicalls_persec(&mut self, value: u64) {
        self.notification_apicalls_persec = Some(value);
    }

    /// Gets the value of NotificationAPICallsPersec
    pub fn get_notification_apicalls_persec(&self) -> Option<&u64> {
        self.notification_apicalls_persec.as_ref()
    }

    /// Sets the value of NotificationBatchAPICallsPersec
    pub fn set_notification_batch_apicalls_persec(&mut self, value: u64) {
        self.notification_batch_apicalls_persec = Some(value);
    }

    /// Gets the value of NotificationBatchAPICallsPersec
    pub fn get_notification_batch_apicalls_persec(&self) -> Option<&u64> {
        self.notification_batch_apicalls_persec.as_ref()
    }

    /// Sets the value of ResourceAPICallsPersec
    pub fn set_resource_apicalls_persec(&mut self, value: u64) {
        self.resource_apicalls_persec = Some(value);
    }

    /// Gets the value of ResourceAPICallsPersec
    pub fn get_resource_apicalls_persec(&self) -> Option<&u64> {
        self.resource_apicalls_persec.as_ref()
    }
}

