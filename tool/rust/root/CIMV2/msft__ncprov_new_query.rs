// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NCProvNewQuery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NCProvNewQuery {
    #[serde(flatten)]
    pub base: MSFT_NCProvEvent,

/// 
    #[serde(rename = "ID")]
    pub id: Option<u32>,

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,
}

impl MSFT_NCProvNewQuery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NCProvEvent::new(),
            id: None,
            query: None,
            query_language: None,
        }
    }


    /// Sets the value of ID
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
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

