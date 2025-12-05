// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ScaleoutVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ScaleoutVolume {

/// 
    #[serde(rename = "AllocationSize")]
    pub allocation_size: Option<u64>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "numZones")]
    pub num_zones: Option<u32>,

/// 
    #[serde(rename = "SizeInBytes")]
    pub size_in_bytes: Option<u64>,

/// 
    #[serde(rename = "VolumeGuid")]
    pub volume_guid: Option<String>,

/// 
    #[serde(rename = "zoneArray")]
    pub zone_array: Vec<MSCluster_ScaleoutZone>,
}

impl MSCluster_ScaleoutVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allocation_size: None,
            friendly_name: None,
            num_zones: None,
            size_in_bytes: None,
            volume_guid: None,
            zone_array: Vec::new(),
        }
    }


    /// Sets the value of AllocationSize
    pub fn set_allocation_size(&mut self, value: u64) {
        self.allocation_size = Some(value);
    }

    /// Gets the value of AllocationSize
    pub fn get_allocation_size(&self) -> Option<&u64> {
        self.allocation_size.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of numZones
    pub fn set_num_zones(&mut self, value: u32) {
        self.num_zones = Some(value);
    }

    /// Gets the value of numZones
    pub fn get_num_zones(&self) -> Option<&u32> {
        self.num_zones.as_ref()
    }

    /// Sets the value of SizeInBytes
    pub fn set_size_in_bytes(&mut self, value: u64) {
        self.size_in_bytes = Some(value);
    }

    /// Gets the value of SizeInBytes
    pub fn get_size_in_bytes(&self) -> Option<&u64> {
        self.size_in_bytes.as_ref()
    }

    /// Sets the value of VolumeGuid
    pub fn set_volume_guid(&mut self, value: String) {
        self.volume_guid = Some(value);
    }

    /// Gets the value of VolumeGuid
    pub fn get_volume_guid(&self) -> Option<&String> {
        self.volume_guid.as_ref()
    }

    /// Sets the value of zoneArray
    pub fn set_zone_array(&mut self, value: Vec<MSCluster_ScaleoutZone>) {
        self.zone_array = value;
    }

    /// Gets the value of zoneArray
    pub fn get_zone_array(&self) -> &Vec<MSCluster_ScaleoutZone> {
        &self.zone_array
    }

/// 

    /// * `max_metadata_volume_size` -  (u64)
    /// * `volume_name` -  (String)
    /// * `zone_group_id` -  (String)

    /// * `created_scaleout_volume` -  (MSCluster_ScaleoutVolume)
    /// * `return_value` -  (u32)
    pub fn new_scaleout_volume(&self, volume_name: &String, max_metadata_volume_size: u64, zone_group_id: &String, created_scaleout_volume: &mut MSCluster_ScaleoutVolume) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeName".to_string(), value: volume_name.into() });
        args.push(MethodParameter { name: "MaxMetadataVolumeSize".to_string(), value: max_metadata_volume_size.into() });
        args.push(MethodParameter { name: "ZoneGroupId".to_string(), value: zone_group_id.into() });

        let result = self.invoke_method("NewScaleoutVolume", &args)?;
        let created_scaleout_volume = result.get_value("CreatedScaleoutVolume")?;
        Ok(result.return_value)

    }


/// 

    /// * `size_in_bytes` -  (u64)
    /// * `zone_group_id` -  (String)
    /// * `zone_id` -  (String)
    /// * `zone_resource` -  (String)
    /// * `zone_target_path` -  (String)
    /// * `zone_volume` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn add_data_zone_to_sv(&self, zone_id: &String, zone_group_id: &String, size_in_bytes: u64, zone_target_path: &String, zone_volume: &String, zone_resource: &String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ZoneId".to_string(), value: zone_id.into() });
        args.push(MethodParameter { name: "ZoneGroupId".to_string(), value: zone_group_id.into() });
        args.push(MethodParameter { name: "sizeInBytes".to_string(), value: size_in_bytes.into() });
        args.push(MethodParameter { name: "ZoneTargetPath".to_string(), value: zone_target_path.into() });
        args.push(MethodParameter { name: "ZoneVolume".to_string(), value: zone_volume.into() });
        args.push(MethodParameter { name: "ZoneResource".to_string(), value: zone_resource.into() });

        let result = self.invoke_method("AddDataZoneToSV", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `cluster_name` -  (String)
    /// * `force` -  (bool)
    /// * `resource_name` -  (String)
    /// * `volume_name` -  (String)
    /// * `zone_id` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn remove_svdata_zone(&self, volume_name: &String, zone_id: &String, cluster_name: &String, resource_name: &String, force: bool, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeName".to_string(), value: volume_name.into() });
        args.push(MethodParameter { name: "ZoneId".to_string(), value: zone_id.into() });
        args.push(MethodParameter { name: "ClusterName".to_string(), value: cluster_name.into() });
        args.push(MethodParameter { name: "ResourceName".to_string(), value: resource_name.into() });
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });

        let result = self.invoke_method("RemoveSVDataZone", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn delete_sv(&self, volume_name: &String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeName".to_string(), value: volume_name.into() });

        let result = self.invoke_method("DeleteSV", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_name` -  (String)

    /// * `retrieved_scaleout_volume` -  (MSCluster_ScaleoutVolume)
    /// * `return_value` -  (u32)
    pub fn get_svinformation(&self, volume_name: &String, retrieved_scaleout_volume: &mut MSCluster_ScaleoutVolume) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeName".to_string(), value: volume_name.into() });

        let result = self.invoke_method("GetSVInformation", &args)?;
        let retrieved_scaleout_volume = result.get_value("RetrievedScaleoutVolume")?;
        Ok(result.return_value)

    }


/// 

    /// * `guids` -  (MSCluster_ScaleoutVolume[])
    /// * `return_value` -  (u32)
    pub fn get_all_sv(&self, guids: &mut Vec<MSCluster_ScaleoutVolume>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAllSV", &[])?;
        let guids = result.get_value("Guids")?;
        Ok(result.return_value)

    }


/// 

    /// * `zone_id` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn suspend_svdata_zone(&self, zone_id: &String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "zoneId".to_string(), value: zone_id.into() });

        let result = self.invoke_method("SuspendSVDataZone", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `zone_id` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn resume_svdata_zone(&self, zone_id: &String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "zoneId".to_string(), value: zone_id.into() });

        let result = self.invoke_method("ResumeSVDataZone", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `zone_id` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn retire_svdata_zone(&self, zone_id: &String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "zoneId".to_string(), value: zone_id.into() });

        let result = self.invoke_method("RetireSVDataZone", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `zone_group_id` -  (String)
    /// * `zone_id` -  (String)

    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn update_group_id_for_svdata_zone(&self, zone_id: &String, zone_group_id: &String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "zoneId".to_string(), value: zone_id.into() });
        args.push(MethodParameter { name: "zoneGroupId".to_string(), value: zone_group_id.into() });

        let result = self.invoke_method("UpdateGroupIdForSVDataZone", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `zone_id` -  (String)

    /// * `retrieved_scaleout_zone` -  (MSCluster_ScaleoutZone)
    /// * `return_value` -  (u32)
    pub fn get_zone_information(&self, zone_id: &String, retrieved_scaleout_zone: &mut MSCluster_ScaleoutZone) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "zoneId".to_string(), value: zone_id.into() });

        let result = self.invoke_method("GetZoneInformation", &args)?;
        let retrieved_scaleout_zone = result.get_value("RetrievedScaleoutZone")?;
        Ok(result.return_value)

    }

}

