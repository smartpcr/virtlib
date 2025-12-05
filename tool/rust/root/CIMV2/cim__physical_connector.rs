// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalConnector struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalConnector {
    #[serde(flatten)]
    pub base: CIM_PhysicalElement,

/// 
    #[serde(rename = "ConnectorPinout")]
    pub connector_pinout: Option<String>,

/// 
    #[serde(rename = "ConnectorType")]
    pub connector_type: Vec<u16>,
}

impl CIM_PhysicalConnector {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalElement::new(),
            connector_pinout: None,
            connector_type: Vec::new(),
        }
    }


    /// Sets the value of ConnectorPinout
    pub fn set_connector_pinout(&mut self, value: String) {
        self.connector_pinout = Some(value);
    }

    /// Gets the value of ConnectorPinout
    pub fn get_connector_pinout(&self) -> Option<&String> {
        self.connector_pinout.as_ref()
    }

    /// Sets the value of ConnectorType
    pub fn set_connector_type(&mut self, value: Vec<u16>) {
        self.connector_type = value;
    }

    /// Gets the value of ConnectorType
    pub fn get_connector_type(&self) -> &Vec<u16> {
        &self.connector_type
    }
}

