// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NetworkPipe struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NetworkPipe {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// 
    #[serde(rename = "AggregationBehavior")]
    pub aggregation_behavior: Option<u16>,

/// 
    #[serde(rename = "Directionality")]
    pub directionality: Option<u16>,
}

impl CIM_NetworkPipe {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            aggregation_behavior: None,
            directionality: None,
        }
    }


    /// Sets the value of AggregationBehavior
    pub fn set_aggregation_behavior(&mut self, value: u16) {
        self.aggregation_behavior = Some(value);
    }

    /// Gets the value of AggregationBehavior
    pub fn get_aggregation_behavior(&self) -> Option<&u16> {
        self.aggregation_behavior.as_ref()
    }

    /// Sets the value of Directionality
    pub fn set_directionality(&mut self, value: u16) {
        self.directionality = Some(value);
    }

    /// Gets the value of Directionality
    pub fn get_directionality(&self) -> Option<&u16> {
        self.directionality.as_ref()
    }
}

