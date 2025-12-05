// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Drive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Drive {

/// 
    #[serde(rename = "Alerts")]
    pub alerts: Vec<SDDC_Alert>,

/// 
    #[serde(rename = "AverageLatency")]
    pub average_latency: Option<f64>,

/// 
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsIndicationEnabled")]
    pub is_indication_enabled: Option<bool>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "PowerOnHours")]
    pub power_on_hours: Option<u32>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "Server")]
    pub server: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "SizeUsed")]
    pub size_used: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Vec<u16>,

/// 
    #[serde(rename = "StatusCategory")]
    pub status_category: Option<u16>,

/// 
    #[serde(rename = "StoragePool")]
    pub storage_pool: Option<String>,

/// 
    #[serde(rename = "TemperatureInCelsius")]
    pub temperature_in_celsius: Option<u8>,

/// 
    #[serde(rename = "TotalIops")]
    pub total_iops: Option<f64>,

/// 
    #[serde(rename = "TotalThroughput")]
    pub total_throughput: Option<f64>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,

/// 
    #[serde(rename = "UsedFor")]
    pub used_for: Option<u16>,

/// 
    #[serde(rename = "WearPercentage")]
    pub wear_percentage: Option<u8>,
}

impl SDDC_Drive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            average_latency: None,
            firmware_version: None,
            id: None,
            is_indication_enabled: None,
            location: None,
            manufacturer: None,
            model: None,
            power_on_hours: None,
            serial_number: None,
            server: None,
            size: None,
            size_used: None,
            status: Vec::new(),
            status_category: None,
            storage_pool: None,
            temperature_in_celsius: None,
            total_iops: None,
            total_throughput: None,
            type: None,
            used_for: None,
            wear_percentage: None,
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

    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsIndicationEnabled
    pub fn set_is_indication_enabled(&mut self, value: bool) {
        self.is_indication_enabled = Some(value);
    }

    /// Gets the value of IsIndicationEnabled
    pub fn get_is_indication_enabled(&self) -> Option<&bool> {
        self.is_indication_enabled.as_ref()
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

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of PowerOnHours
    pub fn set_power_on_hours(&mut self, value: u32) {
        self.power_on_hours = Some(value);
    }

    /// Gets the value of PowerOnHours
    pub fn get_power_on_hours(&self) -> Option<&u32> {
        self.power_on_hours.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of Server
    pub fn set_server(&mut self, value: String) {
        self.server = Some(value);
    }

    /// Gets the value of Server
    pub fn get_server(&self) -> Option<&String> {
        self.server.as_ref()
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

    /// Sets the value of StoragePool
    pub fn set_storage_pool(&mut self, value: String) {
        self.storage_pool = Some(value);
    }

    /// Gets the value of StoragePool
    pub fn get_storage_pool(&self) -> Option<&String> {
        self.storage_pool.as_ref()
    }

    /// Sets the value of TemperatureInCelsius
    pub fn set_temperature_in_celsius(&mut self, value: u8) {
        self.temperature_in_celsius = Some(value);
    }

    /// Gets the value of TemperatureInCelsius
    pub fn get_temperature_in_celsius(&self) -> Option<&u8> {
        self.temperature_in_celsius.as_ref()
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

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

    /// Sets the value of UsedFor
    pub fn set_used_for(&mut self, value: u16) {
        self.used_for = Some(value);
    }

    /// Gets the value of UsedFor
    pub fn get_used_for(&self) -> Option<&u16> {
        self.used_for.as_ref()
    }

    /// Sets the value of WearPercentage
    pub fn set_wear_percentage(&mut self, value: u8) {
        self.wear_percentage = Some(value);
    }

    /// Gets the value of WearPercentage
    pub fn get_wear_percentage(&self) -> Option<&u8> {
        self.wear_percentage.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn retire(&self) -> Result<(), WmiError> {
        self.invoke_method("Retire", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn unretire(&self) -> Result<(), WmiError> {
        self.invoke_method("Unretire", &[])

    }


/// 

    /// * `image_path` -  (String)
    /// * `slot_number` -  (u16)

    /// * `return_value` -  (u32)
    pub fn update_firmware(&self, image_path: &String, slot_number: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ImagePath".to_string(), value: image_path.into() });
        args.push(MethodParameter { name: "SlotNumber".to_string(), value: slot_number.into() });
        self.invoke_method("UpdateFirmware", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn turn_on_light(&self) -> Result<(), WmiError> {
        self.invoke_method("TurnOnLight", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn turn_off_light(&self) -> Result<(), WmiError> {
        self.invoke_method("TurnOffLight", &[])

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

