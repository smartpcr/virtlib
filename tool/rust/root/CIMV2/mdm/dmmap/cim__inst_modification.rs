// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_InstModification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_InstModification {
    #[serde(flatten)]
    pub base: CIM_InstIndication,

/// A copy of the 'previous' instance whose change generated the Indication. PreviousInstance contains 'older' values of an instance's properties (as compared to SourceInstance), selected by the IndicationFilter's Query.
    #[serde(rename = "PreviousInstance")]
    pub previous_instance: Option<serde_json::Value>,
}

impl CIM_InstModification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_InstIndication::new(),
            previous_instance: None,
        }
    }


    /// Sets the value of PreviousInstance
    pub fn set_previous_instance(&mut self, value: serde_json::Value) {
        self.previous_instance = Some(value);
    }

    /// Gets the value of PreviousInstance
    pub fn get_previous_instance(&self) -> Option<&serde_json::Value> {
        self.previous_instance.as_ref()
    }
}

