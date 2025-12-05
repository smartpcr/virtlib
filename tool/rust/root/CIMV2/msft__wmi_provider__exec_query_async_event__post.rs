// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_ExecQueryAsyncEvent_Post struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_ExecQueryAsyncEvent_Post {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Post,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "ObjectParameter")]
    pub object_parameter: Option<serde_json::Value>,

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,

/// 
    #[serde(rename = "ResultCode")]
    pub result_code: Option<u32>,

/// 
    #[serde(rename = "StringParameter")]
    pub string_parameter: Option<String>,
}

impl Msft_WmiProvider_ExecQueryAsyncEvent_Post {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Post::new(),
            flags: None,
            object_parameter: None,
            query: None,
            query_language: None,
            result_code: None,
            string_parameter: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of ObjectParameter
    pub fn set_object_parameter(&mut self, value: serde_json::Value) {
        self.object_parameter = Some(value);
    }

    /// Gets the value of ObjectParameter
    pub fn get_object_parameter(&self) -> Option<&serde_json::Value> {
        self.object_parameter.as_ref()
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

    /// Sets the value of ResultCode
    pub fn set_result_code(&mut self, value: u32) {
        self.result_code = Some(value);
    }

    /// Gets the value of ResultCode
    pub fn get_result_code(&self) -> Option<&u32> {
        self.result_code.as_ref()
    }

    /// Sets the value of StringParameter
    pub fn set_string_parameter(&mut self, value: String) {
        self.string_parameter = Some(value);
    }

    /// Gets the value of StringParameter
    pub fn get_string_parameter(&self) -> Option<&String> {
        self.string_parameter.as_ref()
    }
}

