// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NextHopRoute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NextHopRoute {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AdminDistance")]
    pub admin_distance: Option<u16>,

/// 
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Option<String>,

/// 
    #[serde(rename = "IsStatic")]
    pub is_static: Option<bool>,

/// 
    #[serde(rename = "RouteMetric")]
    pub route_metric: Option<u16>,

/// 
    #[serde(rename = "TypeOfRoute")]
    pub type_of_route: Option<u16>,
}

impl CIM_NextHopRoute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            admin_distance: None,
            destination_address: None,
            is_static: None,
            route_metric: None,
            type_of_route: None,
        }
    }


    /// Sets the value of AdminDistance
    pub fn set_admin_distance(&mut self, value: u16) {
        self.admin_distance = Some(value);
    }

    /// Gets the value of AdminDistance
    pub fn get_admin_distance(&self) -> Option<&u16> {
        self.admin_distance.as_ref()
    }

    /// Sets the value of DestinationAddress
    pub fn set_destination_address(&mut self, value: String) {
        self.destination_address = Some(value);
    }

    /// Gets the value of DestinationAddress
    pub fn get_destination_address(&self) -> Option<&String> {
        self.destination_address.as_ref()
    }

    /// Sets the value of IsStatic
    pub fn set_is_static(&mut self, value: bool) {
        self.is_static = Some(value);
    }

    /// Gets the value of IsStatic
    pub fn get_is_static(&self) -> Option<&bool> {
        self.is_static.as_ref()
    }

    /// Sets the value of RouteMetric
    pub fn set_route_metric(&mut self, value: u16) {
        self.route_metric = Some(value);
    }

    /// Gets the value of RouteMetric
    pub fn get_route_metric(&self) -> Option<&u16> {
        self.route_metric.as_ref()
    }

    /// Sets the value of TypeOfRoute
    pub fn set_type_of_route(&mut self, value: u16) {
        self.type_of_route = Some(value);
    }

    /// Gets the value of TypeOfRoute
    pub fn get_type_of_route(&self) -> Option<&u16> {
        self.type_of_route.as_ref()
    }
}

