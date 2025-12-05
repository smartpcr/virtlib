// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Header_PartitionInfoExtensionV2_TypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header_PartitionInfoExtensionV2_TypeGroup {
    #[serde(flatten)]
    pub base: EventTraceEvent,

/// 
    #[serde(rename = "EventVersion")]
    pub event_version: Option<u16>,

/// 
    #[serde(rename = "ParentId")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PartitionId")]
    pub partition_id: Option<String>,

/// 
    #[serde(rename = "PartitionType")]
    pub partition_type: Option<u32>,

/// 
    #[serde(rename = "QpcOffsetFromRoot")]
    pub qpc_offset_from_root: Option<i64>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,
}

impl Header_PartitionInfoExtensionV2_TypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent::new(),
            event_version: None,
            parent_id: None,
            partition_id: None,
            partition_type: None,
            qpc_offset_from_root: None,
            reserved: None,
        }
    }


    /// Sets the value of EventVersion
    pub fn set_event_version(&mut self, value: u16) {
        self.event_version = Some(value);
    }

    /// Gets the value of EventVersion
    pub fn get_event_version(&self) -> Option<&u16> {
        self.event_version.as_ref()
    }

    /// Sets the value of ParentId
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentId
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PartitionId
    pub fn set_partition_id(&mut self, value: String) {
        self.partition_id = Some(value);
    }

    /// Gets the value of PartitionId
    pub fn get_partition_id(&self) -> Option<&String> {
        self.partition_id.as_ref()
    }

    /// Sets the value of PartitionType
    pub fn set_partition_type(&mut self, value: u32) {
        self.partition_type = Some(value);
    }

    /// Gets the value of PartitionType
    pub fn get_partition_type(&self) -> Option<&u32> {
        self.partition_type.as_ref()
    }

    /// Sets the value of QpcOffsetFromRoot
    pub fn set_qpc_offset_from_root(&mut self, value: i64) {
        self.qpc_offset_from_root = Some(value);
    }

    /// Gets the value of QpcOffsetFromRoot
    pub fn get_qpc_offset_from_root(&self) -> Option<&i64> {
        self.qpc_offset_from_root.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
    }
}

