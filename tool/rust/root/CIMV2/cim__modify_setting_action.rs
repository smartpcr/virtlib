// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ModifySettingAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ModifySettingAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "ActionType")]
    pub action_type: Option<u16>,

/// 
    #[serde(rename = "EntryName")]
    pub entry_name: Option<String>,

/// 
    #[serde(rename = "EntryValue")]
    pub entry_value: Option<String>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "SectionKey")]
    pub section_key: Option<String>,
}

impl CIM_ModifySettingAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            action_type: None,
            entry_name: None,
            entry_value: None,
            file_name: None,
            section_key: None,
        }
    }


    /// Sets the value of ActionType
    pub fn set_action_type(&mut self, value: u16) {
        self.action_type = Some(value);
    }

    /// Gets the value of ActionType
    pub fn get_action_type(&self) -> Option<&u16> {
        self.action_type.as_ref()
    }

    /// Sets the value of EntryName
    pub fn set_entry_name(&mut self, value: String) {
        self.entry_name = Some(value);
    }

    /// Gets the value of EntryName
    pub fn get_entry_name(&self) -> Option<&String> {
        self.entry_name.as_ref()
    }

    /// Sets the value of EntryValue
    pub fn set_entry_value(&mut self, value: String) {
        self.entry_value = Some(value);
    }

    /// Gets the value of EntryValue
    pub fn get_entry_value(&self) -> Option<&String> {
        self.entry_value.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of SectionKey
    pub fn set_section_key(&mut self, value: String) {
        self.section_key = Some(value);
    }

    /// Gets the value of SectionKey
    pub fn get_section_key(&self) -> Option<&String> {
        self.section_key.as_ref()
    }
}

