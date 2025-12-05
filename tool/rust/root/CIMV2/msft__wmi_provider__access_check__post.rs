// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_AccessCheck_Post struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_AccessCheck_Post {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Post,

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,

/// 
    #[serde(rename = "Result")]
    pub result: Option<u32>,

/// 
    #[serde(rename = "Sid")]
    pub sid: Vec<u8>,
}

impl Msft_WmiProvider_AccessCheck_Post {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Post::new(),
            query: None,
            query_language: None,
            result: None,
            sid: Vec::new(),
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

    /// Sets the value of Result
    pub fn set_result(&mut self, value: u32) {
        self.result = Some(value);
    }

    /// Gets the value of Result
    pub fn get_result(&self) -> Option<&u32> {
        self.result.as_ref()
    }

    /// Sets the value of Sid
    pub fn set_sid(&mut self, value: Vec<u8>) {
        self.sid = value;
    }

    /// Gets the value of Sid
    pub fn get_sid(&self) -> &Vec<u8> {
        &self.sid
    }
}

