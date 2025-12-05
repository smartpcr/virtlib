// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_IP4RouteTable struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_IP4RouteTable {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Age")]
    pub age: Option<u32>,

/// 
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

/// 
    #[serde(rename = "Information")]
    pub information: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<i32>,

/// 
    #[serde(rename = "Mask")]
    pub mask: Option<String>,

/// 
    #[serde(rename = "Metric1")]
    pub metric1: Option<i32>,

/// 
    #[serde(rename = "Metric2")]
    pub metric2: Option<i32>,

/// 
    #[serde(rename = "Metric3")]
    pub metric3: Option<i32>,

/// 
    #[serde(rename = "Metric4")]
    pub metric4: Option<i32>,

/// 
    #[serde(rename = "Metric5")]
    pub metric5: Option<i32>,

/// 
    #[serde(rename = "NextHop")]
    pub next_hop: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl Win32_IP4RouteTable {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            age: None,
            destination: None,
            information: None,
            interface_index: None,
            mask: None,
            metric1: None,
            metric2: None,
            metric3: None,
            metric4: None,
            metric5: None,
            next_hop: None,
            protocol: None,
            type: None,
        }
    }


    /// Sets the value of Age
    pub fn set_age(&mut self, value: u32) {
        self.age = Some(value);
    }

    /// Gets the value of Age
    pub fn get_age(&self) -> Option<&u32> {
        self.age.as_ref()
    }

    /// Sets the value of Destination
    pub fn set_destination(&mut self, value: String) {
        self.destination = Some(value);
    }

    /// Gets the value of Destination
    pub fn get_destination(&self) -> Option<&String> {
        self.destination.as_ref()
    }

    /// Sets the value of Information
    pub fn set_information(&mut self, value: String) {
        self.information = Some(value);
    }

    /// Gets the value of Information
    pub fn get_information(&self) -> Option<&String> {
        self.information.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: i32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&i32> {
        self.interface_index.as_ref()
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

    /// Sets the value of Metric2
    pub fn set_metric2(&mut self, value: i32) {
        self.metric2 = Some(value);
    }

    /// Gets the value of Metric2
    pub fn get_metric2(&self) -> Option<&i32> {
        self.metric2.as_ref()
    }

    /// Sets the value of Metric3
    pub fn set_metric3(&mut self, value: i32) {
        self.metric3 = Some(value);
    }

    /// Gets the value of Metric3
    pub fn get_metric3(&self) -> Option<&i32> {
        self.metric3.as_ref()
    }

    /// Sets the value of Metric4
    pub fn set_metric4(&mut self, value: i32) {
        self.metric4 = Some(value);
    }

    /// Gets the value of Metric4
    pub fn get_metric4(&self) -> Option<&i32> {
        self.metric4.as_ref()
    }

    /// Sets the value of Metric5
    pub fn set_metric5(&mut self, value: i32) {
        self.metric5 = Some(value);
    }

    /// Gets the value of Metric5
    pub fn get_metric5(&self) -> Option<&i32> {
        self.metric5.as_ref()
    }

    /// Sets the value of NextHop
    pub fn set_next_hop(&mut self, value: String) {
        self.next_hop = Some(value);
    }

    /// Gets the value of NextHop
    pub fn get_next_hop(&self) -> Option<&String> {
        self.next_hop.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: u32) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&u32> {
        self.protocol.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

