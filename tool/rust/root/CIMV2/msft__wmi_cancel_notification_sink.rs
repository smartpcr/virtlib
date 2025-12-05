// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiCancelNotificationSink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiCancelNotificationSink {
    #[serde(flatten)]
    pub base: MSFT_WmiEssEvent,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "Query")]
    pub query: Option<String>,

/// 
    #[serde(rename = "QueryLanguage")]
    pub query_language: Option<String>,

/// 
    #[serde(rename = "Sink")]
    pub sink: Option<u64>,
}

impl MSFT_WmiCancelNotificationSink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiEssEvent::new(),
            namespace: None,
            query: None,
            query_language: None,
            sink: None,
        }
    }


    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
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

    /// Sets the value of Sink
    pub fn set_sink(&mut self, value: u64) {
        self.sink = Some(value);
    }

    /// Gets the value of Sink
    pub fn get_sink(&self) -> Option<&u64> {
        self.sink.as_ref()
    }
}

