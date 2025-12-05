// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_EventLogSubscriptions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_EventLogSubscriptions {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "EventfilteroperationsPersec")]
    pub eventfilteroperations_persec: Option<u64>,

/// 
    #[serde(rename = "EventsPersec")]
    pub events_persec: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_EventLogSubscriptions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            eventfilteroperations_persec: None,
            events_persec: None,
        }
    }


    /// Sets the value of EventfilteroperationsPersec
    pub fn set_eventfilteroperations_persec(&mut self, value: u64) {
        self.eventfilteroperations_persec = Some(value);
    }

    /// Gets the value of EventfilteroperationsPersec
    pub fn get_eventfilteroperations_persec(&self) -> Option<&u64> {
        self.eventfilteroperations_persec.as_ref()
    }

    /// Sets the value of EventsPersec
    pub fn set_events_persec(&mut self, value: u64) {
        self.events_persec = Some(value);
    }

    /// Gets the value of EventsPersec
    pub fn get_events_persec(&self) -> Option<&u64> {
        self.events_persec.as_ref()
    }
}

