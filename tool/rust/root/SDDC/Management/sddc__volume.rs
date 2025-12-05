// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Volume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Volume {

/// 
    #[serde(rename = "AdditionalStatusBitMap")]
    pub additional_status_bit_map: Option<u64>,

/// 
    #[serde(rename = "Alerts")]
    pub alerts: Vec<SDDC_Alert>,

/// 
    #[serde(rename = "AverageLatency")]
    pub average_latency: Option<f64>,

/// 
    #[serde(rename = "ClusterResourceId")]
    pub cluster_resource_id: Option<String>,

/// 
    #[serde(rename = "DedupSavings")]
    pub dedup_savings: Option<u64>,

/// 
    #[serde(rename = "DedupSavingsRate")]
    pub dedup_savings_rate: Option<u32>,

/// 
    #[serde(rename = "EncryptionPercentage")]
    pub encryption_percentage: Option<u16>,

/// 
    #[serde(rename = "EncryptionStatus")]
    pub encryption_status: Option<u16>,

/// 
    #[serde(rename = "FaultDomainAwareness")]
    pub fault_domain_awareness: Option<u16>,

/// 
    #[serde(rename = "FileSystem")]
    pub file_system: Option<String>,

/// 
    #[serde(rename = "Footprint")]
    pub footprint: Option<u64>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsDedupEnabled")]
    pub is_dedup_enabled: Option<bool>,

/// 
    #[serde(rename = "IsEncrypted")]
    pub is_encrypted: Option<bool>,

/// 
    #[serde(rename = "IsIntegrityEnabled")]
    pub is_integrity_enabled: Option<bool>,

/// 
    #[serde(rename = "IsTiered")]
    pub is_tiered: Option<bool>,

/// 
    #[serde(rename = "Jobs")]
    pub jobs: Vec<SDDC_Job>,

/// 
    #[serde(rename = "Media")]
    pub media: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "ProvisioningType")]
    pub provisioning_type: Option<u16>,

/// 
    #[serde(rename = "ReFSDedupCompressionSavingsSize")]
    pub re_fsdedup_compression_savings_size: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupLastRunStatus")]
    pub re_fsdedup_last_run_status: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupLastRunTime")]
    pub re_fsdedup_last_run_time: Option<String>,

/// 
    #[serde(rename = "ReFSDedupMode")]
    pub re_fsdedup_mode: Option<u32>,

/// 
    #[serde(rename = "ReFSDedupNextRunTime")]
    pub re_fsdedup_next_run_time: Option<String>,

/// 
    #[serde(rename = "ReFSDedupSavingsSize")]
    pub re_fsdedup_savings_size: Option<u64>,

/// 
    #[serde(rename = "ReplicatedDiskType")]
    pub replicated_disk_type: Option<u16>,

/// 
    #[serde(rename = "ReplicationGroupName")]
    pub replication_group_name: Option<String>,

/// 
    #[serde(rename = "ReplicationMode")]
    pub replication_mode: Option<u32>,

/// 
    #[serde(rename = "ReplicationStatus")]
    pub replication_status: Option<u32>,

/// 
    #[serde(rename = "Resiliency")]
    pub resiliency: Option<u16>,

/// 
    #[serde(rename = "Server")]
    pub server: Option<String>,

/// 
    #[serde(rename = "SiteName")]
    pub site_name: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "SizeRemaining")]
    pub size_remaining: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Vec<u16>,

/// 
    #[serde(rename = "StatusCategory")]
    pub status_category: Option<u16>,

/// 
    #[serde(rename = "StoragePoolName")]
    pub storage_pool_name: Option<String>,

/// 
    #[serde(rename = "TierFootprints")]
    pub tier_footprints: Vec<u64>,

/// 
    #[serde(rename = "TierMedias")]
    pub tier_medias: Vec<u16>,

/// 
    #[serde(rename = "TierResiliencies")]
    pub tier_resiliencies: Vec<u16>,

