// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceQualifiedPartitionData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceQualifiedPartitionData {
    #[serde(flatten)]
    pub base: BcdDeviceData,

/// This is the disk signature.
    #[serde(rename = "DiskSignature")]
    pub disk_signature: Option<String>,

/// This is the partition identifier.
    #[serde(rename = "PartitionIdentifier")]
    pub partition_identifier: Option<String>,

/// This is the type of partition, MBR or GPT..
    #[serde(rename = "PartitionStyle")]
    pub partition_style: Option<BcdDeviceQualifiedPartitionData_PartitionStyle>,
}

impl BcdDeviceQualifiedPartitionData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdDeviceData::new(),
            disk_signature: None,
            partition_identifier: None,
            partition_style: None,
        }
    }


    /// Sets the value of DiskSignature
    pub fn set_disk_signature(&mut self, value: String) {
        self.disk_signature = Some(value);
    }

    /// Gets the value of DiskSignature
    pub fn get_disk_signature(&self) -> Option<&String> {
        self.disk_signature.as_ref()
    }

    /// Sets the value of PartitionIdentifier
    pub fn set_partition_identifier(&mut self, value: String) {
        self.partition_identifier = Some(value);
    }

    /// Gets the value of PartitionIdentifier
    pub fn get_partition_identifier(&self) -> Option<&String> {
        self.partition_identifier.as_ref()
    }

    /// Sets the value of PartitionStyle
    pub fn set_partition_style(&mut self, value: BcdDeviceQualifiedPartitionData_PartitionStyle) {
        self.partition_style = Some(value);
    }

    /// Gets the value of PartitionStyle
    pub fn get_partition_style(&self) -> Option<&BcdDeviceQualifiedPartitionData_PartitionStyle> {
        self.partition_style.as_ref()
    }
}

