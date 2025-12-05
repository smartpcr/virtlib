// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Server struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Server {

/// 
    #[serde(rename = "Alerts")]
    pub alerts: Vec<SDDC_Alert>,

/// 
    #[serde(rename = "BuildNumber")]
    pub build_number: Option<String>,

/// 
    #[serde(rename = "Chassis")]
    pub chassis: Option<String>,

/// 
    #[serde(rename = "Cluster")]
    pub cluster: Option<String>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "FreePhysicalMemoryInBytes")]
    pub free_physical_memory_in_bytes: Option<u64>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsBitlockerFeatureInstalled")]
    pub is_bitlocker_feature_installed: Option<bool>,

/// 
    #[serde(rename = "IsDataDedupFeatureInstalled")]
    pub is_data_dedup_feature_installed: Option<bool>,

/// 
    #[serde(rename = "IsStorageReplicaFeatureInstalled")]
    pub is_storage_replica_feature_installed: Option<bool>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MemoryDIMMs")]
    pub memory_dimms: Vec<SDDC_Memory>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OSName")]
    pub osname: Option<String>,

/// 
    #[serde(rename = "OSVersion")]
    pub osversion: Option<String>,

/// 
    #[serde(rename = "Processors")]
    pub processors: Vec<SDDC_Processor>,

/// 
    #[serde(rename = "Rack")]
    pub rack: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "Site")]
    pub site: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Vec<u16>,

/// 
    #[serde(rename = "StatusCategory")]
    pub status_category: Option<u16>,

/// 
    #[serde(rename = "TotalNetworkUsageInBytesPerSecond")]
    pub total_network_usage_in_bytes_per_second: Option<f64>,

/// 
    #[serde(rename = "TotalProcessorsIdlePercentage")]
    pub total_processors_idle_percentage: Option<u64>,

/// 
    #[serde(rename = "TotalRdmaUsageInBytesPerSecond")]
    pub total_rdma_usage_in_bytes_per_second: Option<f64>,

/// 
    #[serde(rename = "Uptime")]
    pub uptime: Option<String>,
}

