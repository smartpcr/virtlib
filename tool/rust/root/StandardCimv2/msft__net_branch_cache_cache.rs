// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheCache {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CacheFileDirectoryPath")]
    pub cache_file_directory_path: Option<String>,

/// 
    #[serde(rename = "CurrentSizeOnDiskAsNumberOfBytes")]
    pub current_size_on_disk_as_number_of_bytes: Option<u64>,

/// 
    #[serde(rename = "MaxCacheSizeAsNumberOfBytes")]
    pub max_cache_size_as_number_of_bytes: Option<u64>,

/// 
    #[serde(rename = "MaxCacheSizeAsPercentageOfDiskVolume")]
    pub max_cache_size_as_percentage_of_disk_volume: Option<u32>,
}

impl MSFT_NetBranchCacheCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            cache_file_directory_path: None,
            current_size_on_disk_as_number_of_bytes: None,
            max_cache_size_as_number_of_bytes: None,
            max_cache_size_as_percentage_of_disk_volume: None,
        }
    }


    /// Sets the value of CacheFileDirectoryPath
    pub fn set_cache_file_directory_path(&mut self, value: String) {
        self.cache_file_directory_path = Some(value);
    }

    /// Gets the value of CacheFileDirectoryPath
    pub fn get_cache_file_directory_path(&self) -> Option<&String> {
        self.cache_file_directory_path.as_ref()
    }

    /// Sets the value of CurrentSizeOnDiskAsNumberOfBytes
    pub fn set_current_size_on_disk_as_number_of_bytes(&mut self, value: u64) {
        self.current_size_on_disk_as_number_of_bytes = Some(value);
    }

    /// Gets the value of CurrentSizeOnDiskAsNumberOfBytes
    pub fn get_current_size_on_disk_as_number_of_bytes(&self) -> Option<&u64> {
        self.current_size_on_disk_as_number_of_bytes.as_ref()
    }

    /// Sets the value of MaxCacheSizeAsNumberOfBytes
    pub fn set_max_cache_size_as_number_of_bytes(&mut self, value: u64) {
        self.max_cache_size_as_number_of_bytes = Some(value);
    }

    /// Gets the value of MaxCacheSizeAsNumberOfBytes
    pub fn get_max_cache_size_as_number_of_bytes(&self) -> Option<&u64> {
        self.max_cache_size_as_number_of_bytes.as_ref()
    }

    /// Sets the value of MaxCacheSizeAsPercentageOfDiskVolume
    pub fn set_max_cache_size_as_percentage_of_disk_volume(&mut self, value: u32) {
        self.max_cache_size_as_percentage_of_disk_volume = Some(value);
    }

    /// Gets the value of MaxCacheSizeAsPercentageOfDiskVolume
    pub fn get_max_cache_size_as_percentage_of_disk_volume(&self) -> Option<&u32> {
        self.max_cache_size_as_percentage_of_disk_volume.as_ref()
    }
}

