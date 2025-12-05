// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.DEFAULT
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NTEventLogEventConsumer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NTEventLogEventConsumer {
    #[serde(flatten)]
    pub base: __EventConsumer,

/// 
    #[serde(rename = "Category")]
    pub category: Option<u16>,

/// 
    #[serde(rename = "EventID")]
    pub event_id: Option<u32>,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<NTEventLogEventConsumer_EventType>,

/// 
    #[serde(rename = "InsertionStringTemplates")]
    pub insertion_string_templates: Vec<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NameOfRawDataProperty")]
    pub name_of_raw_data_property: Option<String>,

/// 
    #[serde(rename = "NameOfUserSIDProperty")]
    pub name_of_user_sidproperty: Option<String>,

/// 
    #[serde(rename = "NumberOfInsertionStrings")]
    pub number_of_insertion_strings: Option<u32>,

/// 
    #[serde(rename = "SourceName")]
    pub source_name: Option<String>,

/// 
    #[serde(rename = "UNCServerName")]
    pub uncserver_name: Option<String>,
}

impl NTEventLogEventConsumer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventConsumer::new(),
            category: None,
            event_id: None,
            event_type: None,
            insertion_string_templates: Vec::new(),
            name: None,
            name_of_raw_data_property: None,
            name_of_user_sidproperty: None,
            number_of_insertion_strings: None,
            source_name: None,
            uncserver_name: None,
        }
    }


    /// Sets the value of Category
    pub fn set_category(&mut self, value: u16) {
        self.category = Some(value);
    }

    /// Gets the value of Category
    pub fn get_category(&self) -> Option<&u16> {
        self.category.as_ref()
    }

    /// Sets the value of EventID
    pub fn set_event_id(&mut self, value: u32) {
        self.event_id = Some(value);
    }

    /// Gets the value of EventID
    pub fn get_event_id(&self) -> Option<&u32> {
        self.event_id.as_ref()
    }

    /// Sets the value of EventType
    pub fn set_event_type(&mut self, value: NTEventLogEventConsumer_EventType) {
        self.event_type = Some(value);
    }

    /// Gets the value of EventType
    pub fn get_event_type(&self) -> Option<&NTEventLogEventConsumer_EventType> {
        self.event_type.as_ref()
    }

    /// Sets the value of InsertionStringTemplates
    pub fn set_insertion_string_templates(&mut self, value: Vec<String>) {
        self.insertion_string_templates = value;
    }

    /// Gets the value of InsertionStringTemplates
    pub fn get_insertion_string_templates(&self) -> &Vec<String> {
        &self.insertion_string_templates
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NameOfRawDataProperty
    pub fn set_name_of_raw_data_property(&mut self, value: String) {
        self.name_of_raw_data_property = Some(value);
    }

    /// Gets the value of NameOfRawDataProperty
    pub fn get_name_of_raw_data_property(&self) -> Option<&String> {
        self.name_of_raw_data_property.as_ref()
    }

    /// Sets the value of NameOfUserSIDProperty
    pub fn set_name_of_user_sidproperty(&mut self, value: String) {
        self.name_of_user_sidproperty = Some(value);
    }

    /// Gets the value of NameOfUserSIDProperty
    pub fn get_name_of_user_sidproperty(&self) -> Option<&String> {
        self.name_of_user_sidproperty.as_ref()
    }

    /// Sets the value of NumberOfInsertionStrings
    pub fn set_number_of_insertion_strings(&mut self, value: u32) {
        self.number_of_insertion_strings = Some(value);
    }

    /// Gets the value of NumberOfInsertionStrings
    pub fn get_number_of_insertion_strings(&self) -> Option<&u32> {
        self.number_of_insertion_strings.as_ref()
    }

    /// Sets the value of SourceName
    pub fn set_source_name(&mut self, value: String) {
        self.source_name = Some(value);
    }

    /// Gets the value of SourceName
    pub fn get_source_name(&self) -> Option<&String> {
        self.source_name.as_ref()
    }

    /// Sets the value of UNCServerName
    pub fn set_uncserver_name(&mut self, value: String) {
        self.uncserver_name = Some(value);
    }

    /// Gets the value of UNCServerName
    pub fn get_uncserver_name(&self) -> Option<&String> {
        self.uncserver_name.as_ref()
    }
}

