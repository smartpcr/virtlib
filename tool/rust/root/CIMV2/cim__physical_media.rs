// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalMedia struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalMedia {
    #[serde(flatten)]
    pub base: CIM_PhysicalComponent,

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// 
    #[serde(rename = "CleanerMedia")]
    pub cleaner_media: Option<bool>,

/// 
    #[serde(rename = "MediaDescription")]
    pub media_description: Option<String>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u16>,

/// 
    #[serde(rename = "WriteProtectOn")]
    pub write_protect_on: Option<bool>,
}

impl CIM_PhysicalMedia {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalComponent::new(),
            capacity: None,
            cleaner_media: None,
            media_description: None,
            media_type: None,
            write_protect_on: None,
        }
    }


    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of CleanerMedia
    pub fn set_cleaner_media(&mut self, value: bool) {
        self.cleaner_media = Some(value);
    }

    /// Gets the value of CleanerMedia
    pub fn get_cleaner_media(&self) -> Option<&bool> {
        self.cleaner_media.as_ref()
    }

    /// Sets the value of MediaDescription
    pub fn set_media_description(&mut self, value: String) {
        self.media_description = Some(value);
    }

    /// Gets the value of MediaDescription
    pub fn get_media_description(&self) -> Option<&String> {
        self.media_description.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u16) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u16> {
        self.media_type.as_ref()
    }

    /// Sets the value of WriteProtectOn
    pub fn set_write_protect_on(&mut self, value: bool) {
        self.write_protect_on = Some(value);
    }

    /// Gets the value of WriteProtectOn
    pub fn get_write_protect_on(&self) -> Option<&bool> {
        self.write_protect_on.as_ref()
    }
}

