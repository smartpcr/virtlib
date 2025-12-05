// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_RouteList02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_RouteList02_01 {

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "ExclusionRoute")]
    pub exclusion_route: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Metric")]
    pub metric: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PrefixSize")]
    pub prefix_size: Option<i32>,
}

impl MDM_VPNv2_RouteList02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            exclusion_route: None,
            instance_id: None,
            metric: None,
            parent_id: None,
            prefix_size: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of ExclusionRoute
    pub fn set_exclusion_route(&mut self, value: bool) {
        self.exclusion_route = Some(value);
    }

    /// Gets the value of ExclusionRoute
    pub fn get_exclusion_route(&self) -> Option<&bool> {
        self.exclusion_route.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Metric
    pub fn set_metric(&mut self, value: i32) {
        self.metric = Some(value);
    }

    /// Gets the value of Metric
    pub fn get_metric(&self) -> Option<&i32> {
        self.metric.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PrefixSize
    pub fn set_prefix_size(&mut self, value: i32) {
        self.prefix_size = Some(value);
    }

    /// Gets the value of PrefixSize
    pub fn get_prefix_size(&self) -> Option<&i32> {
        self.prefix_size.as_ref()
    }
}

