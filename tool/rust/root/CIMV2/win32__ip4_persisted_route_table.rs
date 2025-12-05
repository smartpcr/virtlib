// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_IP4PersistedRouteTable struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_IP4PersistedRouteTable {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

/// 
    #[serde(rename = "Mask")]
    pub mask: Option<String>,

/// 
    #[serde(rename = "Metric1")]
    pub metric1: Option<i32>,

/// 
    #[serde(rename = "NextHop")]
    pub next_hop: Option<String>,
}

impl Win32_IP4PersistedRouteTable {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            destination: None,
            mask: None,
            metric1: None,
            next_hop: None,
        }
    }


    /// Sets the value of Destination
    pub fn set_destination(&mut self, value: String) {
        self.destination = Some(value);
    }

    /// Gets the value of Destination
    pub fn get_destination(&self) -> Option<&String> {
        self.destination.as_ref()
    }

    /// Sets the value of Mask
    pub fn set_mask(&mut self, value: String) {
        self.mask = Some(value);
    }

    /// Gets the value of Mask
    pub fn get_mask(&self) -> Option<&String> {
        self.mask.as_ref()
    }

    /// Sets the value of Metric1
    pub fn set_metric1(&mut self, value: i32) {
        self.metric1 = Some(value);
    }

    /// Gets the value of Metric1
    pub fn get_metric1(&self) -> Option<&i32> {
        self.metric1.as_ref()
    }

    /// Sets the value of NextHop
    pub fn set_next_hop(&mut self, value: String) {
        self.next_hop = Some(value);
    }

    /// Gets the value of NextHop
    pub fn get_next_hop(&self) -> Option<&String> {
        self.next_hop.as_ref()
    }
}

