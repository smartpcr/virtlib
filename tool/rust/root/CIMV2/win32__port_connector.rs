// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PortConnector struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PortConnector {
    #[serde(flatten)]
    pub base: CIM_PhysicalConnector,

/// 
    #[serde(rename = "ExternalReferenceDesignator")]
    pub external_reference_designator: Option<String>,

/// 
    #[serde(rename = "InternalReferenceDesignator")]
    pub internal_reference_designator: Option<String>,

/// 
    #[serde(rename = "PortType")]
    pub port_type: Option<u16>,
}

impl Win32_PortConnector {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalConnector::new(),
            external_reference_designator: None,
            internal_reference_designator: None,
            port_type: None,
        }
    }


    /// Sets the value of ExternalReferenceDesignator
    pub fn set_external_reference_designator(&mut self, value: String) {
        self.external_reference_designator = Some(value);
    }

    /// Gets the value of ExternalReferenceDesignator
    pub fn get_external_reference_designator(&self) -> Option<&String> {
        self.external_reference_designator.as_ref()
    }

    /// Sets the value of InternalReferenceDesignator
    pub fn set_internal_reference_designator(&mut self, value: String) {
        self.internal_reference_designator = Some(value);
    }

    /// Gets the value of InternalReferenceDesignator
    pub fn get_internal_reference_designator(&self) -> Option<&String> {
        self.internal_reference_designator.as_ref()
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: u16) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&u16> {
        self.port_type.as_ref()
    }
}

