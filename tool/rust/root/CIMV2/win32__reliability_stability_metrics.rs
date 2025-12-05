// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ReliabilityStabilityMetrics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ReliabilityStabilityMetrics {
    #[serde(flatten)]
    pub base: Win32_Reliability,

/// 
    #[serde(rename = "EndMeasurementDate")]
    pub end_measurement_date: Option<String>,

/// 
    #[serde(rename = "RelID")]
    pub rel_id: Option<String>,

/// 
    #[serde(rename = "StartMeasurementDate")]
    pub start_measurement_date: Option<String>,

/// 
    #[serde(rename = "SystemStabilityIndex")]
    pub system_stability_index: Option<f64>,

/// 
    #[serde(rename = "TimeGenerated")]
    pub time_generated: Option<String>,
}

impl Win32_ReliabilityStabilityMetrics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_Reliability::new(),
            end_measurement_date: None,
            rel_id: None,
            start_measurement_date: None,
            system_stability_index: None,
            time_generated: None,
        }
    }


    /// Sets the value of EndMeasurementDate
    pub fn set_end_measurement_date(&mut self, value: String) {
        self.end_measurement_date = Some(value);
    }

    /// Gets the value of EndMeasurementDate
    pub fn get_end_measurement_date(&self) -> Option<&String> {
        self.end_measurement_date.as_ref()
    }

    /// Sets the value of RelID
    pub fn set_rel_id(&mut self, value: String) {
        self.rel_id = Some(value);
    }

    /// Gets the value of RelID
    pub fn get_rel_id(&self) -> Option<&String> {
        self.rel_id.as_ref()
    }

    /// Sets the value of StartMeasurementDate
    pub fn set_start_measurement_date(&mut self, value: String) {
        self.start_measurement_date = Some(value);
    }

    /// Gets the value of StartMeasurementDate
    pub fn get_start_measurement_date(&self) -> Option<&String> {
        self.start_measurement_date.as_ref()
    }

    /// Sets the value of SystemStabilityIndex
    pub fn set_system_stability_index(&mut self, value: f64) {
        self.system_stability_index = Some(value);
    }

    /// Gets the value of SystemStabilityIndex
    pub fn get_system_stability_index(&self) -> Option<&f64> {
        self.system_stability_index.as_ref()
    }

    /// Sets the value of TimeGenerated
    pub fn set_time_generated(&mut self, value: String) {
        self.time_generated = Some(value);
    }

    /// Gets the value of TimeGenerated
    pub fn get_time_generated(&self) -> Option<&String> {
        self.time_generated.as_ref()
    }

/// 

    /// * `record_count` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_record_count(&self, record_count: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetRecordCount", &[])?;
        let record_count = result.get_value("RecordCount")?;
        Ok(result.return_value)

    }

}

