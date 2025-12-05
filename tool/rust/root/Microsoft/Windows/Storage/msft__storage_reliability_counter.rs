// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageReliabilityCounter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageReliabilityCounter {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "FlushLatencyMax")]
    pub flush_latency_max: Option<u64>,

/// 
    #[serde(rename = "LoadUnloadCycleCount")]
    pub load_unload_cycle_count: Option<u32>,

/// 
    #[serde(rename = "LoadUnloadCycleCountMax")]
    pub load_unload_cycle_count_max: Option<u32>,

/// 
    #[serde(rename = "ManufactureDate")]
    pub manufacture_date: Option<String>,

/// 
    #[serde(rename = "PowerOnHours")]
    pub power_on_hours: Option<u32>,

/// 
    #[serde(rename = "ReadErrorsCorrected")]
    pub read_errors_corrected: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsTotal")]
    pub read_errors_total: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsUncorrected")]
    pub read_errors_uncorrected: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyMax")]
    pub read_latency_max: Option<u64>,

/// 
    #[serde(rename = "StartStopCycleCount")]
    pub start_stop_cycle_count: Option<u32>,

/// 
    #[serde(rename = "StartStopCycleCountMax")]
    pub start_stop_cycle_count_max: Option<u32>,

/// 
    #[serde(rename = "Temperature")]
    pub temperature: Option<u8>,

/// 
    #[serde(rename = "TemperatureMax")]
    pub temperature_max: Option<u8>,

/// 
    #[serde(rename = "Wear")]
    pub wear: Option<u8>,

/// 
    #[serde(rename = "WriteErrorsCorrected")]
    pub write_errors_corrected: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsTotal")]
    pub write_errors_total: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsUncorrected")]
    pub write_errors_uncorrected: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyMax")]
    pub write_latency_max: Option<u64>,
}

