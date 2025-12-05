// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageTier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageTier {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// 
    #[serde(rename = "AllocationUnitSize")]
    pub allocation_unit_size: Option<u64>,

/// 
    #[serde(rename = "ColumnIsolation")]
    pub column_isolation: Option<u16>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FaultDomainAwareness")]
    pub fault_domain_awareness: Option<u16>,

/// 
    #[serde(rename = "FootprintOnPool")]
    pub footprint_on_pool: Option<u64>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "Interleave")]
    pub interleave: Option<u64>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u16>,

/// 
    #[serde(rename = "NumberOfColumns")]
    pub number_of_columns: Option<u16>,

/// 
    #[serde(rename = "NumberOfDataCopies")]
    pub number_of_data_copies: Option<u16>,

/// 
    #[serde(rename = "NumberOfGroups")]
    pub number_of_groups: Option<u16>,

/// 
    #[serde(rename = "ParityLayout")]
    pub parity_layout: Option<u16>,

/// 
    #[serde(rename = "PhysicalDiskRedundancy")]
    pub physical_disk_redundancy: Option<u16>,

/// 
    #[serde(rename = "ProvisioningType")]
    pub provisioning_type: Option<u16>,

/// 
    #[serde(rename = "ResiliencySettingName")]
    pub resiliency_setting_name: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "TierClass")]
    pub tier_class: Option<u16>,

/// 
    #[serde(rename = "Usage")]
    pub usage: Option<u16>,
}

