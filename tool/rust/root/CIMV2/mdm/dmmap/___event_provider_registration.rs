// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventProviderRegistration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventProviderRegistration {
    #[serde(flatten)]
    pub base: __ProviderRegistration,

/// 
    #[serde(rename = "EventQueryList")]
    pub event_query_list: Vec<String>,
}

impl __EventProviderRegistration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ProviderRegistration::new(),
            event_query_list: Vec::new(),
        }
    }


    /// Sets the value of EventQueryList
    pub fn set_event_query_list(&mut self, value: Vec<String>) {
        self.event_query_list = value;
    }

    /// Gets the value of EventQueryList
    pub fn get_event_query_list(&self) -> &Vec<String> {
        &self.event_query_list
    }
}