impl SDDC_Server {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            build_number: None,
            chassis: None,
            cluster: None,
            domain: None,
            free_physical_memory_in_bytes: None,
            id: None,
            is_bitlocker_feature_installed: None,
            is_data_dedup_feature_installed: None,
            is_storage_replica_feature_installed: None,
            location: None,
            manufacturer: None,
            memory_dimms: Vec::new(),
            model: None,
            name: None,
            osname: None,
            osversion: None,
            processors: Vec::new(),
            rack: None,
            serial_number: None,
            site: None,
            status: Vec::new(),
            status_category: None,
            total_network_usage_in_bytes_per_second: None,
            total_processors_idle_percentage: None,
            total_rdma_usage_in_bytes_per_second: None,
            uptime: None,
        }
    }


    /// Sets the value of Alerts
    pub fn set_alerts(&mut self, value: Vec<SDDC_Alert>) {
        self.alerts = value;
    }

    /// Gets the value of Alerts
    pub fn get_alerts(&self) -> &Vec<SDDC_Alert> {
        &self.alerts
    }

    /// Sets the value of BuildNumber
    pub fn set_build_number(&mut self, value: String) {
        self.build_number = Some(value);
    }

    /// Gets the value of BuildNumber
    pub fn get_build_number(&self) -> Option<&String> {
        self.build_number.as_ref()
    }

    /// Sets the value of Chassis
    pub fn set_chassis(&mut self, value: String) {
        self.chassis = Some(value);
    }

    /// Gets the value of Chassis
    pub fn get_chassis(&self) -> Option<&String> {
        self.chassis.as_ref()
    }

    /// Sets the value of Cluster
    pub fn set_cluster(&mut self, value: String) {
        self.cluster = Some(value);
    }

    /// Gets the value of Cluster
    pub fn get_cluster(&self) -> Option<&String> {
        self.cluster.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of FreePhysicalMemoryInBytes
    pub fn set_free_physical_memory_in_bytes(&mut self, value: u64) {
        self.free_physical_memory_in_bytes = Some(value);
    }

    /// Gets the value of FreePhysicalMemoryInBytes
    pub fn get_free_physical_memory_in_bytes(&self) -> Option<&u64> {
        self.free_physical_memory_in_bytes.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsBitlockerFeatureInstalled
    pub fn set_is_bitlocker_feature_installed(&mut self, value: bool) {
        self.is_bitlocker_feature_installed = Some(value);
    }

    /// Gets the value of IsBitlockerFeatureInstalled
    pub fn get_is_bitlocker_feature_installed(&self) -> Option<&bool> {
        self.is_bitlocker_feature_installed.as_ref()
    }

    /// Sets the value of IsDataDedupFeatureInstalled
    pub fn set_is_data_dedup_feature_installed(&mut self, value: bool) {
        self.is_data_dedup_feature_installed = Some(value);
    }

    /// Gets the value of IsDataDedupFeatureInstalled
    pub fn get_is_data_dedup_feature_installed(&self) -> Option<&bool> {
        self.is_data_dedup_feature_installed.as_ref()
    }

    /// Sets the value of IsStorageReplicaFeatureInstalled
    pub fn set_is_storage_replica_feature_installed(&mut self, value: bool) {
        self.is_storage_replica_feature_installed = Some(value);
    }

    /// Gets the value of IsStorageReplicaFeatureInstalled
    pub fn get_is_storage_replica_feature_installed(&self) -> Option<&bool> {
        self.is_storage_replica_feature_installed.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MemoryDIMMs
    pub fn set_memory_dimms(&mut self, value: Vec<SDDC_Memory>) {
        self.memory_dimms = value;
    }

    /// Gets the value of MemoryDIMMs
    pub fn get_memory_dimms(&self) -> &Vec<SDDC_Memory> {
        &self.memory_dimms
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OSName
    pub fn set_osname(&mut self, value: String) {
        self.osname = Some(value);
    }

    /// Gets the value of OSName
    pub fn get_osname(&self) -> Option<&String> {
        self.osname.as_ref()
    }

    /// Sets the value of OSVersion
    pub fn set_osversion(&mut self, value: String) {
        self.osversion = Some(value);
    }

    /// Gets the value of OSVersion
    pub fn get_osversion(&self) -> Option<&String> {
        self.osversion.as_ref()
    }

    /// Sets the value of Processors
    pub fn set_processors(&mut self, value: Vec<SDDC_Processor>) {
        self.processors = value;
    }

    /// Gets the value of Processors
    pub fn get_processors(&self) -> &Vec<SDDC_Processor> {
        &self.processors
    }

    /// Sets the value of Rack
    pub fn set_rack(&mut self, value: String) {
        self.rack = Some(value);
    }

    /// Gets the value of Rack
    pub fn get_rack(&self) -> Option<&String> {
        self.rack.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of Site
    pub fn set_site(&mut self, value: String) {
        self.site = Some(value);
    }

    /// Gets the value of Site
    pub fn get_site(&self) -> Option<&String> {
        self.site.as_ref()
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

    /// Sets the value of TotalNetworkUsageInBytesPerSecond
    pub fn set_total_network_usage_in_bytes_per_second(&mut self, value: f64) {
        self.total_network_usage_in_bytes_per_second = Some(value);
    }

    /// Gets the value of TotalNetworkUsageInBytesPerSecond
    pub fn get_total_network_usage_in_bytes_per_second(&self) -> Option<&f64> {
        self.total_network_usage_in_bytes_per_second.as_ref()
    }

    /// Sets the value of TotalProcessorsIdlePercentage
    pub fn set_total_processors_idle_percentage(&mut self, value: u64) {
        self.total_processors_idle_percentage = Some(value);
    }

    /// Gets the value of TotalProcessorsIdlePercentage
    pub fn get_total_processors_idle_percentage(&self) -> Option<&u64> {
        self.total_processors_idle_percentage.as_ref()
    }

    /// Sets the value of TotalRdmaUsageInBytesPerSecond
    pub fn set_total_rdma_usage_in_bytes_per_second(&mut self, value: f64) {
        self.total_rdma_usage_in_bytes_per_second = Some(value);
    }

    /// Gets the value of TotalRdmaUsageInBytesPerSecond
    pub fn get_total_rdma_usage_in_bytes_per_second(&self) -> Option<&f64> {
        self.total_rdma_usage_in_bytes_per_second.as_ref()
    }

    /// Sets the value of Uptime
    pub fn set_uptime(&mut self, value: String) {
        self.uptime = Some(value);
    }

    /// Gets the value of Uptime
    pub fn get_uptime(&self) -> Option<&String> {
        self.uptime.as_ref()
    }

/// 

    /// * `server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_server(&self, server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        self.invoke_method("AddServer", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn remove_server(&self) -> Result<(), WmiError> {
        self.invoke_method("RemoveServer", &[])

    }


/// 

    /// * `drain_server` -  (bool)

    /// * `return_value` -  (u32)
    pub fn pause_server(&self, drain_server: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DrainServer".to_string(), value: drain_server.into() });
        self.invoke_method("PauseServer", &args)

    }


/// 

    /// * `failback_type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn resume_server(&self, failback_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FailbackType".to_string(), value: failback_type.into() });
        self.invoke_method("ResumeServer", &args)

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

}

