// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_NewQuery_Pre struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_NewQuery_Pre {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Pre,

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryId")]
    pub query_id: Option<u32>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,
}

impl Msft_WmiProvider_NewQuery_Pre {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Pre::new(),
            query: None,
            query_id: None,
            query_language: None,
        }
    }


    /// Sets the value of Query
    pub fn set_query(&mut self, value: String) {
        self.query = Some(value);
    }

    /// Gets the value of Query
    pub fn get_query(&self) -> Option<&String> {
        self.query.as_ref()
    }

    /// Sets the value of QueryId
    pub fn set_query_id(&mut self, value: u32) {
        self.query_id = Some(value);
    }

    /// Gets the value of QueryId
    pub fn get_query_id(&self) -> Option<&u32> {
        self.query_id.as_ref()
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

