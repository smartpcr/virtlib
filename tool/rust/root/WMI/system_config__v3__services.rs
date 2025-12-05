// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V3_Services struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V3_Services {
    #[serde(flatten)]
    pub base: SystemConfig_V3,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "LoadOrderGroup")]
    pub load_order_group: Option<String>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ProcessName")]
    pub process_name: Option<String>,

/// 
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

/// 
    #[serde(rename = "ServiceState")]
    pub service_state: Option<u32>,

/// 
    #[serde(rename = "SubProcessTag")]
    pub sub_process_tag: Option<u32>,

/// 
    #[serde(rename = "SvchostGroup")]
    pub svchost_group: Option<String>,
}

impl SystemConfig_V3_Services {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V3::new(),
            display_name: None,
            load_order_group: None,
            process_id: None,
            process_name: None,
            service_name: None,
            service_state: None,
            sub_process_tag: None,
            svchost_group: None,
        }
    }


    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of LoadOrderGroup
    pub fn set_load_order_group(&mut self, value: String) {
        self.load_order_group = Some(value);
    }

    /// Gets the value of LoadOrderGroup
    pub fn get_load_order_group(&self) -> Option<&String> {
        self.load_order_group.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ProcessName
    pub fn set_process_name(&mut self, value: String) {
        self.process_name = Some(value);
    }

    /// Gets the value of ProcessName
    pub fn get_process_name(&self) -> Option<&String> {
        self.process_name.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    /// Sets the value of ServiceState
    pub fn set_service_state(&mut self, value: u32) {
        self.service_state = Some(value);
    }

    /// Gets the value of ServiceState
    pub fn get_service_state(&self) -> Option<&u32> {
        self.service_state.as_ref()
    }

    /// Sets the value of SubProcessTag
    pub fn set_sub_process_tag(&mut self, value: u32) {
        self.sub_process_tag = Some(value);
    }

    /// Gets the value of SubProcessTag
    pub fn get_sub_process_tag(&self) -> Option<&u32> {
        self.sub_process_tag.as_ref()
    }

    /// Sets the value of SvchostGroup
    pub fn set_svchost_group(&mut self, value: String) {
        self.svchost_group = Some(value);
    }

    /// Gets the value of SvchostGroup
    pub fn get_svchost_group(&self) -> Option<&String> {
        self.svchost_group.as_ref()
    }
}

