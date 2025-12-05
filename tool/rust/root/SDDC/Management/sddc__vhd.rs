// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Vhd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Vhd {

/// 
    #[serde(rename = "Alerts")]
    pub alerts: Vec<SDDC_Alert>,

/// 
    #[serde(rename = "AverageLatency")]
    pub average_latency: Option<f64>,

/// 
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "SizeUsed")]
    pub size_used: Option<u64>,

/// 
    #[serde(rename = "TotalIops")]
    pub total_iops: Option<f64>,

/// 
    #[serde(rename = "TotalThroughput")]
    pub total_throughput: Option<f64>,

/// 
    #[serde(rename = "VhdFormat")]
    pub vhd_format: Option<u16>,

/// 
    #[serde(rename = "VhdType")]
    pub vhd_type: Option<u16>,

/// 
    #[serde(rename = "VolumeId")]
    pub volume_id: Option<String>,
}

impl SDDC_Vhd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            average_latency: None,
            file_path: None,
            id: None,
            size: None,
            size_used: None,
            total_iops: None,
            total_throughput: None,
            vhd_format: None,
            vhd_type: None,
            volume_id: None,
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

    /// Sets the value of AverageLatency
    pub fn set_average_latency(&mut self, value: f64) {
        self.average_latency = Some(value);
    }

    /// Gets the value of AverageLatency
    pub fn get_average_latency(&self) -> Option<&f64> {
        self.average_latency.as_ref()
    }

    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of SizeUsed
    pub fn set_size_used(&mut self, value: u64) {
        self.size_used = Some(value);
    }

    /// Gets the value of SizeUsed
    pub fn get_size_used(&self) -> Option<&u64> {
        self.size_used.as_ref()
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

    /// Sets the value of VhdFormat
    pub fn set_vhd_format(&mut self, value: u16) {
        self.vhd_format = Some(value);
    }

    /// Gets the value of VhdFormat
    pub fn get_vhd_format(&self) -> Option<&u16> {
        self.vhd_format.as_ref()
    }

    /// Sets the value of VhdType
    pub fn set_vhd_type(&mut self, value: u16) {
        self.vhd_type = Some(value);
    }

    /// Gets the value of VhdType
    pub fn get_vhd_type(&self) -> Option<&u16> {
        self.vhd_type.as_ref()
    }

    /// Sets the value of VolumeId
    pub fn set_volume_id(&mut self, value: String) {
        self.volume_id = Some(value);
    }

    /// Gets the value of VolumeId
    pub fn get_volume_id(&self) -> Option<&String> {
        self.volume_id.as_ref()
    }

/// 

    /// * `file_path` -  (String)
    /// * `series_name` -  (String)
    /// * `time_frame` -  (u16)

    /// * `metric` -  (SDDC_Metric)
    /// * `return_value` -  (u32)
    pub fn get_metrics(&self, file_path: &String, series_name: &String, time_frame: u16, metric: &mut SDDC_Metric) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });
        args.push(MethodParameter { name: "SeriesName".to_string(), value: series_name.into() });
        args.push(MethodParameter { name: "TimeFrame".to_string(), value: time_frame.into() });

        let result = self.invoke_method("GetMetrics", &args)?;
        let metric = result.get_value("Metric")?;
        Ok(result.return_value)

    }

}

