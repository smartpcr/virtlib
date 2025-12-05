// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSAcpi_ThermalZoneTemperature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSAcpi_ThermalZoneTemperature {
    #[serde(flatten)]
    pub base: MSAcpi,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "ActiveTripPoint")]
    pub active_trip_point: Vec<u32>,

/// 
    #[serde(rename = "ActiveTripPointCount")]
    pub active_trip_point_count: Option<u32>,

/// 
    #[serde(rename = "CriticalTripPoint")]
    pub critical_trip_point: Option<u32>,

/// 
    #[serde(rename = "CurrentTemperature")]
    pub current_temperature: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PassiveTripPoint")]
    pub passive_trip_point: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// 
    #[serde(rename = "SamplingPeriod")]
    pub sampling_period: Option<u32>,

/// 
    #[serde(rename = "ThermalConstant1")]
    pub thermal_constant1: Option<u32>,

/// 
    #[serde(rename = "ThermalConstant2")]
    pub thermal_constant2: Option<u32>,

/// 
    #[serde(rename = "ThermalStamp")]
    pub thermal_stamp: Option<u32>,
}

impl MSAcpi_ThermalZoneTemperature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSAcpi::new(),
            active: None,
            active_trip_point: Vec::new(),
            active_trip_point_count: None,
            critical_trip_point: None,
            current_temperature: None,
            instance_name: None,
            passive_trip_point: None,
            reserved: None,
            sampling_period: None,
            thermal_constant1: None,
            thermal_constant2: None,
            thermal_stamp: None,
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

    /// Sets the value of ActiveTripPoint
    pub fn set_active_trip_point(&mut self, value: Vec<u32>) {
        self.active_trip_point = value;
    }

    /// Gets the value of ActiveTripPoint
    pub fn get_active_trip_point(&self) -> &Vec<u32> {
        &self.active_trip_point
    }

    /// Sets the value of ActiveTripPointCount
    pub fn set_active_trip_point_count(&mut self, value: u32) {
        self.active_trip_point_count = Some(value);
    }

    /// Gets the value of ActiveTripPointCount
    pub fn get_active_trip_point_count(&self) -> Option<&u32> {
        self.active_trip_point_count.as_ref()
    }

    /// Sets the value of CriticalTripPoint
    pub fn set_critical_trip_point(&mut self, value: u32) {
        self.critical_trip_point = Some(value);
    }

    /// Gets the value of CriticalTripPoint
    pub fn get_critical_trip_point(&self) -> Option<&u32> {
        self.critical_trip_point.as_ref()
    }

    /// Sets the value of CurrentTemperature
    pub fn set_current_temperature(&mut self, value: u32) {
        self.current_temperature = Some(value);
    }

    /// Gets the value of CurrentTemperature
    pub fn get_current_temperature(&self) -> Option<&u32> {
        self.current_temperature.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PassiveTripPoint
    pub fn set_passive_trip_point(&mut self, value: u32) {
        self.passive_trip_point = Some(value);
    }

    /// Gets the value of PassiveTripPoint
    pub fn get_passive_trip_point(&self) -> Option<&u32> {
        self.passive_trip_point.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of SamplingPeriod
    pub fn set_sampling_period(&mut self, value: u32) {
        self.sampling_period = Some(value);
    }

    /// Gets the value of SamplingPeriod
    pub fn get_sampling_period(&self) -> Option<&u32> {
        self.sampling_period.as_ref()
    }

    /// Sets the value of ThermalConstant1
    pub fn set_thermal_constant1(&mut self, value: u32) {
        self.thermal_constant1 = Some(value);
    }

    /// Gets the value of ThermalConstant1
    pub fn get_thermal_constant1(&self) -> Option<&u32> {
        self.thermal_constant1.as_ref()
    }

    /// Sets the value of ThermalConstant2
    pub fn set_thermal_constant2(&mut self, value: u32) {
        self.thermal_constant2 = Some(value);
    }

    /// Gets the value of ThermalConstant2
    pub fn get_thermal_constant2(&self) -> Option<&u32> {
        self.thermal_constant2.as_ref()
    }

    /// Sets the value of ThermalStamp
    pub fn set_thermal_stamp(&mut self, value: u32) {
        self.thermal_stamp = Some(value);
    }

    /// Gets the value of ThermalStamp
    pub fn get_thermal_stamp(&self) -> Option<&u32> {
        self.thermal_stamp.as_ref()
    }
}

