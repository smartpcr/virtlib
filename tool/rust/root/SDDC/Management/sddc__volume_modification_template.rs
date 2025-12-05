// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_VolumeModificationTemplate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_VolumeModificationTemplate {

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Vec<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Resiliency")]
    pub resiliency: Vec<u16>,

/// 
    #[serde(rename = "SizeAvailable")]
    pub size_available: Vec<u64>,
}

impl SDDC_VolumeModificationTemplate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            media_type: Vec::new(),
            name: None,
            resiliency: Vec::new(),
            size_available: Vec::new(),
        }
    }


    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: Vec<u16>) {
        self.media_type = value;
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> &Vec<u16> {
        &self.media_type
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Resiliency
    pub fn set_resiliency(&mut self, value: Vec<u16>) {
        self.resiliency = value;
    }

    /// Gets the value of Resiliency
    pub fn get_resiliency(&self) -> &Vec<u16> {
        &self.resiliency
    }

    /// Sets the value of SizeAvailable
    pub fn set_size_available(&mut self, value: Vec<u64>) {
        self.size_available = value;
    }

    /// Gets the value of SizeAvailable
    pub fn get_size_available(&self) -> &Vec<u64> {
        &self.size_available
    }
}

