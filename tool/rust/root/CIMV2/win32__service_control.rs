// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ServiceControl struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ServiceControl {
    #[serde(flatten)]
    pub base: Win32_MSIResource,

/// 
    #[serde(rename = "Arguments")]
    pub arguments: Option<String>,

/// 
    #[serde(rename = "Event")]
    pub event: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProductCode")]
    pub product_code: Option<String>,

/// 
    #[serde(rename = "Wait")]
    pub wait: Option<u16>,
}

impl Win32_ServiceControl {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_MSIResource::new(),
            arguments: None,
            event: None,
            id: None,
            name: None,
            product_code: None,
            wait: None,
        }
    }


    /// Sets the value of Arguments
    pub fn set_arguments(&mut self, value: String) {
        self.arguments = Some(value);
    }

    /// Gets the value of Arguments
    pub fn get_arguments(&self) -> Option<&String> {
        self.arguments.as_ref()
    }

    /// Sets the value of Event
    pub fn set_event(&mut self, value: String) {
        self.event = Some(value);
    }

    /// Gets the value of Event
    pub fn get_event(&self) -> Option<&String> {
        self.event.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProductCode
    pub fn set_product_code(&mut self, value: String) {
        self.product_code = Some(value);
    }

    /// Gets the value of ProductCode
    pub fn get_product_code(&self) -> Option<&String> {
        self.product_code.as_ref()
    }

    /// Sets the value of Wait
    pub fn set_wait(&mut self, value: u16) {
        self.wait = Some(value);
    }

    /// Gets the value of Wait
    pub fn get_wait(&self) -> Option<&u16> {
        self.wait.as_ref()
    }
}

