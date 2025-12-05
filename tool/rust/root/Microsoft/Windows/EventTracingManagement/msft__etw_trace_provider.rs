// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.EventTracingManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_EtwTraceProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_EtwTraceProvider {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "AutologgerName")]
    pub autologger_name: Option<String>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u8>,

/// 
    #[serde(rename = "MatchAllKeyword")]
    pub match_all_keyword: Option<u64>,

/// 
    #[serde(rename = "MatchAnyKeyword")]
    pub match_any_keyword: Option<u64>,

/// 
    #[serde(rename = "Property")]
    pub property: Option<u32>,

/// 
    #[serde(rename = "SessionName")]
    pub session_name: Option<String>,
}

impl MSFT_EtwTraceProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            autologger_name: None,
            guid: None,
            level: None,
            match_all_keyword: None,
            match_any_keyword: None,
            property: None,
            session_name: None,
        }
    }


    /// Sets the value of AutologgerName
    pub fn set_autologger_name(&mut self, value: String) {
        self.autologger_name = Some(value);
    }

    /// Gets the value of AutologgerName
    pub fn get_autologger_name(&self) -> Option<&String> {
        self.autologger_name.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u8) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u8> {
        self.level.as_ref()
    }

    /// Sets the value of MatchAllKeyword
    pub fn set_match_all_keyword(&mut self, value: u64) {
        self.match_all_keyword = Some(value);
    }

    /// Gets the value of MatchAllKeyword
    pub fn get_match_all_keyword(&self) -> Option<&u64> {
        self.match_all_keyword.as_ref()
    }

    /// Sets the value of MatchAnyKeyword
    pub fn set_match_any_keyword(&mut self, value: u64) {
        self.match_any_keyword = Some(value);
    }

    /// Gets the value of MatchAnyKeyword
    pub fn get_match_any_keyword(&self) -> Option<&u64> {
        self.match_any_keyword.as_ref()
    }

    /// Sets the value of Property
    pub fn set_property(&mut self, value: u32) {
        self.property = Some(value);
    }

    /// Gets the value of Property
    pub fn get_property(&self) -> Option<&u32> {
        self.property.as_ref()
    }

    /// Sets the value of SessionName
    pub fn set_session_name(&mut self, value: String) {
        self.session_name = Some(value);
    }

    /// Gets the value of SessionName
    pub fn get_session_name(&self) -> Option<&String> {
        self.session_name.as_ref()
    }
}