impl MSFT_StorageReliabilityCounter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            device_id: None,
            flush_latency_max: None,
            load_unload_cycle_count: None,
            load_unload_cycle_count_max: None,
            manufacture_date: None,
            power_on_hours: None,
            read_errors_corrected: None,
            read_errors_total: None,
            read_errors_uncorrected: None,
            read_latency_max: None,
            start_stop_cycle_count: None,
            start_stop_cycle_count_max: None,
            temperature: None,
            temperature_max: None,
            wear: None,
            write_errors_corrected: None,
            write_errors_total: None,
            write_errors_uncorrected: None,
            write_latency_max: None,
        }
    }


    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of FlushLatencyMax
    pub fn set_flush_latency_max(&mut self, value: u64) {
        self.flush_latency_max = Some(value);
    }

    /// Gets the value of FlushLatencyMax
    pub fn get_flush_latency_max(&self) -> Option<&u64> {
        self.flush_latency_max.as_ref()
    }

    /// Sets the value of LoadUnloadCycleCount
    pub fn set_load_unload_cycle_count(&mut self, value: u32) {
        self.load_unload_cycle_count = Some(value);
    }

    /// Gets the value of LoadUnloadCycleCount
    pub fn get_load_unload_cycle_count(&self) -> Option<&u32> {
        self.load_unload_cycle_count.as_ref()
    }

    /// Sets the value of LoadUnloadCycleCountMax
    pub fn set_load_unload_cycle_count_max(&mut self, value: u32) {
        self.load_unload_cycle_count_max = Some(value);
    }

    /// Gets the value of LoadUnloadCycleCountMax
    pub fn get_load_unload_cycle_count_max(&self) -> Option<&u32> {
        self.load_unload_cycle_count_max.as_ref()
    }

    /// Sets the value of ManufactureDate
    pub fn set_manufacture_date(&mut self, value: String) {
        self.manufacture_date = Some(value);
    }

    /// Gets the value of ManufactureDate
    pub fn get_manufacture_date(&self) -> Option<&String> {
        self.manufacture_date.as_ref()
    }

    /// Sets the value of PowerOnHours
    pub fn set_power_on_hours(&mut self, value: u32) {
        self.power_on_hours = Some(value);
    }

    /// Gets the value of PowerOnHours
    pub fn get_power_on_hours(&self) -> Option<&u32> {
        self.power_on_hours.as_ref()
    }

    /// Sets the value of ReadErrorsCorrected
    pub fn set_read_errors_corrected(&mut self, value: u64) {
        self.read_errors_corrected = Some(value);
    }

    /// Gets the value of ReadErrorsCorrected
    pub fn get_read_errors_corrected(&self) -> Option<&u64> {
        self.read_errors_corrected.as_ref()
    }

    /// Sets the value of ReadErrorsTotal
    pub fn set_read_errors_total(&mut self, value: u64) {
        self.read_errors_total = Some(value);
    }

    /// Gets the value of ReadErrorsTotal
    pub fn get_read_errors_total(&self) -> Option<&u64> {
        self.read_errors_total.as_ref()
    }

    /// Sets the value of ReadErrorsUncorrected
    pub fn set_read_errors_uncorrected(&mut self, value: u64) {
        self.read_errors_uncorrected = Some(value);
    }

    /// Gets the value of ReadErrorsUncorrected
    pub fn get_read_errors_uncorrected(&self) -> Option<&u64> {
        self.read_errors_uncorrected.as_ref()
    }

    /// Sets the value of ReadLatencyMax
    pub fn set_read_latency_max(&mut self, value: u64) {
        self.read_latency_max = Some(value);
    }

    /// Gets the value of ReadLatencyMax
    pub fn get_read_latency_max(&self) -> Option<&u64> {
        self.read_latency_max.as_ref()
    }

    /// Sets the value of StartStopCycleCount
    pub fn set_start_stop_cycle_count(&mut self, value: u32) {
        self.start_stop_cycle_count = Some(value);
    }

    /// Gets the value of StartStopCycleCount
    pub fn get_start_stop_cycle_count(&self) -> Option<&u32> {
        self.start_stop_cycle_count.as_ref()
    }

    /// Sets the value of StartStopCycleCountMax
    pub fn set_start_stop_cycle_count_max(&mut self, value: u32) {
        self.start_stop_cycle_count_max = Some(value);
    }

    /// Gets the value of StartStopCycleCountMax
    pub fn get_start_stop_cycle_count_max(&self) -> Option<&u32> {
        self.start_stop_cycle_count_max.as_ref()
    }

    /// Sets the value of Temperature
    pub fn set_temperature(&mut self, value: u8) {
        self.temperature = Some(value);
    }

    /// Gets the value of Temperature
    pub fn get_temperature(&self) -> Option<&u8> {
        self.temperature.as_ref()
    }

    /// Sets the value of TemperatureMax
    pub fn set_temperature_max(&mut self, value: u8) {
        self.temperature_max = Some(value);
    }

    /// Gets the value of TemperatureMax
    pub fn get_temperature_max(&self) -> Option<&u8> {
        self.temperature_max.as_ref()
    }

    /// Sets the value of Wear
    pub fn set_wear(&mut self, value: u8) {
        self.wear = Some(value);
    }

    /// Gets the value of Wear
    pub fn get_wear(&self) -> Option<&u8> {
        self.wear.as_ref()
    }

    /// Sets the value of WriteErrorsCorrected
    pub fn set_write_errors_corrected(&mut self, value: u64) {
        self.write_errors_corrected = Some(value);
    }

    /// Gets the value of WriteErrorsCorrected
    pub fn get_write_errors_corrected(&self) -> Option<&u64> {
        self.write_errors_corrected.as_ref()
    }

    /// Sets the value of WriteErrorsTotal
    pub fn set_write_errors_total(&mut self, value: u64) {
        self.write_errors_total = Some(value);
    }

    /// Gets the value of WriteErrorsTotal
    pub fn get_write_errors_total(&self) -> Option<&u64> {
        self.write_errors_total.as_ref()
    }

    /// Sets the value of WriteErrorsUncorrected
    pub fn set_write_errors_uncorrected(&mut self, value: u64) {
        self.write_errors_uncorrected = Some(value);
    }

    /// Gets the value of WriteErrorsUncorrected
    pub fn get_write_errors_uncorrected(&self) -> Option<&u64> {
        self.write_errors_uncorrected.as_ref()
    }

    /// Sets the value of WriteLatencyMax
    pub fn set_write_latency_max(&mut self, value: u64) {
        self.write_latency_max = Some(value);
    }

    /// Gets the value of WriteLatencyMax
    pub fn get_write_latency_max(&self) -> Option<&u64> {
        self.write_latency_max.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn reset(&self) -> Result<(), WmiError> {
        self.invoke_method("Reset", &[])

    }

}

