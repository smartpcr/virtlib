// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FrequencyRangeDescriptor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrequencyRangeDescriptor {

/// 
    #[serde(rename = "ActiveHeight")]
    pub active_height: Option<u32>,

/// 
    #[serde(rename = "ActiveWidth")]
    pub active_width: Option<u32>,

/// 
    #[serde(rename = "ConstraintType")]
    pub constraint_type: Option<u32>,

/// 
    #[serde(rename = "MaxHSyncDenominator")]
    pub max_hsync_denominator: Option<u32>,

/// 
    #[serde(rename = "MaxHSyncNumerator")]
    pub max_hsync_numerator: Option<u32>,

/// 
    #[serde(rename = "MaxPixelRate")]
    pub max_pixel_rate: Option<u32>,

/// 
    #[serde(rename = "MaxVSyncDenominator")]
    pub max_vsync_denominator: Option<u32>,

/// 
    #[serde(rename = "MaxVSyncNumerator")]
    pub max_vsync_numerator: Option<u32>,

/// 
    #[serde(rename = "MinHSyncDenominator")]
    pub min_hsync_denominator: Option<u32>,

/// 
    #[serde(rename = "MinHSyncNumerator")]
    pub min_hsync_numerator: Option<u32>,

/// 
    #[serde(rename = "MinVSyncDenominator")]
    pub min_vsync_denominator: Option<u32>,

/// 
    #[serde(rename = "MinVSyncNumerator")]
    pub min_vsync_numerator: Option<u32>,

/// 
    #[serde(rename = "Origin")]
    pub origin: Option<u8>,
}

impl FrequencyRangeDescriptor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active_height: None,
            active_width: None,
            constraint_type: None,
            max_hsync_denominator: None,
            max_hsync_numerator: None,
            max_pixel_rate: None,
            max_vsync_denominator: None,
            max_vsync_numerator: None,
            min_hsync_denominator: None,
            min_hsync_numerator: None,
            min_vsync_denominator: None,
            min_vsync_numerator: None,
            origin: None,
        }
    }


    /// Sets the value of ActiveHeight
    pub fn set_active_height(&mut self, value: u32) {
        self.active_height = Some(value);
    }

    /// Gets the value of ActiveHeight
    pub fn get_active_height(&self) -> Option<&u32> {
        self.active_height.as_ref()
    }

    /// Sets the value of ActiveWidth
    pub fn set_active_width(&mut self, value: u32) {
        self.active_width = Some(value);
    }

    /// Gets the value of ActiveWidth
    pub fn get_active_width(&self) -> Option<&u32> {
        self.active_width.as_ref()
    }

    /// Sets the value of ConstraintType
    pub fn set_constraint_type(&mut self, value: u32) {
        self.constraint_type = Some(value);
    }

    /// Gets the value of ConstraintType
    pub fn get_constraint_type(&self) -> Option<&u32> {
        self.constraint_type.as_ref()
    }

    /// Sets the value of MaxHSyncDenominator
    pub fn set_max_hsync_denominator(&mut self, value: u32) {
        self.max_hsync_denominator = Some(value);
    }

    /// Gets the value of MaxHSyncDenominator
    pub fn get_max_hsync_denominator(&self) -> Option<&u32> {
        self.max_hsync_denominator.as_ref()
    }

    /// Sets the value of MaxHSyncNumerator
    pub fn set_max_hsync_numerator(&mut self, value: u32) {
        self.max_hsync_numerator = Some(value);
    }

    /// Gets the value of MaxHSyncNumerator
    pub fn get_max_hsync_numerator(&self) -> Option<&u32> {
        self.max_hsync_numerator.as_ref()
    }

    /// Sets the value of MaxPixelRate
    pub fn set_max_pixel_rate(&mut self, value: u32) {
        self.max_pixel_rate = Some(value);
    }

    /// Gets the value of MaxPixelRate
    pub fn get_max_pixel_rate(&self) -> Option<&u32> {
        self.max_pixel_rate.as_ref()
    }

    /// Sets the value of MaxVSyncDenominator
    pub fn set_max_vsync_denominator(&mut self, value: u32) {
        self.max_vsync_denominator = Some(value);
    }

    /// Gets the value of MaxVSyncDenominator
    pub fn get_max_vsync_denominator(&self) -> Option<&u32> {
        self.max_vsync_denominator.as_ref()
    }

    /// Sets the value of MaxVSyncNumerator
    pub fn set_max_vsync_numerator(&mut self, value: u32) {
        self.max_vsync_numerator = Some(value);
    }

    /// Gets the value of MaxVSyncNumerator
    pub fn get_max_vsync_numerator(&self) -> Option<&u32> {
        self.max_vsync_numerator.as_ref()
    }

    /// Sets the value of MinHSyncDenominator
    pub fn set_min_hsync_denominator(&mut self, value: u32) {
        self.min_hsync_denominator = Some(value);
    }

    /// Gets the value of MinHSyncDenominator
    pub fn get_min_hsync_denominator(&self) -> Option<&u32> {
        self.min_hsync_denominator.as_ref()
    }

    /// Sets the value of MinHSyncNumerator
    pub fn set_min_hsync_numerator(&mut self, value: u32) {
        self.min_hsync_numerator = Some(value);
    }

    /// Gets the value of MinHSyncNumerator
    pub fn get_min_hsync_numerator(&self) -> Option<&u32> {
        self.min_hsync_numerator.as_ref()
    }

    /// Sets the value of MinVSyncDenominator
    pub fn set_min_vsync_denominator(&mut self, value: u32) {
        self.min_vsync_denominator = Some(value);
    }

    /// Gets the value of MinVSyncDenominator
    pub fn get_min_vsync_denominator(&self) -> Option<&u32> {
        self.min_vsync_denominator.as_ref()
    }

    /// Sets the value of MinVSyncNumerator
    pub fn set_min_vsync_numerator(&mut self, value: u32) {
        self.min_vsync_numerator = Some(value);
    }

    /// Gets the value of MinVSyncNumerator
    pub fn get_min_vsync_numerator(&self) -> Option<&u32> {
        self.min_vsync_numerator.as_ref()
    }

    /// Sets the value of Origin
    pub fn set_origin(&mut self, value: u8) {
        self.origin = Some(value);
    }

    /// Gets the value of Origin
    pub fn get_origin(&self) -> Option<&u8> {
        self.origin.as_ref()
    }
}

