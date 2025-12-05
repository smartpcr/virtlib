// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Card struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Card {
    #[serde(flatten)]
    pub base: CIM_PhysicalPackage,

/// 
    #[serde(rename = "HostingBoard")]
    pub hosting_board: Option<bool>,

/// 
    #[serde(rename = "RequirementsDescription")]
    pub requirements_description: Option<String>,

/// 
    #[serde(rename = "RequiresDaughterBoard")]
    pub requires_daughter_board: Option<bool>,

/// 
    #[serde(rename = "SlotLayout")]
    pub slot_layout: Option<String>,

/// 
    #[serde(rename = "SpecialRequirements")]
    pub special_requirements: Option<bool>,
}

impl CIM_Card {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalPackage::new(),
            hosting_board: None,
            requirements_description: None,
            requires_daughter_board: None,
            slot_layout: None,
            special_requirements: None,
        }
    }


    /// Sets the value of HostingBoard
    pub fn set_hosting_board(&mut self, value: bool) {
        self.hosting_board = Some(value);
    }

    /// Gets the value of HostingBoard
    pub fn get_hosting_board(&self) -> Option<&bool> {
        self.hosting_board.as_ref()
    }

    /// Sets the value of RequirementsDescription
    pub fn set_requirements_description(&mut self, value: String) {
        self.requirements_description = Some(value);
    }

    /// Gets the value of RequirementsDescription
    pub fn get_requirements_description(&self) -> Option<&String> {
        self.requirements_description.as_ref()
    }

    /// Sets the value of RequiresDaughterBoard
    pub fn set_requires_daughter_board(&mut self, value: bool) {
        self.requires_daughter_board = Some(value);
    }

    /// Gets the value of RequiresDaughterBoard
    pub fn get_requires_daughter_board(&self) -> Option<&bool> {
        self.requires_daughter_board.as_ref()
    }

    /// Sets the value of SlotLayout
    pub fn set_slot_layout(&mut self, value: String) {
        self.slot_layout = Some(value);
    }

    /// Gets the value of SlotLayout
    pub fn get_slot_layout(&self) -> Option<&String> {
        self.slot_layout.as_ref()
    }

    /// Sets the value of SpecialRequirements
    pub fn set_special_requirements(&mut self, value: bool) {
        self.special_requirements = Some(value);
    }

    /// Gets the value of SpecialRequirements
    pub fn get_special_requirements(&self) -> Option<&bool> {
        self.special_requirements.as_ref()
    }
}

