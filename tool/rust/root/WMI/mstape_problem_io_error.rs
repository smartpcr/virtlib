// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSTapeProblemIoError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSTapeProblemIoError {
    #[serde(flatten)]
    pub base: MSTapeDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NonMediumErrors")]
    pub non_medium_errors: Option<u32>,

/// 
    #[serde(rename = "ReadCorrectedWithDelay")]
    pub read_corrected_with_delay: Option<u32>,

/// 
    #[serde(rename = "ReadCorrectedWithoutDelay")]
    pub read_corrected_without_delay: Option<u32>,

/// 
    #[serde(rename = "ReadCorrectionAlgorithmProcessed")]
    pub read_correction_algorithm_processed: Option<u32>,

/// 
    #[serde(rename = "ReadTotalCorrectedErrors")]
    pub read_total_corrected_errors: Option<u32>,

/// 
    #[serde(rename = "ReadTotalErrors")]
    pub read_total_errors: Option<u32>,

/// 
    #[serde(rename = "ReadTotalUncorrectedErrors")]
    pub read_total_uncorrected_errors: Option<u32>,

/// 
    #[serde(rename = "WriteCorrectedWithDelay")]
    pub write_corrected_with_delay: Option<u32>,

/// 
    #[serde(rename = "WriteCorrectedWithoutDelay")]
    pub write_corrected_without_delay: Option<u32>,

/// 
    #[serde(rename = "WriteCorrectionAlgorithmProcessed")]
    pub write_correction_algorithm_processed: Option<u32>,

/// 
    #[serde(rename = "WriteTotalCorrectedErrors")]
    pub write_total_corrected_errors: Option<u32>,

/// 
    #[serde(rename = "WriteTotalErrors")]
    pub write_total_errors: Option<u32>,

/// 
    #[serde(rename = "WriteTotalUncorrectedErrors")]
    pub write_total_uncorrected_errors: Option<u32>,
}

impl MSTapeProblemIoError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSTapeDriver::new(),
            active: None,
            instance_name: None,
            non_medium_errors: None,
            read_corrected_with_delay: None,
            read_corrected_without_delay: None,
            read_correction_algorithm_processed: None,
            read_total_corrected_errors: None,
            read_total_errors: None,
            read_total_uncorrected_errors: None,
            write_corrected_with_delay: None,
            write_corrected_without_delay: None,
            write_correction_algorithm_processed: None,
            write_total_corrected_errors: None,
            write_total_errors: None,
            write_total_uncorrected_errors: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NonMediumErrors
    pub fn set_non_medium_errors(&mut self, value: u32) {
        self.non_medium_errors = Some(value);
    }

    /// Gets the value of NonMediumErrors
    pub fn get_non_medium_errors(&self) -> Option<&u32> {
        self.non_medium_errors.as_ref()
    }

    /// Sets the value of ReadCorrectedWithDelay
    pub fn set_read_corrected_with_delay(&mut self, value: u32) {
        self.read_corrected_with_delay = Some(value);
    }

    /// Gets the value of ReadCorrectedWithDelay
    pub fn get_read_corrected_with_delay(&self) -> Option<&u32> {
        self.read_corrected_with_delay.as_ref()
    }

    /// Sets the value of ReadCorrectedWithoutDelay
    pub fn set_read_corrected_without_delay(&mut self, value: u32) {
        self.read_corrected_without_delay = Some(value);
    }

    /// Gets the value of ReadCorrectedWithoutDelay
    pub fn get_read_corrected_without_delay(&self) -> Option<&u32> {
        self.read_corrected_without_delay.as_ref()
    }

    /// Sets the value of ReadCorrectionAlgorithmProcessed
    pub fn set_read_correction_algorithm_processed(&mut self, value: u32) {
        self.read_correction_algorithm_processed = Some(value);
    }

    /// Gets the value of ReadCorrectionAlgorithmProcessed
    pub fn get_read_correction_algorithm_processed(&self) -> Option<&u32> {
        self.read_correction_algorithm_processed.as_ref()
    }

    /// Sets the value of ReadTotalCorrectedErrors
    pub fn set_read_total_corrected_errors(&mut self, value: u32) {
        self.read_total_corrected_errors = Some(value);
    }

    /// Gets the value of ReadTotalCorrectedErrors
    pub fn get_read_total_corrected_errors(&self) -> Option<&u32> {
        self.read_total_corrected_errors.as_ref()
    }

    /// Sets the value of ReadTotalErrors
    pub fn set_read_total_errors(&mut self, value: u32) {
        self.read_total_errors = Some(value);
    }

    /// Gets the value of ReadTotalErrors
    pub fn get_read_total_errors(&self) -> Option<&u32> {
        self.read_total_errors.as_ref()
    }

    /// Sets the value of ReadTotalUncorrectedErrors
    pub fn set_read_total_uncorrected_errors(&mut self, value: u32) {
        self.read_total_uncorrected_errors = Some(value);
    }

    /// Gets the value of ReadTotalUncorrectedErrors
    pub fn get_read_total_uncorrected_errors(&self) -> Option<&u32> {
        self.read_total_uncorrected_errors.as_ref()
    }

    /// Sets the value of WriteCorrectedWithDelay
    pub fn set_write_corrected_with_delay(&mut self, value: u32) {
        self.write_corrected_with_delay = Some(value);
    }

    /// Gets the value of WriteCorrectedWithDelay
    pub fn get_write_corrected_with_delay(&self) -> Option<&u32> {
        self.write_corrected_with_delay.as_ref()
    }

    /// Sets the value of WriteCorrectedWithoutDelay
    pub fn set_write_corrected_without_delay(&mut self, value: u32) {
        self.write_corrected_without_delay = Some(value);
    }

    /// Gets the value of WriteCorrectedWithoutDelay
    pub fn get_write_corrected_without_delay(&self) -> Option<&u32> {
        self.write_corrected_without_delay.as_ref()
    }

    /// Sets the value of WriteCorrectionAlgorithmProcessed
    pub fn set_write_correction_algorithm_processed(&mut self, value: u32) {
        self.write_correction_algorithm_processed = Some(value);
    }

    /// Gets the value of WriteCorrectionAlgorithmProcessed
    pub fn get_write_correction_algorithm_processed(&self) -> Option<&u32> {
        self.write_correction_algorithm_processed.as_ref()
    }

    /// Sets the value of WriteTotalCorrectedErrors
    pub fn set_write_total_corrected_errors(&mut self, value: u32) {
        self.write_total_corrected_errors = Some(value);
    }

    /// Gets the value of WriteTotalCorrectedErrors
    pub fn get_write_total_corrected_errors(&self) -> Option<&u32> {
        self.write_total_corrected_errors.as_ref()
    }

    /// Sets the value of WriteTotalErrors
    pub fn set_write_total_errors(&mut self, value: u32) {
        self.write_total_errors = Some(value);
    }

    /// Gets the value of WriteTotalErrors
    pub fn get_write_total_errors(&self) -> Option<&u32> {
        self.write_total_errors.as_ref()
    }

    /// Sets the value of WriteTotalUncorrectedErrors
    pub fn set_write_total_uncorrected_errors(&mut self, value: u32) {
        self.write_total_uncorrected_errors = Some(value);
    }

    /// Gets the value of WriteTotalUncorrectedErrors
    pub fn get_write_total_uncorrected_errors(&self) -> Option<&u32> {
        self.write_total_uncorrected_errors.as_ref()
    }
}

