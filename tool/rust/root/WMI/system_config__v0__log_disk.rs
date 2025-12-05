// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V0_LogDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V0_LogDisk {
    #[serde(flatten)]
    pub base: SystemConfig_V0,

/// 
    #[serde(rename = "BytesPerSector")]
    pub bytes_per_sector: Option<u32>,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 
    #[serde(rename = "DriveLetterString")]
    pub drive_letter_string: Vec<char>,

/// 
    #[serde(rename = "DriveType")]
    pub drive_type: Option<u32>,

/// 
    #[serde(rename = "FileSystem")]
    pub file_system: Vec<char>,

/// 
    #[serde(rename = "NumberOfFreeClusters")]
    pub number_of_free_clusters: Option<i64>,

/// 
    #[serde(rename = "Pad1")]
    pub pad1: Option<u32>,

/// 
    #[serde(rename = "Pad2")]
    pub pad2: Option<u32>,

/// 
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<u32>,

/// 
    #[serde(rename = "PartitionSize")]
    pub partition_size: Option<u64>,

/// 
    #[serde(rename = "SectorsPerCluster")]
    pub sectors_per_cluster: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "StartOffset")]
    pub start_offset: Option<u64>,

/// 
    #[serde(rename = "TotalNumberOfClusters")]
    pub total_number_of_clusters: Option<i64>,

/// 
    #[serde(rename = "VolumeExt")]
    pub volume_ext: Option<u32>,
}

impl SystemConfig_V0_LogDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V0::new(),
            bytes_per_sector: None,
            disk_number: None,
            drive_letter_string: Vec::new(),
            drive_type: None,
            file_system: Vec::new(),
            number_of_free_clusters: None,
            pad1: None,
            pad2: None,
            partition_number: None,
            partition_size: None,
            sectors_per_cluster: None,
            size: None,
            start_offset: None,
            total_number_of_clusters: None,
            volume_ext: None,
        }
    }


    /// Sets the value of BytesPerSector
    pub fn set_bytes_per_sector(&mut self, value: u32) {
        self.bytes_per_sector = Some(value);
    }

    /// Gets the value of BytesPerSector
    pub fn get_bytes_per_sector(&self) -> Option<&u32> {
        self.bytes_per_sector.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of DriveLetterString
    pub fn set_drive_letter_string(&mut self, value: Vec<char>) {
        self.drive_letter_string = value;
    }

    /// Gets the value of DriveLetterString
    pub fn get_drive_letter_string(&self) -> &Vec<char> {
        &self.drive_letter_string
    }

    /// Sets the value of DriveType
    pub fn set_drive_type(&mut self, value: u32) {
        self.drive_type = Some(value);
    }

    /// Gets the value of DriveType
    pub fn get_drive_type(&self) -> Option<&u32> {
        self.drive_type.as_ref()
    }

    /// Sets the value of FileSystem
    pub fn set_file_system(&mut self, value: Vec<char>) {
        self.file_system = value;
    }

    /// Gets the value of FileSystem
    pub fn get_file_system(&self) -> &Vec<char> {
        &self.file_system
    }

    /// Sets the value of NumberOfFreeClusters
    pub fn set_number_of_free_clusters(&mut self, value: i64) {
        self.number_of_free_clusters = Some(value);
    }

    /// Gets the value of NumberOfFreeClusters
    pub fn get_number_of_free_clusters(&self) -> Option<&i64> {
        self.number_of_free_clusters.as_ref()
    }

    /// Sets the value of Pad1
    pub fn set_pad1(&mut self, value: u32) {
        self.pad1 = Some(value);
    }

    /// Gets the value of Pad1
    pub fn get_pad1(&self) -> Option<&u32> {
        self.pad1.as_ref()
    }

    /// Sets the value of Pad2
    pub fn set_pad2(&mut self, value: u32) {
        self.pad2 = Some(value);
    }

    /// Gets the value of Pad2
    pub fn get_pad2(&self) -> Option<&u32> {
        self.pad2.as_ref()
    }

    /// Sets the value of PartitionNumber
    pub fn set_partition_number(&mut self, value: u32) {
        self.partition_number = Some(value);
    }

    /// Gets the value of PartitionNumber
    pub fn get_partition_number(&self) -> Option<&u32> {
        self.partition_number.as_ref()
    }

    /// Sets the value of PartitionSize
    pub fn set_partition_size(&mut self, value: u64) {
        self.partition_size = Some(value);
    }

    /// Gets the value of PartitionSize
    pub fn get_partition_size(&self) -> Option<&u64> {
        self.partition_size.as_ref()
    }

    /// Sets the value of SectorsPerCluster
    pub fn set_sectors_per_cluster(&mut self, value: u32) {
        self.sectors_per_cluster = Some(value);
    }

    /// Gets the value of SectorsPerCluster
    pub fn get_sectors_per_cluster(&self) -> Option<&u32> {
        self.sectors_per_cluster.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
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

    /// Sets the value of TotalNumberOfClusters
    pub fn set_total_number_of_clusters(&mut self, value: i64) {
        self.total_number_of_clusters = Some(value);
    }

    /// Gets the value of TotalNumberOfClusters
    pub fn get_total_number_of_clusters(&self) -> Option<&i64> {
        self.total_number_of_clusters.as_ref()
    }

    /// Sets the value of VolumeExt
    pub fn set_volume_ext(&mut self, value: u32) {
        self.volume_ext = Some(value);
    }

    /// Gets the value of VolumeExt
    pub fn get_volume_ext(&self) -> Option<&u32> {
        self.volume_ext.as_ref()
    }
}