/// 
    #[serde(rename = "TierSizes")]
    pub tier_sizes: Vec<u64>,

/// 
    #[serde(rename = "TotalIops")]
    pub total_iops: Option<f64>,

/// 
    #[serde(rename = "TotalThroughput")]
    pub total_throughput: Option<f64>,
}

impl SDDC_Volume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            additional_status_bit_map: None,
            alerts: Vec::new(),
            average_latency: None,
            cluster_resource_id: None,
            dedup_savings: None,
            dedup_savings_rate: None,
            encryption_percentage: None,
            encryption_status: None,
            fault_domain_awareness: None,
            file_system: None,
            footprint: None,
            id: None,
            is_dedup_enabled: None,
            is_encrypted: None,
            is_integrity_enabled: None,
            is_tiered: None,
            jobs: Vec::new(),
            media: None,
            name: None,
            path: None,
            provisioning_type: None,
            re_fsdedup_compression_savings_size: None,
            re_fsdedup_last_run_status: None,
            re_fsdedup_last_run_time: None,
            re_fsdedup_mode: None,
            re_fsdedup_next_run_time: None,
            re_fsdedup_savings_size: None,
            replicated_disk_type: None,
            replication_group_name: None,
            replication_mode: None,
            replication_status: None,
            resiliency: None,
            server: None,
            site_name: None,
            size: None,
            size_remaining: None,
            status: Vec::new(),
            status_category: None,
            storage_pool_name: None,
            tier_footprints: Vec::new(),
            tier_medias: Vec::new(),
            tier_resiliencies: Vec::new(),
            tier_sizes: Vec::new(),
            total_iops: None,
            total_throughput: None,
        }
    }


    /// Sets the value of AdditionalStatusBitMap
    pub fn set_additional_status_bit_map(&mut self, value: u64) {
        self.additional_status_bit_map = Some(value);
    }

    /// Gets the value of AdditionalStatusBitMap
    pub fn get_additional_status_bit_map(&self) -> Option<&u64> {
        self.additional_status_bit_map.as_ref()
    }

    /// Sets the value of Alerts
    pub fn set_alerts(&mut self, value: Vec<SDDC_Alert>) {
        self.alerts = value;
    }

    /// Gets the value of Alerts
    pub fn get_alerts(&self) -> &Vec<SDDC_Alert> {
        &self.alerts
    }

    /// Sets the value of AverageLatency
    pub fn set_average_latency(&mut self, value: f64) {
        self.average_latency = Some(value);
    }

    /// Gets the value of AverageLatency
    pub fn get_average_latency(&self) -> Option<&f64> {
        self.average_latency.as_ref()
    }

    /// Sets the value of ClusterResourceId
    pub fn set_cluster_resource_id(&mut self, value: String) {
        self.cluster_resource_id = Some(value);
    }

    /// Gets the value of ClusterResourceId
    pub fn get_cluster_resource_id(&self) -> Option<&String> {
        self.cluster_resource_id.as_ref()
    }

    /// Sets the value of DedupSavings
    pub fn set_dedup_savings(&mut self, value: u64) {
        self.dedup_savings = Some(value);
    }

    /// Gets the value of DedupSavings
    pub fn get_dedup_savings(&self) -> Option<&u64> {
        self.dedup_savings.as_ref()
    }

    /// Sets the value of DedupSavingsRate
    pub fn set_dedup_savings_rate(&mut self, value: u32) {
        self.dedup_savings_rate = Some(value);
    }

    /// Gets the value of DedupSavingsRate
    pub fn get_dedup_savings_rate(&self) -> Option<&u32> {
        self.dedup_savings_rate.as_ref()
    }

    /// Sets the value of EncryptionPercentage
    pub fn set_encryption_percentage(&mut self, value: u16) {
        self.encryption_percentage = Some(value);
    }

    /// Gets the value of EncryptionPercentage
    pub fn get_encryption_percentage(&self) -> Option<&u16> {
        self.encryption_percentage.as_ref()
    }

    /// Sets the value of EncryptionStatus
    pub fn set_encryption_status(&mut self, value: u16) {
        self.encryption_status = Some(value);
    }

    /// Gets the value of EncryptionStatus
    pub fn get_encryption_status(&self) -> Option<&u16> {
        self.encryption_status.as_ref()
    }

    /// Sets the value of FaultDomainAwareness
    pub fn set_fault_domain_awareness(&mut self, value: u16) {
        self.fault_domain_awareness = Some(value);
    }

    /// Gets the value of FaultDomainAwareness
    pub fn get_fault_domain_awareness(&self) -> Option<&u16> {
        self.fault_domain_awareness.as_ref()
    }

    /// Sets the value of FileSystem
    pub fn set_file_system(&mut self, value: String) {
        self.file_system = Some(value);
    }

    /// Gets the value of FileSystem
    pub fn get_file_system(&self) -> Option<&String> {
        self.file_system.as_ref()
    }

    /// Sets the value of Footprint
    pub fn set_footprint(&mut self, value: u64) {
        self.footprint = Some(value);
    }

    /// Gets the value of Footprint
    pub fn get_footprint(&self) -> Option<&u64> {
        self.footprint.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsDedupEnabled
    pub fn set_is_dedup_enabled(&mut self, value: bool) {
        self.is_dedup_enabled = Some(value);
    }

    /// Gets the value of IsDedupEnabled
    pub fn get_is_dedup_enabled(&self) -> Option<&bool> {
        self.is_dedup_enabled.as_ref()
    }

    /// Sets the value of IsEncrypted
    pub fn set_is_encrypted(&mut self, value: bool) {
        self.is_encrypted = Some(value);
    }

    /// Gets the value of IsEncrypted
    pub fn get_is_encrypted(&self) -> Option<&bool> {
        self.is_encrypted.as_ref()
    }

    /// Sets the value of IsIntegrityEnabled
    pub fn set_is_integrity_enabled(&mut self, value: bool) {
        self.is_integrity_enabled = Some(value);
    }

    /// Gets the value of IsIntegrityEnabled
    pub fn get_is_integrity_enabled(&self) -> Option<&bool> {
        self.is_integrity_enabled.as_ref()
    }

    /// Sets the value of IsTiered
    pub fn set_is_tiered(&mut self, value: bool) {
        self.is_tiered = Some(value);
    }

    /// Gets the value of IsTiered
    pub fn get_is_tiered(&self) -> Option<&bool> {
        self.is_tiered.as_ref()
    }

    /// Sets the value of Jobs
    pub fn set_jobs(&mut self, value: Vec<SDDC_Job>) {
        self.jobs = value;
    }

    /// Gets the value of Jobs
    pub fn get_jobs(&self) -> &Vec<SDDC_Job> {
        &self.jobs
    }

    /// Sets the value of Media
    pub fn set_media(&mut self, value: u16) {
        self.media = Some(value);
    }

    /// Gets the value of Media
    pub fn get_media(&self) -> Option<&u16> {
        self.media.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of ProvisioningType
    pub fn set_provisioning_type(&mut self, value: u16) {
        self.provisioning_type = Some(value);
    }

    /// Gets the value of ProvisioningType
    pub fn get_provisioning_type(&self) -> Option<&u16> {
        self.provisioning_type.as_ref()
    }

    /// Sets the value of ReFSDedupCompressionSavingsSize
    pub fn set_re_fsdedup_compression_savings_size(&mut self, value: u64) {
        self.re_fsdedup_compression_savings_size = Some(value);
    }

    /// Gets the value of ReFSDedupCompressionSavingsSize
    pub fn get_re_fsdedup_compression_savings_size(&self) -> Option<&u64> {
        self.re_fsdedup_compression_savings_size.as_ref()
    }

    /// Sets the value of ReFSDedupLastRunStatus
    pub fn set_re_fsdedup_last_run_status(&mut self, value: u64) {
        self.re_fsdedup_last_run_status = Some(value);
    }

    /// Gets the value of ReFSDedupLastRunStatus
    pub fn get_re_fsdedup_last_run_status(&self) -> Option<&u64> {
        self.re_fsdedup_last_run_status.as_ref()
    }

    /// Sets the value of ReFSDedupLastRunTime
    pub fn set_re_fsdedup_last_run_time(&mut self, value: String) {
        self.re_fsdedup_last_run_time = Some(value);
    }

    /// Gets the value of ReFSDedupLastRunTime
    pub fn get_re_fsdedup_last_run_time(&self) -> Option<&String> {
        self.re_fsdedup_last_run_time.as_ref()
    }

    /// Sets the value of ReFSDedupMode
    pub fn set_re_fsdedup_mode(&mut self, value: u32) {
        self.re_fsdedup_mode = Some(value);
    }

    /// Gets the value of ReFSDedupMode
    pub fn get_re_fsdedup_mode(&self) -> Option<&u32> {
        self.re_fsdedup_mode.as_ref()
    }

    /// Sets the value of ReFSDedupNextRunTime
    pub fn set_re_fsdedup_next_run_time(&mut self, value: String) {
        self.re_fsdedup_next_run_time = Some(value);
    }

    /// Gets the value of ReFSDedupNextRunTime
    pub fn get_re_fsdedup_next_run_time(&self) -> Option<&String> {
        self.re_fsdedup_next_run_time.as_ref()
    }

    /// Sets the value of ReFSDedupSavingsSize
    pub fn set_re_fsdedup_savings_size(&mut self, value: u64) {
        self.re_fsdedup_savings_size = Some(value);
    }

    /// Gets the value of ReFSDedupSavingsSize
    pub fn get_re_fsdedup_savings_size(&self) -> Option<&u64> {
        self.re_fsdedup_savings_size.as_ref()
    }

    /// Sets the value of ReplicatedDiskType
    pub fn set_replicated_disk_type(&mut self, value: u16) {
        self.replicated_disk_type = Some(value);
    }

    /// Gets the value of ReplicatedDiskType
    pub fn get_replicated_disk_type(&self) -> Option<&u16> {
        self.replicated_disk_type.as_ref()
    }

    /// Sets the value of ReplicationGroupName
    pub fn set_replication_group_name(&mut self, value: String) {
        self.replication_group_name = Some(value);
    }

    /// Gets the value of ReplicationGroupName
    pub fn get_replication_group_name(&self) -> Option<&String> {
        self.replication_group_name.as_ref()
    }

    /// Sets the value of ReplicationMode
    pub fn set_replication_mode(&mut self, value: u32) {
        self.replication_mode = Some(value);
    }

    /// Gets the value of ReplicationMode
    pub fn get_replication_mode(&self) -> Option<&u32> {
        self.replication_mode.as_ref()
    }

    /// Sets the value of ReplicationStatus
    pub fn set_replication_status(&mut self, value: u32) {
        self.replication_status = Some(value);
    }

    /// Gets the value of ReplicationStatus
    pub fn get_replication_status(&self) -> Option<&u32> {
        self.replication_status.as_ref()
    }

    /// Sets the value of Resiliency
    pub fn set_resiliency(&mut self, value: u16) {
        self.resiliency = Some(value);
    }

    /// Gets the value of Resiliency
    pub fn get_resiliency(&self) -> Option<&u16> {
        self.resiliency.as_ref()
    }

    /// Sets the value of Server
    pub fn set_server(&mut self, value: String) {
        self.server = Some(value);
    }

    /// Gets the value of Server
    pub fn get_server(&self) -> Option<&String> {
        self.server.as_ref()
    }

    /// Sets the value of SiteName
    pub fn set_site_name(&mut self, value: String) {
        self.site_name = Some(value);
    }

    /// Gets the value of SiteName
    pub fn get_site_name(&self) -> Option<&String> {
        self.site_name.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of SizeRemaining
    pub fn set_size_remaining(&mut self, value: u64) {
        self.size_remaining = Some(value);
    }

    /// Gets the value of SizeRemaining
    pub fn get_size_remaining(&self) -> Option<&u64> {
        self.size_remaining.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: Vec<u16>) {
        self.status = value;
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> &Vec<u16> {
        &self.status
    }

    /// Sets the value of StatusCategory
    pub fn set_status_category(&mut self, value: u16) {
        self.status_category = Some(value);
    }

    /// Gets the value of StatusCategory
    pub fn get_status_category(&self) -> Option<&u16> {
        self.status_category.as_ref()
    }

    /// Sets the value of StoragePoolName
    pub fn set_storage_pool_name(&mut self, value: String) {
        self.storage_pool_name = Some(value);
    }

    /// Gets the value of StoragePoolName
    pub fn get_storage_pool_name(&self) -> Option<&String> {
        self.storage_pool_name.as_ref()
    }

    /// Sets the value of TierFootprints
    pub fn set_tier_footprints(&mut self, value: Vec<u64>) {
        self.tier_footprints = value;
    }

    /// Gets the value of TierFootprints
    pub fn get_tier_footprints(&self) -> &Vec<u64> {
        &self.tier_footprints
    }

    /// Sets the value of TierMedias
    pub fn set_tier_medias(&mut self, value: Vec<u16>) {
        self.tier_medias = value;
    }

    /// Gets the value of TierMedias
    pub fn get_tier_medias(&self) -> &Vec<u16> {
        &self.tier_medias
    }

    /// Sets the value of TierResiliencies
    pub fn set_tier_resiliencies(&mut self, value: Vec<u16>) {
        self.tier_resiliencies = value;
    }

    /// Gets the value of TierResiliencies
    pub fn get_tier_resiliencies(&self) -> &Vec<u16> {
        &self.tier_resiliencies
    }

    /// Sets the value of TierSizes
    pub fn set_tier_sizes(&mut self, value: Vec<u64>) {
        self.tier_sizes = value;
    }

    /// Gets the value of TierSizes
    pub fn get_tier_sizes(&self) -> &Vec<u64> {
        &self.tier_sizes
    }

    /// Sets the value of TotalIops
    pub fn set_total_iops(&mut self, value: f64) {
        self.total_iops = Some(value);
    }

    /// Gets the value of TotalIops
    pub fn get_total_iops(&self) -> Option<&f64> {
        self.total_iops.as_ref()
    }

    /// Sets the value of TotalThroughput
    pub fn set_total_throughput(&mut self, value: f64) {
        self.total_throughput = Some(value);
    }

    /// Gets the value of TotalThroughput
    pub fn get_total_throughput(&self) -> Option<&f64> {
        self.total_throughput.as_ref()
    }

/// 

    /// * `new_volume_template` -  (SDDC_VolumeModificationTemplate)
    /// * `return_value` -  (u32)
    pub fn get_new_volume_template(&self, new_volume_template: &mut SDDC_VolumeModificationTemplate) -> Result<(), WmiError> {

        let result = self.invoke_method("GetNewVolumeTemplate", &[])?;
        let new_volume_template = result.get_value("NewVolumeTemplate")?;
        Ok(result.return_value)

    }


/// 

    /// * `backup_reovery_password_to_ad` -  (bool)
    /// * `dedup_mode` -  (u32)
    /// * `enable_bit_locker` -  (bool)
    /// * `is_tiered` -  (bool)
    /// * `password_protector` -  (String)
    /// * `resiliency` -  (u16)
    /// * `set_file_integrity` -  (bool)
    /// * `sizes` -  (u64[])
    /// * `volume_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn new_volume(&self, volume_name: &String, resiliency: u16, is_tiered: bool, sizes: &Vec<u64>, dedup_mode: u32, set_file_integrity: bool, enable_bit_locker: bool, backup_reovery_password_to_ad: bool, password_protector: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeName".to_string(), value: volume_name.into() });
        args.push(MethodParameter { name: "Resiliency".to_string(), value: resiliency.into() });
        args.push(MethodParameter { name: "IsTiered".to_string(), value: is_tiered.into() });
        args.push(MethodParameter { name: "Sizes".to_string(), value: sizes.into() });
        args.push(MethodParameter { name: "DedupMode".to_string(), value: dedup_mode.into() });
        args.push(MethodParameter { name: "SetFileIntegrity".to_string(), value: set_file_integrity.into() });
        args.push(MethodParameter { name: "EnableBitLocker".to_string(), value: enable_bit_locker.into() });
        args.push(MethodParameter { name: "BackupReoveryPasswordToAD".to_string(), value: backup_reovery_password_to_ad.into() });
        args.push(MethodParameter { name: "PasswordProtector".to_string(), value: password_protector.into() });
        self.invoke_method("NewVolume", &args)

    }


/// 

    /// * `resize_volume_template` -  (SDDC_VolumeModificationTemplate)
    /// * `return_value` -  (u32)
    pub fn get_resize_volume_template(&self, resize_volume_template: &mut SDDC_VolumeModificationTemplate) -> Result<(), WmiError> {

        let result = self.invoke_method("GetResizeVolumeTemplate", &[])?;
        let resize_volume_template = result.get_value("ResizeVolumeTemplate")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_tiered` -  (bool)
    /// * `new_size` -  (u64[])

    /// * `return_value` -  (u32)
    pub fn resize_volume(&self, is_tiered: bool, new_size: &Vec<u64>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsTiered".to_string(), value: is_tiered.into() });
        args.push(MethodParameter { name: "NewSize".to_string(), value: new_size.into() });
        self.invoke_method("ResizeVolume", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn delete_volume(&self) -> Result<(), WmiError> {
        self.invoke_method("DeleteVolume", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn online_volume(&self) -> Result<(), WmiError> {
        self.invoke_method("OnlineVolume", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn offline_volume(&self) -> Result<(), WmiError> {
        self.invoke_method("OfflineVolume", &[])

    }


/// 

    /// * `backup_recovery_password_to_ad` -  (bool)
    /// * `password_protector` -  (String)

    /// * `result` -  (SDDC_BitlockerResult)
    /// * `return_value` -  (u32)
    pub fn encrypt_volume(&self, password_protector: &String, backup_recovery_password_to_ad: bool, result: &mut SDDC_BitlockerResult) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PasswordProtector".to_string(), value: password_protector.into() });
        args.push(MethodParameter { name: "BackupRecoveryPasswordToAD".to_string(), value: backup_recovery_password_to_ad.into() });

        let result = self.invoke_method("EncryptVolume", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `result` -  (SDDC_BitlockerResult)
    /// * `return_value` -  (u32)
    pub fn get_encrypted_volume_recovery_password(&self, result: &mut SDDC_BitlockerResult) -> Result<(), WmiError> {

        let result = self.invoke_method("GetEncryptedVolumeRecoveryPassword", &[])?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn decrypt_volume(&self) -> Result<(), WmiError> {
        self.invoke_method("DecryptVolume", &[])

    }


/// 

    /// * `mode` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_dedup_mode(&self, mode: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Mode".to_string(), value: mode.into() });
        self.invoke_method("SetDedupMode", &args)

    }


/// 

    /// * `series_name` -  (String)
    /// * `time_frame` -  (u16)

    /// * `metric` -  (SDDC_Metric)
    /// * `return_value` -  (u32)
    pub fn get_metrics(&self, series_name: &String, time_frame: u16, metric: &mut SDDC_Metric) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SeriesName".to_string(), value: series_name.into() });
        args.push(MethodParameter { name: "TimeFrame".to_string(), value: time_frame.into() });

        let result = self.invoke_method("GetMetrics", &args)?;
        let metric = result.get_value("Metric")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn refresh(&self) -> Result<(), WmiError> {
        self.invoke_method("Refresh", &[])

    }

}

