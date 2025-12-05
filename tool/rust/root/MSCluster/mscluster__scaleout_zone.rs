// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ScaleoutZone struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ScaleoutZone {

/// 
    #[serde(rename = "ClusterSize")]
    pub cluster_size: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u64>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "StartOffset")]
    pub start_offset: Option<u64>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,

/// 
    #[serde(rename = "ZoneGroupId")]
    pub zone_group_id: Option<String>,

/// 
    #[serde(rename = "ZoneId")]
    pub zone_id: Option<String>,

/// 
    #[serde(rename = "ZoneNumber")]
    pub zone_number: Option<u32>,
}

impl MSCluster_ScaleoutZone {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_size: None,
            flags: None,
            size: None,
            start_offset: None,
            type: None,
            zone_group_id: None,
            zone_id: None,
            zone_number: None,
        }
    }


    /// Sets the value of ClusterSize
    pub fn set_cluster_size(&mut self, value: u32) {
        self.cluster_size = Some(value);
    }

    /// Gets the value of ClusterSize
    pub fn get_cluster_size(&self) -> Option<&u32> {
        self.cluster_size.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u64) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u64> {
        self.flags.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of StartOffset
    pub fn set_start_offset(&mut self, value: u64) {
        self.start_offset = Some(value);
    }

    /// Gets the value of StartOffset
    pub fn get_start_offset(&self) -> Option<&u64> {
        self.start_offset.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

    /// Sets the value of ZoneGroupId
    pub fn set_zone_group_id(&mut self, value: String) {
        self.zone_group_id = Some(value);
    }

    /// Gets the value of ZoneGroupId
    pub fn get_zone_group_id(&self) -> Option<&String> {
        self.zone_group_id.as_ref()
    }

    /// Sets the value of ZoneId
    pub fn set_zone_id(&mut self, value: String) {
        self.zone_id = Some(value);
    }

    /// Gets the value of ZoneId
    pub fn get_zone_id(&self) -> Option<&String> {
        self.zone_id.as_ref()
    }

    /// Sets the value of ZoneNumber
    pub fn set_zone_number(&mut self, value: u32) {
        self.zone_number = Some(value);
    }

    /// Gets the value of ZoneNumber
    pub fn get_zone_number(&self) -> Option<&u32> {
        self.zone_number.as_ref()
    }
}

