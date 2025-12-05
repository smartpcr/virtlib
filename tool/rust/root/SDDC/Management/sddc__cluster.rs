// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Cluster struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Cluster {

/// 
    #[serde(rename = "Alerts")]
    pub alerts: Vec<SDDC_Alert>,

/// 
    #[serde(rename = "IsStretch")]
    pub is_stretch: Option<bool>,

/// 
    #[serde(rename = "Jobs")]
    pub jobs: Vec<SDDC_Job>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "StoragePools")]
    pub storage_pools: Vec<String>,
}

impl SDDC_Cluster {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            is_stretch: None,
            jobs: Vec::new(),
            name: None,
            storage_pools: Vec::new(),
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

    /// Sets the value of IsStretch
    pub fn set_is_stretch(&mut self, value: bool) {
        self.is_stretch = Some(value);
    }

    /// Gets the value of IsStretch
    pub fn get_is_stretch(&self) -> Option<&bool> {
        self.is_stretch.as_ref()
    }

    /// Sets the value of Jobs
    pub fn set_jobs(&mut self, value: Vec<SDDC_Job>) {
        self.jobs = value;
    }

    /// Gets the value of Jobs
    pub fn get_jobs(&self) -> &Vec<SDDC_Job> {
        &self.jobs
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of StoragePools
    pub fn set_storage_pools(&mut self, value: Vec<String>) {
        self.storage_pools = value;
    }

    /// Gets the value of StoragePools
    pub fn get_storage_pools(&self) -> &Vec<String> {
        &self.storage_pools
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