impl MSFT_StorageTier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            allocated_size: None,
            allocation_unit_size: None,
            column_isolation: None,
            description: None,
            fault_domain_awareness: None,
            footprint_on_pool: None,
            friendly_name: None,
            interleave: None,
            media_type: None,
            number_of_columns: None,
            number_of_data_copies: None,
            number_of_groups: None,
            parity_layout: None,
            physical_disk_redundancy: None,
            provisioning_type: None,
            resiliency_setting_name: None,
            size: None,
            tier_class: None,
            usage: None,
        }
    }


    /// Sets the value of AllocatedSize
    pub fn set_allocated_size(&mut self, value: u64) {
        self.allocated_size = Some(value);
    }

    /// Gets the value of AllocatedSize
    pub fn get_allocated_size(&self) -> Option<&u64> {
        self.allocated_size.as_ref()
    }

    /// Sets the value of AllocationUnitSize
    pub fn set_allocation_unit_size(&mut self, value: u64) {
        self.allocation_unit_size = Some(value);
    }

    /// Gets the value of AllocationUnitSize
    pub fn get_allocation_unit_size(&self) -> Option<&u64> {
        self.allocation_unit_size.as_ref()
    }

    /// Sets the value of ColumnIsolation
    pub fn set_column_isolation(&mut self, value: u16) {
        self.column_isolation = Some(value);
    }

    /// Gets the value of ColumnIsolation
    pub fn get_column_isolation(&self) -> Option<&u16> {
        self.column_isolation.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of FaultDomainAwareness
    pub fn set_fault_domain_awareness(&mut self, value: u16) {
        self.fault_domain_awareness = Some(value);
    }

    /// Gets the value of FaultDomainAwareness
    pub fn get_fault_domain_awareness(&self) -> Option<&u16> {
        self.fault_domain_awareness.as_ref()
    }

    /// Sets the value of FootprintOnPool
    pub fn set_footprint_on_pool(&mut self, value: u64) {
        self.footprint_on_pool = Some(value);
    }

    /// Gets the value of FootprintOnPool
    pub fn get_footprint_on_pool(&self) -> Option<&u64> {
        self.footprint_on_pool.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of Interleave
    pub fn set_interleave(&mut self, value: u64) {
        self.interleave = Some(value);
    }

    /// Gets the value of Interleave
    pub fn get_interleave(&self) -> Option<&u64> {
        self.interleave.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u16) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u16> {
        self.media_type.as_ref()
    }

    /// Sets the value of NumberOfColumns
    pub fn set_number_of_columns(&mut self, value: u16) {
        self.number_of_columns = Some(value);
    }

    /// Gets the value of NumberOfColumns
    pub fn get_number_of_columns(&self) -> Option<&u16> {
        self.number_of_columns.as_ref()
    }

    /// Sets the value of NumberOfDataCopies
    pub fn set_number_of_data_copies(&mut self, value: u16) {
        self.number_of_data_copies = Some(value);
    }

    /// Gets the value of NumberOfDataCopies
    pub fn get_number_of_data_copies(&self) -> Option<&u16> {
        self.number_of_data_copies.as_ref()
    }

    /// Sets the value of NumberOfGroups
    pub fn set_number_of_groups(&mut self, value: u16) {
        self.number_of_groups = Some(value);
    }

    /// Gets the value of NumberOfGroups
    pub fn get_number_of_groups(&self) -> Option<&u16> {
        self.number_of_groups.as_ref()
    }

    /// Sets the value of ParityLayout
    pub fn set_parity_layout(&mut self, value: u16) {
        self.parity_layout = Some(value);
    }

    /// Gets the value of ParityLayout
    pub fn get_parity_layout(&self) -> Option<&u16> {
        self.parity_layout.as_ref()
    }

    /// Sets the value of PhysicalDiskRedundancy
    pub fn set_physical_disk_redundancy(&mut self, value: u16) {
        self.physical_disk_redundancy = Some(value);
    }

    /// Gets the value of PhysicalDiskRedundancy
    pub fn get_physical_disk_redundancy(&self) -> Option<&u16> {
        self.physical_disk_redundancy.as_ref()
    }

    /// Sets the value of ProvisioningType
    pub fn set_provisioning_type(&mut self, value: u16) {
        self.provisioning_type = Some(value);
    }

    /// Gets the value of ProvisioningType
    pub fn get_provisioning_type(&self) -> Option<&u16> {
        self.provisioning_type.as_ref()
    }

    /// Sets the value of ResiliencySettingName
    pub fn set_resiliency_setting_name(&mut self, value: String) {
        self.resiliency_setting_name = Some(value);
    }

    /// Gets the value of ResiliencySettingName
    pub fn get_resiliency_setting_name(&self) -> Option<&String> {
        self.resiliency_setting_name.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of TierClass
    pub fn set_tier_class(&mut self, value: u16) {
        self.tier_class = Some(value);
    }

    /// Gets the value of TierClass
    pub fn get_tier_class(&self) -> Option<&u16> {
        self.tier_class.as_ref()
    }

    /// Sets the value of Usage
    pub fn set_usage(&mut self, value: u16) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&u16> {
        self.usage.as_ref()
    }

/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `physical_extents` -  (MSFT_PhysicalExtent[])
    /// * `return_value` -  (u32)
    pub fn get_physical_extent(&self, physical_extents: &mut Vec<MSFT_PhysicalExtent>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPhysicalExtent", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let physical_extents = result.get_value("PhysicalExtents")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("DeleteObject", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `size` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `size` -  (u64)
    pub fn resize(&self, size: &mut u64, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Resize", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let size = result.get_value("Size")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_friendly_name(&self, friendly_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });

        let result = self.invoke_method("SetFriendlyName", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `allocation_unit_size` -  (u64)
    /// * `column_isolation` -  (u16)
    /// * `fault_domain_awareness` -  (u16)
    /// * `interleave` -  (u64)
    /// * `media_type` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `resiliency_setting_name` -  (String)
    /// * `usage` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, resiliency_setting_name: &String, usage: u16, physical_disk_redundancy: u16, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, interleave: u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_description(&self, description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("SetDescription", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `resiliency_setting_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_sizes` -  (u64[])
    /// * `tier_size_divisor` -  (u64)
    /// * `tier_size_max` -  (u64)
    /// * `tier_size_min` -  (u64)
    pub fn get_supported_size(&self, resiliency_setting_name: &String, supported_sizes: &mut Vec<u64>, tier_size_min: &mut u64, tier_size_max: &mut u64, tier_size_divisor: &mut u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });

        let result = self.invoke_method("GetSupportedSize", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_sizes = result.get_value("SupportedSizes")?;
        let tier_size_divisor = result.get_value("TierSizeDivisor")?;
        let tier_size_max = result.get_value("TierSizeMax")?;
        let tier_size_min = result.get_value("TierSizeMin")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `storage_fault_domains` -  (MSFT_StorageFaultDomain[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_storage_fault_domain(&self, storage_fault_domains: &Vec<MSFT_StorageFaultDomain>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageFaultDomains".to_string(), value: storage_fault_domains.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("AddStorageFaultDomain", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `storage_fault_domains` -  (MSFT_StorageFaultDomain[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_storage_fault_domain(&self, storage_fault_domains: &Vec<MSFT_StorageFaultDomain>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageFaultDomains".to_string(), value: storage_fault_domains.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("RemoveStorageFaultDomain", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

