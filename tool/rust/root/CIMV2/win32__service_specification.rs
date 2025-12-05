// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ServiceSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ServiceSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "Dependencies")]
    pub dependencies: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "ErrorControl")]
    pub error_control: Option<i32>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "LoadOrderGroup")]
    pub load_order_group: Option<String>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "ServiceType")]
    pub service_type: Option<i32>,

/// 
    #[serde(rename = "StartName")]
    pub start_name: Option<String>,

/// 
    #[serde(rename = "StartType")]
    pub start_type: Option<i32>,
}

impl Win32_ServiceSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            dependencies: None,
            display_name: None,
            error_control: None,
            id: None,
            load_order_group: None,
            password: None,
            service_type: None,
            start_name: None,
            start_type: None,
        }
    }


    /// Sets the value of Dependencies
    pub fn set_dependencies(&mut self, value: String) {
        self.dependencies = Some(value);
    }

    /// Gets the value of Dependencies
    pub fn get_dependencies(&self) -> Option<&String> {
        self.dependencies.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of ErrorControl
    pub fn set_error_control(&mut self, value: i32) {
        self.error_control = Some(value);
    }

    /// Gets the value of ErrorControl
    pub fn get_error_control(&self) -> Option<&i32> {
        self.error_control.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of LoadOrderGroup
    pub fn set_load_order_group(&mut self, value: String) {
        self.load_order_group = Some(value);
    }

    /// Gets the value of LoadOrderGroup
    pub fn get_load_order_group(&self) -> Option<&String> {
        self.load_order_group.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of ServiceType
    pub fn set_service_type(&mut self, value: i32) {
        self.service_type = Some(value);
    }

    /// Gets the value of ServiceType
    pub fn get_service_type(&self) -> Option<&i32> {
        self.service_type.as_ref()
    }

    /// Sets the value of StartName
    pub fn set_start_name(&mut self, value: String) {
        self.start_name = Some(value);
    }

    /// Gets the value of StartName
    pub fn get_start_name(&self) -> Option<&String> {
        self.start_name.as_ref()
    }

    /// Sets the value of StartType
    pub fn set_start_type(&mut self, value: i32) {
        self.start_type = Some(value);
    }

    /// Gets the value of StartType
    pub fn get_start_type(&self) -> Option<&i32> {
        self.start_type.as_ref()
    }
}

