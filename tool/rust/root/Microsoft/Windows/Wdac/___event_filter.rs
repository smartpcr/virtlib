// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventFilter {
    #[serde(flatten)]
    pub base: __IndicationRelated,

/// 
    #[serde(rename = "CreatorSID")]
    pub creator_sid: Vec<u8>,

/// 
    #[serde(rename = "EventAccess")]
    pub event_access: Option<String>,

/// 
    #[serde(rename = "EventNamespace")]
    pub event_namespace: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,
}

impl __EventFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
            creator_sid: Vec::new(),
            event_access: None,
            event_namespace: None,
            name: None,
            query: None,
            query_language: None,
        }
    }


    /// Sets the value of CreatorSID
    pub fn set_creator_sid(&mut self, value: Vec<u8>) {
        self.creator_sid = value;
    }

    /// Gets the value of CreatorSID
    pub fn get_creator_sid(&self) -> &Vec<u8> {
        &self.creator_sid
    }

    /// Sets the value of EventAccess
    pub fn set_event_access(&mut self, value: String) {
        self.event_access = Some(value);
    }

    /// Gets the value of EventAccess
    pub fn get_event_access(&self) -> Option<&String> {
        self.event_access.as_ref()
    }

    /// Sets the value of EventNamespace
    pub fn set_event_namespace(&mut self, value: String) {
        self.event_namespace = Some(value);
    }

    /// Gets the value of EventNamespace
    pub fn get_event_namespace(&self) -> Option<&String> {
        self.event_namespace.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Query
    pub fn set_query(&mut self, value: String) {
        self.query = Some(value);
    }

    /// Gets the value of Query
    pub fn get_query(&self) -> Option<&String> {
        self.query.as_ref()
    }

    /// Sets the value of QueryLanguage
    pub fn set_query_language(&mut self, value: String) {
        self.query_language = Some(value);
    }

    /// Gets the value of QueryLanguage
    pub fn get_query_language(&self) -> Option<&String> {
        self.query_language.as_ref()
    }
}

