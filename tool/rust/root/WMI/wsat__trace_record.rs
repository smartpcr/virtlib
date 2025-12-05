// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSAT_TraceRecord struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSAT_TraceRecord {
    #[serde(flatten)]
    pub base: WSAT_TraceEvent,

/// Activity ID
    #[serde(rename = "ActivityID")]
    pub activity_id: Option<serde_json::Value>,

/// EventID
    #[serde(rename = "EventID")]
    pub event_id: Option<i32>,

/// Trace Record
    #[serde(rename = "TraceRecord")]
    pub trace_record: Option<String>,
}

impl WSAT_TraceRecord {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WSAT_TraceEvent::new(),
            activity_id: None,
            event_id: None,
            trace_record: None,
        }
    }


    /// Sets the value of ActivityID
    pub fn set_activity_id(&mut self, value: serde_json::Value) {
        self.activity_id = Some(value);
    }

    /// Gets the value of ActivityID
    pub fn get_activity_id(&self) -> Option<&serde_json::Value> {
        self.activity_id.as_ref()
    }

    /// Sets the value of EventID
    pub fn set_event_id(&mut self, value: i32) {
        self.event_id = Some(value);
    }

    /// Gets the value of EventID
    pub fn get_event_id(&self) -> Option<&i32> {
        self.event_id.as_ref()
    }

    /// Sets the value of TraceRecord
    pub fn set_trace_record(&mut self, value: String) {
        self.trace_record = Some(value);
    }

    /// Gets the value of TraceRecord
    pub fn get_trace_record(&self) -> Option<&String> {
        self.trace_record.as_ref()
    }
}

