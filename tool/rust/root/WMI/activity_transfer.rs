// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ActivityTransfer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTransfer {
    #[serde(flatten)]
    pub base: WSAT_TraceEvent,

/// Activity ID
    #[serde(rename = "ActivityID")]
    pub activity_id: Option<serde_json::Value>,

/// Related Activity ID
    #[serde(rename = "RelatedActivityID")]
    pub related_activity_id: Option<serde_json::Value>,
}

impl ActivityTransfer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WSAT_TraceEvent::new(),
            activity_id: None,
            related_activity_id: None,
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

    /// Sets the value of RelatedActivityID
    pub fn set_related_activity_id(&mut self, value: serde_json::Value) {
        self.related_activity_id = Some(value);
    }

    /// Gets the value of RelatedActivityID
    pub fn get_related_activity_id(&self) -> Option<&serde_json::Value> {
        self.related_activity_id.as_ref()
    }
}

