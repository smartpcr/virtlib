// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Policy
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SomFilterStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SomFilterStatus {

/// 
    #[serde(rename = "ContainerAvailable")]
    pub container_available: Option<bool>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "SchemaAvailable")]
    pub schema_available: Option<bool>,
}

impl MSFT_SomFilterStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            container_available: None,
            domain: None,
            schema_available: None,
        }
    }


    /// Sets the value of ContainerAvailable
    pub fn set_container_available(&mut self, value: bool) {
        self.container_available = Some(value);
    }

    /// Gets the value of ContainerAvailable
    pub fn get_container_available(&self) -> Option<&bool> {
        self.container_available.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of SchemaAvailable
    pub fn set_schema_available(&mut self, value: bool) {
        self.schema_available = Some(value);
    }

    /// Gets the value of SchemaAvailable
    pub fn get_schema_available(&self) -> Option<&bool> {
        self.schema_available.as_ref()
    }
}

