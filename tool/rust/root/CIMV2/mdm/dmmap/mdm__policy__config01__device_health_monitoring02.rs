// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_DeviceHealthMonitoring02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_DeviceHealthMonitoring02 {

/// 
    #[serde(rename = "AllowDeviceHealthMonitoring")]
    pub allow_device_health_monitoring: Option<i32>,

/// 
    #[serde(rename = "ConfigDeviceHealthMonitoringScope")]
    pub config_device_health_monitoring_scope: Option<String>,

/// 
    #[serde(rename = "ConfigDeviceHealthMonitoringServiceInstance")]
    pub config_device_health_monitoring_service_instance: Option<String>,

/// 
    #[serde(rename = "ConfigDeviceHealthMonitoringUploadDestination")]
    pub config_device_health_monitoring_upload_destination: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Config01_DeviceHealthMonitoring02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_device_health_monitoring: None,
            config_device_health_monitoring_scope: None,
            config_device_health_monitoring_service_instance: None,
            config_device_health_monitoring_upload_destination: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowDeviceHealthMonitoring
    pub fn set_allow_device_health_monitoring(&mut self, value: i32) {
        self.allow_device_health_monitoring = Some(value);
    }

    /// Gets the value of AllowDeviceHealthMonitoring
    pub fn get_allow_device_health_monitoring(&self) -> Option<&i32> {
        self.allow_device_health_monitoring.as_ref()
    }

    /// Sets the value of ConfigDeviceHealthMonitoringScope
    pub fn set_config_device_health_monitoring_scope(&mut self, value: String) {
        self.config_device_health_monitoring_scope = Some(value);
    }

    /// Gets the value of ConfigDeviceHealthMonitoringScope
    pub fn get_config_device_health_monitoring_scope(&self) -> Option<&String> {
        self.config_device_health_monitoring_scope.as_ref()
    }

    /// Sets the value of ConfigDeviceHealthMonitoringServiceInstance
    pub fn set_config_device_health_monitoring_service_instance(&mut self, value: String) {
        self.config_device_health_monitoring_service_instance = Some(value);
    }

    /// Gets the value of ConfigDeviceHealthMonitoringServiceInstance
    pub fn get_config_device_health_monitoring_service_instance(&self) -> Option<&String> {
        self.config_device_health_monitoring_service_instance.as_ref()
    }

    /// Sets the value of ConfigDeviceHealthMonitoringUploadDestination
    pub fn set_config_device_health_monitoring_upload_destination(&mut self, value: String) {
        self.config_device_health_monitoring_upload_destination = Some(value);
    }

    /// Gets the value of ConfigDeviceHealthMonitoringUploadDestination
    pub fn get_config_device_health_monitoring_upload_destination(&self) -> Option<&String> {
        self.config_device_health_monitoring_upload_destination.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

