// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSKeyboard_PortInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSKeyboard_PortInformation {
    #[serde(flatten)]
    pub base: MSKeyboard,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

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
    #[serde(rename = "FunctionKeys")]
    pub function_keys: Option<u32>,

/// 
    #[serde(rename = "Indicators")]
    pub indicators: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSKeyboard_PortInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSKeyboard::new(),
            active: None,
            connector_type: None,
            data_queue_size: None,
            error_count: None,
            function_keys: None,
            indicators: None,
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

    /// Sets the value of FunctionKeys
    pub fn set_function_keys(&mut self, value: u32) {
        self.function_keys = Some(value);
    }

    /// Gets the value of FunctionKeys
    pub fn get_function_keys(&self) -> Option<&u32> {
        self.function_keys.as_ref()
    }

    /// Sets the value of Indicators
    pub fn set_indicators(&mut self, value: u32) {
        self.indicators = Some(value);
    }

    /// Gets the value of Indicators
    pub fn get_indicators(&self) -> Option<&u32> {
        self.indicators.as_ref()
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

