// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSMouse_PortInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSMouse_PortInformation {
    #[serde(flatten)]
    pub base: MSMouse,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Buttons")]
    pub buttons: Option<u32>,

/// 
    #[serde(rename = "ConnectorType")]
    pub connector_type: Option<u32>,

/// 
    #[serde(rename = "DataQueueSize")]
    pub data_queue_size: Option<u32>,

/// 
    #[serde(rename = "ErrorCount")]
    pub error_count: Option<u32>,

/// 
    #[serde(rename = "HardwareType")]
    pub hardware_type: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSMouse_PortInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMouse::new(),
            active: None,
            buttons: None,
            connector_type: None,
            data_queue_size: None,
            error_count: None,
            hardware_type: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of Buttons
    pub fn set_buttons(&mut self, value: u32) {
        self.buttons = Some(value);
    }

    /// Gets the value of Buttons
    pub fn get_buttons(&self) -> Option<&u32> {
        self.buttons.as_ref()
    }

    /// Sets the value of ConnectorType
    pub fn set_connector_type(&mut self, value: u32) {
        self.connector_type = Some(value);
    }

    /// Gets the value of ConnectorType
    pub fn get_connector_type(&self) -> Option<&u32> {
        self.connector_type.as_ref()
    }

    /// Sets the value of DataQueueSize
    pub fn set_data_queue_size(&mut self, value: u32) {
        self.data_queue_size = Some(value);
    }

    /// Gets the value of DataQueueSize
    pub fn get_data_queue_size(&self) -> Option<&u32> {
        self.data_queue_size.as_ref()
    }

    /// Sets the value of ErrorCount
    pub fn set_error_count(&mut self, value: u32) {
        self.error_count = Some(value);
    }

    /// Gets the value of ErrorCount
    pub fn get_error_count(&self) -> Option<&u32> {
        self.error_count.as_ref()
    }

    /// Sets the value of HardwareType
    pub fn set_hardware_type(&mut self, value: u32) {
        self.hardware_type = Some(value);
    }

    /// Gets the value of HardwareType
    pub fn get_hardware_type(&self) -> Option<&u32> {
        self.hardware_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }
}

