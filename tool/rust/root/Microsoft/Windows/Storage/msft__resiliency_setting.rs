// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ResiliencySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ResiliencySetting {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "InterleaveDefault")]
    pub interleave_default: Option<u64>,

/// 
    #[serde(rename = "InterleaveMax")]
    pub interleave_max: Option<u64>,

/// 
    #[serde(rename = "InterleaveMin")]
    pub interleave_min: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NumberOfColumnsDefault")]
    pub number_of_columns_default: Option<u16>,

/// 
    #[serde(rename = "NumberOfColumnsMax")]
    pub number_of_columns_max: Option<u16>,

/// 
    #[serde(rename = "NumberOfColumnsMin")]
    pub number_of_columns_min: Option<u16>,

/// 
    #[serde(rename = "NumberOfDataCopiesDefault")]
    pub number_of_data_copies_default: Option<u16>,

/// 
    #[serde(rename = "NumberOfDataCopiesMax")]
    pub number_of_data_copies_max: Option<u16>,

/// 
    #[serde(rename = "NumberOfDataCopiesMin")]
    pub number_of_data_copies_min: Option<u16>,

/// 
    #[serde(rename = "NumberOfGroupsDefault")]
    pub number_of_groups_default: Option<u16>,

/// 
    #[serde(rename = "NumberOfGroupsMax")]
    pub number_of_groups_max: Option<u16>,

/// 
    #[serde(rename = "NumberOfGroupsMin")]
    pub number_of_groups_min: Option<u16>,

/// 
    #[serde(rename = "ParityLayout")]
    pub parity_layout: Option<u16>,

/// 
    #[serde(rename = "PhysicalDiskRedundancyDefault")]
    pub physical_disk_redundancy_default: Option<u16>,

/// 
    #[serde(rename = "PhysicalDiskRedundancyMax")]
    pub physical_disk_redundancy_max: Option<u16>,

/// 
    #[serde(rename = "PhysicalDiskRedundancyMin")]
    pub physical_disk_redundancy_min: Option<u16>,

/// 
    #[serde(rename = "RequestNoSinglePointOfFailure")]
    pub request_no_single_point_of_failure: Option<bool>,
}

