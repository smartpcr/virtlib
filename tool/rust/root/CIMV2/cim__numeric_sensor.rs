// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NumericSensor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NumericSensor {
    #[serde(flatten)]
    pub base: CIM_Sensor,

/// 
    #[serde(rename = "Accuracy")]
    pub accuracy: Option<i32>,

/// 
    #[serde(rename = "CurrentReading")]
    pub current_reading: Option<i32>,

/// 
    #[serde(rename = "IsLinear")]
    pub is_linear: Option<bool>,

/// 
    #[serde(rename = "LowerThresholdCritical")]
    pub lower_threshold_critical: Option<i32>,

/// 
    #[serde(rename = "LowerThresholdFatal")]
    pub lower_threshold_fatal: Option<i32>,

/// 
    #[serde(rename = "LowerThresholdNonCritical")]
    pub lower_threshold_non_critical: Option<i32>,

/// 
    #[serde(rename = "MaxReadable")]
    pub max_readable: Option<i32>,

/// 
    #[serde(rename = "MinReadable")]
    pub min_readable: Option<i32>,

/// 
    #[serde(rename = "NominalReading")]
    pub nominal_reading: Option<i32>,

/// 
    #[serde(rename = "NormalMax")]
    pub normal_max: Option<i32>,

/// 
    #[serde(rename = "NormalMin")]
    pub normal_min: Option<i32>,

/// 
    #[serde(rename = "Resolution")]
    pub resolution: Option<u32>,

/// 
    #[serde(rename = "Tolerance")]
    pub tolerance: Option<i32>,

/// 
    #[serde(rename = "UpperThresholdCritical")]
    pub upper_threshold_critical: Option<i32>,

/// 
    #[serde(rename = "UpperThresholdFatal")]
    pub upper_threshold_fatal: Option<i32>,

/// 
    #[serde(rename = "UpperThresholdNonCritical")]
    pub upper_threshold_non_critical: Option<i32>,
}

impl CIM_NumericSensor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Sensor::new(),
            accuracy: None,
            current_reading: None,
            is_linear: None,
            lower_threshold_critical: None,
            lower_threshold_fatal: None,
            lower_threshold_non_critical: None,
            max_readable: None,
            min_readable: None,
            nominal_reading: None,
            normal_max: None,
            normal_min: None,
            resolution: None,
            tolerance: None,
            upper_threshold_critical: None,
            upper_threshold_fatal: None,
            upper_threshold_non_critical: None,
        }
    }


    /// Sets the value of Accuracy
    pub fn set_accuracy(&mut self, value: i32) {
        self.accuracy = Some(value);
    }

    /// Gets the value of Accuracy
    pub fn get_accuracy(&self) -> Option<&i32> {
        self.accuracy.as_ref()
    }

    /// Sets the value of CurrentReading
    pub fn set_current_reading(&mut self, value: i32) {
        self.current_reading = Some(value);
    }

    /// Gets the value of CurrentReading
    pub fn get_current_reading(&self) -> Option<&i32> {
        self.current_reading.as_ref()
    }

    /// Sets the value of IsLinear
    pub fn set_is_linear(&mut self, value: bool) {
        self.is_linear = Some(value);
    }

    /// Gets the value of IsLinear
    pub fn get_is_linear(&self) -> Option<&bool> {
        self.is_linear.as_ref()
    }

    /// Sets the value of LowerThresholdCritical
    pub fn set_lower_threshold_critical(&mut self, value: i32) {
        self.lower_threshold_critical = Some(value);
    }

    /// Gets the value of LowerThresholdCritical
    pub fn get_lower_threshold_critical(&self) -> Option<&i32> {
        self.lower_threshold_critical.as_ref()
    }

    /// Sets the value of LowerThresholdFatal
    pub fn set_lower_threshold_fatal(&mut self, value: i32) {
        self.lower_threshold_fatal = Some(value);
    }

    /// Gets the value of LowerThresholdFatal
    pub fn get_lower_threshold_fatal(&self) -> Option<&i32> {
        self.lower_threshold_fatal.as_ref()
    }

    /// Sets the value of LowerThresholdNonCritical
    pub fn set_lower_threshold_non_critical(&mut self, value: i32) {
        self.lower_threshold_non_critical = Some(value);
    }

    /// Gets the value of LowerThresholdNonCritical
    pub fn get_lower_threshold_non_critical(&self) -> Option<&i32> {
        self.lower_threshold_non_critical.as_ref()
    }

    /// Sets the value of MaxReadable
    pub fn set_max_readable(&mut self, value: i32) {
        self.max_readable = Some(value);
    }

    /// Gets the value of MaxReadable
    pub fn get_max_readable(&self) -> Option<&i32> {
        self.max_readable.as_ref()
    }

    /// Sets the value of MinReadable
    pub fn set_min_readable(&mut self, value: i32) {
        self.min_readable = Some(value);
    }

    /// Gets the value of MinReadable
    pub fn get_min_readable(&self) -> Option<&i32> {
        self.min_readable.as_ref()
    }

    /// Sets the value of NominalReading
    pub fn set_nominal_reading(&mut self, value: i32) {
        self.nominal_reading = Some(value);
    }

    /// Gets the value of NominalReading
    pub fn get_nominal_reading(&self) -> Option<&i32> {
        self.nominal_reading.as_ref()
    }

    /// Sets the value of NormalMax
    pub fn set_normal_max(&mut self, value: i32) {
        self.normal_max = Some(value);
    }

    /// Gets the value of NormalMax
    pub fn get_normal_max(&self) -> Option<&i32> {
        self.normal_max.as_ref()
    }

    /// Sets the value of NormalMin
    pub fn set_normal_min(&mut self, value: i32) {
        self.normal_min = Some(value);
    }

    /// Gets the value of NormalMin
    pub fn get_normal_min(&self) -> Option<&i32> {
        self.normal_min.as_ref()
    }

    /// Sets the value of Resolution
    pub fn set_resolution(&mut self, value: u32) {
        self.resolution = Some(value);
    }

    /// Gets the value of Resolution
    pub fn get_resolution(&self) -> Option<&u32> {
        self.resolution.as_ref()
    }

    /// Sets the value of Tolerance
    pub fn set_tolerance(&mut self, value: i32) {
        self.tolerance = Some(value);
    }

    /// Gets the value of Tolerance
    pub fn get_tolerance(&self) -> Option<&i32> {
        self.tolerance.as_ref()
    }

    /// Sets the value of UpperThresholdCritical
    pub fn set_upper_threshold_critical(&mut self, value: i32) {
        self.upper_threshold_critical = Some(value);
    }

    /// Gets the value of UpperThresholdCritical
    pub fn get_upper_threshold_critical(&self) -> Option<&i32> {
        self.upper_threshold_critical.as_ref()
    }

    /// Sets the value of UpperThresholdFatal
    pub fn set_upper_threshold_fatal(&mut self, value: i32) {
        self.upper_threshold_fatal = Some(value);
    }

    /// Gets the value of UpperThresholdFatal
    pub fn get_upper_threshold_fatal(&self) -> Option<&i32> {
        self.upper_threshold_fatal.as_ref()
    }

    /// Sets the value of UpperThresholdNonCritical
    pub fn set_upper_threshold_non_critical(&mut self, value: i32) {
        self.upper_threshold_non_critical = Some(value);
    }

    /// Gets the value of UpperThresholdNonCritical
    pub fn get_upper_threshold_non_critical(&self) -> Option<&i32> {
        self.upper_threshold_non_critical.as_ref()
    }
}

