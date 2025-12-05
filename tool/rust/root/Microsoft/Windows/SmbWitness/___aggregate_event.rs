// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SmbWitness
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __AggregateEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __AggregateEvent {
    #[serde(flatten)]
    pub base: __IndicationRelated,

/// 
    #[serde(rename = "NumberOfEvents")]
    pub number_of_events: Option<u32>,

/// 
    #[serde(rename = "Representative")]
    pub representative: Option<serde_json::Value>,
}

impl __AggregateEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
            number_of_events: None,
            representative: None,
        }
    }


    /// Sets the value of NumberOfEvents
    pub fn set_number_of_events(&mut self, value: u32) {
        self.number_of_events = Some(value);
    }

    /// Gets the value of NumberOfEvents
    pub fn get_number_of_events(&self) -> Option<&u32> {
        self.number_of_events.as_ref()
    }

    /// Sets the value of Representative
    pub fn set_representative(&mut self, value: serde_json::Value) {
        self.representative = Some(value);
    }

    /// Gets the value of Representative
    pub fn get_representative(&self) -> Option<&serde_json::Value> {
        self.representative.as_ref()
    }
}