impl MSFT_ResiliencySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            description: None,
            interleave_default: None,
            interleave_max: None,
            interleave_min: None,
            name: None,
            number_of_columns_default: None,
            number_of_columns_max: None,
            number_of_columns_min: None,
            number_of_data_copies_default: None,
            number_of_data_copies_max: None,
            number_of_data_copies_min: None,
            number_of_groups_default: None,
            number_of_groups_max: None,
            number_of_groups_min: None,
            parity_layout: None,
            physical_disk_redundancy_default: None,
            physical_disk_redundancy_max: None,
            physical_disk_redundancy_min: None,
            request_no_single_point_of_failure: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of InterleaveDefault
    pub fn set_interleave_default(&mut self, value: u64) {
        self.interleave_default = Some(value);
    }

    /// Gets the value of InterleaveDefault
    pub fn get_interleave_default(&self) -> Option<&u64> {
        self.interleave_default.as_ref()
    }

    /// Sets the value of InterleaveMax
    pub fn set_interleave_max(&mut self, value: u64) {
        self.interleave_max = Some(value);
    }

    /// Gets the value of InterleaveMax
    pub fn get_interleave_max(&self) -> Option<&u64> {
        self.interleave_max.as_ref()
    }

    /// Sets the value of InterleaveMin
    pub fn set_interleave_min(&mut self, value: u64) {
        self.interleave_min = Some(value);
    }

    /// Gets the value of InterleaveMin
    pub fn get_interleave_min(&self) -> Option<&u64> {
        self.interleave_min.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NumberOfColumnsDefault
    pub fn set_number_of_columns_default(&mut self, value: u16) {
        self.number_of_columns_default = Some(value);
    }

    /// Gets the value of NumberOfColumnsDefault
    pub fn get_number_of_columns_default(&self) -> Option<&u16> {
        self.number_of_columns_default.as_ref()
    }

    /// Sets the value of NumberOfColumnsMax
    pub fn set_number_of_columns_max(&mut self, value: u16) {
        self.number_of_columns_max = Some(value);
    }

    /// Gets the value of NumberOfColumnsMax
    pub fn get_number_of_columns_max(&self) -> Option<&u16> {
        self.number_of_columns_max.as_ref()
    }

    /// Sets the value of NumberOfColumnsMin
    pub fn set_number_of_columns_min(&mut self, value: u16) {
        self.number_of_columns_min = Some(value);
    }

    /// Gets the value of NumberOfColumnsMin
    pub fn get_number_of_columns_min(&self) -> Option<&u16> {
        self.number_of_columns_min.as_ref()
    }

    /// Sets the value of NumberOfDataCopiesDefault
    pub fn set_number_of_data_copies_default(&mut self, value: u16) {
        self.number_of_data_copies_default = Some(value);
    }

    /// Gets the value of NumberOfDataCopiesDefault
    pub fn get_number_of_data_copies_default(&self) -> Option<&u16> {
        self.number_of_data_copies_default.as_ref()
    }

    /// Sets the value of NumberOfDataCopiesMax
    pub fn set_number_of_data_copies_max(&mut self, value: u16) {
        self.number_of_data_copies_max = Some(value);
    }

    /// Gets the value of NumberOfDataCopiesMax
    pub fn get_number_of_data_copies_max(&self) -> Option<&u16> {
        self.number_of_data_copies_max.as_ref()
    }

    /// Sets the value of NumberOfDataCopiesMin
    pub fn set_number_of_data_copies_min(&mut self, value: u16) {
        self.number_of_data_copies_min = Some(value);
    }

    /// Gets the value of NumberOfDataCopiesMin
    pub fn get_number_of_data_copies_min(&self) -> Option<&u16> {
        self.number_of_data_copies_min.as_ref()
    }

    /// Sets the value of NumberOfGroupsDefault
    pub fn set_number_of_groups_default(&mut self, value: u16) {
        self.number_of_groups_default = Some(value);
    }

    /// Gets the value of NumberOfGroupsDefault
    pub fn get_number_of_groups_default(&self) -> Option<&u16> {
        self.number_of_groups_default.as_ref()
    }

    /// Sets the value of NumberOfGroupsMax
    pub fn set_number_of_groups_max(&mut self, value: u16) {
        self.number_of_groups_max = Some(value);
    }

    /// Gets the value of NumberOfGroupsMax
    pub fn get_number_of_groups_max(&self) -> Option<&u16> {
        self.number_of_groups_max.as_ref()
    }

    /// Sets the value of NumberOfGroupsMin
    pub fn set_number_of_groups_min(&mut self, value: u16) {
        self.number_of_groups_min = Some(value);
    }

    /// Gets the value of NumberOfGroupsMin
    pub fn get_number_of_groups_min(&self) -> Option<&u16> {
        self.number_of_groups_min.as_ref()
    }

    /// Sets the value of ParityLayout
    pub fn set_parity_layout(&mut self, value: u16) {
        self.parity_layout = Some(value);
    }

    /// Gets the value of ParityLayout
    pub fn get_parity_layout(&self) -> Option<&u16> {
        self.parity_layout.as_ref()
    }

    /// Sets the value of PhysicalDiskRedundancyDefault
    pub fn set_physical_disk_redundancy_default(&mut self, value: u16) {
        self.physical_disk_redundancy_default = Some(value);
    }

    /// Gets the value of PhysicalDiskRedundancyDefault
    pub fn get_physical_disk_redundancy_default(&self) -> Option<&u16> {
        self.physical_disk_redundancy_default.as_ref()
    }

    /// Sets the value of PhysicalDiskRedundancyMax
    pub fn set_physical_disk_redundancy_max(&mut self, value: u16) {
        self.physical_disk_redundancy_max = Some(value);
    }

    /// Gets the value of PhysicalDiskRedundancyMax
    pub fn get_physical_disk_redundancy_max(&self) -> Option<&u16> {
        self.physical_disk_redundancy_max.as_ref()
    }

    /// Sets the value of PhysicalDiskRedundancyMin
    pub fn set_physical_disk_redundancy_min(&mut self, value: u16) {
        self.physical_disk_redundancy_min = Some(value);
    }

    /// Gets the value of PhysicalDiskRedundancyMin
    pub fn get_physical_disk_redundancy_min(&self) -> Option<&u16> {
        self.physical_disk_redundancy_min.as_ref()
    }

    /// Sets the value of RequestNoSinglePointOfFailure
    pub fn set_request_no_single_point_of_failure(&mut self, value: bool) {
        self.request_no_single_point_of_failure = Some(value);
    }

    /// Gets the value of RequestNoSinglePointOfFailure
    pub fn get_request_no_single_point_of_failure(&self) -> Option<&bool> {
        self.request_no_single_point_of_failure.as_ref()
    }

/// 

    /// * `auto_number_of_columns` -  (bool)
    /// * `interleave_default` -  (u64)
    /// * `number_of_columns_default` -  (u16)
    /// * `number_of_data_copies_default` -  (u16)
    /// * `number_of_groups_default` -  (u16)
    /// * `physical_disk_redundancy_default` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_defaults(&self, number_of_data_copies_default: u16, physical_disk_redundancy_default: u16, number_of_columns_default: u16, auto_number_of_columns: bool, interleave_default: u64, number_of_groups_default: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NumberOfDataCopiesDefault".to_string(), value: number_of_data_copies_default.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancyDefault".to_string(), value: physical_disk_redundancy_default.into() });
        args.push(MethodParameter { name: "NumberOfColumnsDefault".to_string(), value: number_of_columns_default.into() });
        args.push(MethodParameter { name: "AutoNumberOfColumns".to_string(), value: auto_number_of_columns.into() });
        args.push(MethodParameter { name: "InterleaveDefault".to_string(), value: interleave_default.into() });
        args.push(MethodParameter { name: "NumberOfGroupsDefault".to_string(), value: number_of_groups_default.into() });

        let result = self.invoke_method("SetDefaults", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

