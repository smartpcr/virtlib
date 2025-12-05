// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileStorageTier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileStorageTier {

/// 
    #[serde(rename = "DesiredStorageTierClass")]
    pub desired_storage_tier_class: Option<u16>,

/// 
    #[serde(rename = "DesiredStorageTierName")]
    pub desired_storage_tier_name: Option<String>,

/// 
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// 
    #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,

/// 
    #[serde(rename = "FileSizeOnCapacityTierClass")]
    pub file_size_on_capacity_tier_class: Option<u64>,

/// 
    #[serde(rename = "FileSizeOnDesiredStorageTier")]
    pub file_size_on_desired_storage_tier: Option<u64>,

/// 
    #[serde(rename = "FileSizeOnDesiredStorageTierClass")]
    pub file_size_on_desired_storage_tier_class: Option<u64>,

/// 
    #[serde(rename = "FileSizeOnPerformanceTierClass")]
    pub file_size_on_performance_tier_class: Option<u64>,

/// 
    #[serde(rename = "PlacementStatus")]
    pub placement_status: Option<u16>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u16>,
}

impl MSFT_FileStorageTier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            desired_storage_tier_class: None,
            desired_storage_tier_name: None,
            file_path: None,
            file_size: None,
            file_size_on_capacity_tier_class: None,
            file_size_on_desired_storage_tier: None,
            file_size_on_desired_storage_tier_class: None,
            file_size_on_performance_tier_class: None,
            placement_status: None,
            state: None,
        }
    }


    /// Sets the value of DesiredStorageTierClass
    pub fn set_desired_storage_tier_class(&mut self, value: u16) {
        self.desired_storage_tier_class = Some(value);
    }

    /// Gets the value of DesiredStorageTierClass
    pub fn get_desired_storage_tier_class(&self) -> Option<&u16> {
        self.desired_storage_tier_class.as_ref()
    }

    /// Sets the value of DesiredStorageTierName
    pub fn set_desired_storage_tier_name(&mut self, value: String) {
        self.desired_storage_tier_name = Some(value);
    }

    /// Gets the value of DesiredStorageTierName
    pub fn get_desired_storage_tier_name(&self) -> Option<&String> {
        self.desired_storage_tier_name.as_ref()
    }

    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of FileSize
    pub fn set_file_size(&mut self, value: u64) {
        self.file_size = Some(value);
    }

    /// Gets the value of FileSize
    pub fn get_file_size(&self) -> Option<&u64> {
        self.file_size.as_ref()
    }

    /// Sets the value of FileSizeOnCapacityTierClass
    pub fn set_file_size_on_capacity_tier_class(&mut self, value: u64) {
        self.file_size_on_capacity_tier_class = Some(value);
    }

    /// Gets the value of FileSizeOnCapacityTierClass
    pub fn get_file_size_on_capacity_tier_class(&self) -> Option<&u64> {
        self.file_size_on_capacity_tier_class.as_ref()
    }

    /// Sets the value of FileSizeOnDesiredStorageTier
    pub fn set_file_size_on_desired_storage_tier(&mut self, value: u64) {
        self.file_size_on_desired_storage_tier = Some(value);
    }

    /// Gets the value of FileSizeOnDesiredStorageTier
    pub fn get_file_size_on_desired_storage_tier(&self) -> Option<&u64> {
        self.file_size_on_desired_storage_tier.as_ref()
    }

    /// Sets the value of FileSizeOnDesiredStorageTierClass
    pub fn set_file_size_on_desired_storage_tier_class(&mut self, value: u64) {
        self.file_size_on_desired_storage_tier_class = Some(value);
    }

    /// Gets the value of FileSizeOnDesiredStorageTierClass
    pub fn get_file_size_on_desired_storage_tier_class(&self) -> Option<&u64> {
        self.file_size_on_desired_storage_tier_class.as_ref()
    }

    /// Sets the value of FileSizeOnPerformanceTierClass
    pub fn set_file_size_on_performance_tier_class(&mut self, value: u64) {
        self.file_size_on_performance_tier_class = Some(value);
    }

    /// Gets the value of FileSizeOnPerformanceTierClass
    pub fn get_file_size_on_performance_tier_class(&self) -> Option<&u64> {
        self.file_size_on_performance_tier_class.as_ref()
    }

    /// Sets the value of PlacementStatus
    pub fn set_placement_status(&mut self, value: u16) {
        self.placement_status = Some(value);
    }

    /// Gets the value of PlacementStatus
    pub fn get_placement_status(&self) -> Option<&u16> {
        self.placement_status.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u16) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u16> {
        self.state.as_ref()
    }

/// 

    /// * `allocated_storage_tier_class` -  (u16)
    /// * `file_path` -  (String)
    /// * `pinned_state` -  (u16)
    /// * `pinned_storage_tier_class` -  (u16)
    /// * `volume` -  (MSFT_Volume)
    /// * `volume_drive_letter` -  (char)
    /// * `volume_path` -  (String)

    /// * `file_storage_tier` -  (MSFT_FileStorageTier[])
    /// * `return_value` -  (u32)
    pub fn get(&self, file_path: &String, volume_drive_letter: char, volume_path: &String, volume: MSFT_Volume, pinned_state: u16, pinned_storage_tier_class: u16, allocated_storage_tier_class: u16, file_storage_tier: &mut Vec<MSFT_FileStorageTier>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });
        args.push(MethodParameter { name: "VolumeDriveLetter".to_string(), value: volume_drive_letter.into() });
        args.push(MethodParameter { name: "VolumePath".to_string(), value: volume_path.into() });
        args.push(MethodParameter { name: "Volume".to_string(), value: volume.into() });
        args.push(MethodParameter { name: "PinnedState".to_string(), value: pinned_state.into() });
        args.push(MethodParameter { name: "PinnedStorageTierClass".to_string(), value: pinned_storage_tier_class.into() });
        args.push(MethodParameter { name: "AllocatedStorageTierClass".to_string(), value: allocated_storage_tier_class.into() });

        let result = self.invoke_method("Get", &args)?;
        let file_storage_tier = result.get_value("FileStorageTier")?;
        Ok(result.return_value)

    }


/// 

    /// * `desired_storage_tier` -  (MSFT_StorageTier)
    /// * `desired_storage_tier_class` -  (u16)
    /// * `desired_storage_tier_friendly_name` -  (String)
    /// * `desired_storage_tier_unique_id` -  (String)
    /// * `file_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn set(&self, file_path: &String, desired_storage_tier_friendly_name: &String, desired_storage_tier_unique_id: &String, desired_storage_tier_class: u16, desired_storage_tier: MSFT_StorageTier) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });
        args.push(MethodParameter { name: "DesiredStorageTierFriendlyName".to_string(), value: desired_storage_tier_friendly_name.into() });
        args.push(MethodParameter { name: "DesiredStorageTierUniqueId".to_string(), value: desired_storage_tier_unique_id.into() });
        args.push(MethodParameter { name: "DesiredStorageTierClass".to_string(), value: desired_storage_tier_class.into() });
        args.push(MethodParameter { name: "DesiredStorageTier".to_string(), value: desired_storage_tier.into() });
        self.invoke_method("Set", &args)

    }


/// 

    /// * `file_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn clear(&self, file_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });
        self.invoke_method("Clear", &args)

    }

}

