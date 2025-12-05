// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Policy
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Rule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Rule {

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,

/// 
    #[serde(rename = "TargetNameSpace")]
    pub target_name_space: Option<String>,
}

impl MSFT_Rule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            query: None,
            query_language: None,
            target_name_space: None,
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

    /// Sets the value of QueryLanguage
    pub fn set_query_language(&mut self, value: String) {
        self.query_language = Some(value);
    }

    /// Gets the value of QueryLanguage
    pub fn get_query_language(&self) -> Option<&String> {
        self.query_language.as_ref()
    }

    /// Sets the value of TargetNameSpace
    pub fn set_target_name_space(&mut self, value: String) {
        self.target_name_space = Some(value);
    }

    /// Gets the value of TargetNameSpace
    pub fn get_target_name_space(&self) -> Option<&String> {
        self.target_name_space.as_ref()
    }
}

