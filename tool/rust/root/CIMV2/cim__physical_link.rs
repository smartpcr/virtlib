// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalLink {
    #[serde(flatten)]
    pub base: CIM_PhysicalElement,

/// 
    #[serde(rename = "Length")]
    pub length: Option<f64>,

/// 
    #[serde(rename = "MaxLength")]
    pub max_length: Option<f64>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u16>,

/// 
    #[serde(rename = "Wired")]
    pub wired: Option<bool>,
}

impl CIM_PhysicalLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalElement::new(),
            length: None,
            max_length: None,
            media_type: None,
            wired: None,
        }
    }


    /// Sets the value of Length
    pub fn set_length(&mut self, value: f64) {
        self.length = Some(value);
    }

    /// Gets the value of Length
    pub fn get_length(&self) -> Option<&f64> {
        self.length.as_ref()
    }

    /// Sets the value of MaxLength
    pub fn set_max_length(&mut self, value: f64) {
        self.max_length = Some(value);
    }

    /// Gets the value of MaxLength
    pub fn get_max_length(&self) -> Option<&f64> {
        self.max_length.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u16) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u16> {
        self.media_type.as_ref()
    }

    /// Sets the value of Wired
    pub fn set_wired(&mut self, value: bool) {
        self.wired = Some(value);
    }

    /// Gets the value of Wired
    pub fn get_wired(&self) -> Option<&bool> {
        self.wired.as_ref()
    }
}

