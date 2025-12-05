// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VideoModeDescriptor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VideoModeDescriptor {

/// 
    #[serde(rename = "CompositePolarityType")]
    pub composite_polarity_type: Option<u8>,

/// 
    #[serde(rename = "HorizontalActivePixels")]
    pub horizontal_active_pixels: Option<u16>,

/// 
    #[serde(rename = "HorizontalBlankingPixels")]
    pub horizontal_blanking_pixels: Option<u16>,

/// 
    #[serde(rename = "HorizontalBorder")]
    pub horizontal_border: Option<u16>,

/// 
    #[serde(rename = "HorizontalImageSize")]
    pub horizontal_image_size: Option<u16>,

/// 
    #[serde(rename = "HorizontalPolarityType")]
    pub horizontal_polarity_type: Option<u8>,

/// 
    #[serde(rename = "HorizontalRefreshRateDenominator")]
    pub horizontal_refresh_rate_denominator: Option<u32>,

/// 
    #[serde(rename = "HorizontalRefreshRateNumerator")]
    pub horizontal_refresh_rate_numerator: Option<u32>,

/// 
    #[serde(rename = "HorizontalSyncOffset")]
    pub horizontal_sync_offset: Option<u16>,

/// 
    #[serde(rename = "HorizontalSyncPulseWidth")]
    pub horizontal_sync_pulse_width: Option<u16>,

/// 
    #[serde(rename = "IsInterlaced")]
    pub is_interlaced: Option<bool>,

/// 
    #[serde(rename = "IsSerrationRequired")]
    pub is_serration_required: Option<u8>,

/// 
    #[serde(rename = "IsSyncOnRGB")]
    pub is_sync_on_rgb: Option<u8>,

/// 
    #[serde(rename = "Origin")]
    pub origin: Option<u8>,

/// 
    #[serde(rename = "PixelClockRate")]
    pub pixel_clock_rate: Option<u32>,

/// 
    #[serde(rename = "StereoModeType")]
    pub stereo_mode_type: Option<u8>,

/// 
    #[serde(rename = "SyncSignalType")]
    pub sync_signal_type: Option<u8>,

/// 
    #[serde(rename = "TimingType")]
    pub timing_type: Option<u8>,

/// 
    #[serde(rename = "VerticalActivePixels")]
    pub vertical_active_pixels: Option<u16>,

/// 
    #[serde(rename = "VerticalBlankingPixels")]
    pub vertical_blanking_pixels: Option<u16>,

/// 
    #[serde(rename = "VerticalBorder")]
    pub vertical_border: Option<u16>,

/// 
    #[serde(rename = "VerticalImageSize")]
    pub vertical_image_size: Option<u16>,

/// 
    #[serde(rename = "VerticalPolarityType")]
    pub vertical_polarity_type: Option<u8>,

/// 
    #[serde(rename = "VerticalRefreshRateDenominator")]
    pub vertical_refresh_rate_denominator: Option<u32>,

/// 
    #[serde(rename = "VerticalRefreshRateNumerator")]
    pub vertical_refresh_rate_numerator: Option<u32>,

/// 
    #[serde(rename = "VerticalSyncOffset")]
    pub vertical_sync_offset: Option<u16>,

/// 
    #[serde(rename = "VerticalSyncPulseWidth")]
    pub vertical_sync_pulse_width: Option<u16>,

/// 
    #[serde(rename = "VideoStandardType")]
    pub video_standard_type: Option<u8>,
}

impl VideoModeDescriptor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            composite_polarity_type: None,
            horizontal_active_pixels: None,
            horizontal_blanking_pixels: None,
            horizontal_border: None,
            horizontal_image_size: None,
            horizontal_polarity_type: None,
            horizontal_refresh_rate_denominator: None,
            horizontal_refresh_rate_numerator: None,
            horizontal_sync_offset: None,
            horizontal_sync_pulse_width: None,
            is_interlaced: None,
            is_serration_required: None,
            is_sync_on_rgb: None,
            origin: None,
            pixel_clock_rate: None,
            stereo_mode_type: None,
            sync_signal_type: None,
            timing_type: None,
            vertical_active_pixels: None,
            vertical_blanking_pixels: None,
            vertical_border: None,
            vertical_image_size: None,
            vertical_polarity_type: None,
            vertical_refresh_rate_denominator: None,
            vertical_refresh_rate_numerator: None,
            vertical_sync_offset: None,
            vertical_sync_pulse_width: None,
            video_standard_type: None,
        }
    }


    /// Sets the value of CompositePolarityType
    pub fn set_composite_polarity_type(&mut self, value: u8) {
        self.composite_polarity_type = Some(value);
    }

    /// Gets the value of CompositePolarityType
    pub fn get_composite_polarity_type(&self) -> Option<&u8> {
        self.composite_polarity_type.as_ref()
    }

    /// Sets the value of HorizontalActivePixels
    pub fn set_horizontal_active_pixels(&mut self, value: u16) {
        self.horizontal_active_pixels = Some(value);
    }

    /// Gets the value of HorizontalActivePixels
    pub fn get_horizontal_active_pixels(&self) -> Option<&u16> {
        self.horizontal_active_pixels.as_ref()
    }

    /// Sets the value of HorizontalBlankingPixels
    pub fn set_horizontal_blanking_pixels(&mut self, value: u16) {
        self.horizontal_blanking_pixels = Some(value);
    }

    /// Gets the value of HorizontalBlankingPixels
    pub fn get_horizontal_blanking_pixels(&self) -> Option<&u16> {
        self.horizontal_blanking_pixels.as_ref()
    }

    /// Sets the value of HorizontalBorder
    pub fn set_horizontal_border(&mut self, value: u16) {
        self.horizontal_border = Some(value);
    }

    /// Gets the value of HorizontalBorder
    pub fn get_horizontal_border(&self) -> Option<&u16> {
        self.horizontal_border.as_ref()
    }

    /// Sets the value of HorizontalImageSize
    pub fn set_horizontal_image_size(&mut self, value: u16) {
        self.horizontal_image_size = Some(value);
    }

    /// Gets the value of HorizontalImageSize
    pub fn get_horizontal_image_size(&self) -> Option<&u16> {
        self.horizontal_image_size.as_ref()
    }

    /// Sets the value of HorizontalPolarityType
    pub fn set_horizontal_polarity_type(&mut self, value: u8) {
        self.horizontal_polarity_type = Some(value);
    }

    /// Gets the value of HorizontalPolarityType
    pub fn get_horizontal_polarity_type(&self) -> Option<&u8> {
        self.horizontal_polarity_type.as_ref()
    }

    /// Sets the value of HorizontalRefreshRateDenominator
    pub fn set_horizontal_refresh_rate_denominator(&mut self, value: u32) {
        self.horizontal_refresh_rate_denominator = Some(value);
    }

    /// Gets the value of HorizontalRefreshRateDenominator
    pub fn get_horizontal_refresh_rate_denominator(&self) -> Option<&u32> {
        self.horizontal_refresh_rate_denominator.as_ref()
    }

    /// Sets the value of HorizontalRefreshRateNumerator
    pub fn set_horizontal_refresh_rate_numerator(&mut self, value: u32) {
        self.horizontal_refresh_rate_numerator = Some(value);
    }

    /// Gets the value of HorizontalRefreshRateNumerator
    pub fn get_horizontal_refresh_rate_numerator(&self) -> Option<&u32> {
        self.horizontal_refresh_rate_numerator.as_ref()
    }

    /// Sets the value of HorizontalSyncOffset
    pub fn set_horizontal_sync_offset(&mut self, value: u16) {
        self.horizontal_sync_offset = Some(value);
    }

    /// Gets the value of HorizontalSyncOffset
    pub fn get_horizontal_sync_offset(&self) -> Option<&u16> {
        self.horizontal_sync_offset.as_ref()
    }

    /// Sets the value of HorizontalSyncPulseWidth
    pub fn set_horizontal_sync_pulse_width(&mut self, value: u16) {
        self.horizontal_sync_pulse_width = Some(value);
    }

    /// Gets the value of HorizontalSyncPulseWidth
    pub fn get_horizontal_sync_pulse_width(&self) -> Option<&u16> {
        self.horizontal_sync_pulse_width.as_ref()
    }

    /// Sets the value of IsInterlaced
    pub fn set_is_interlaced(&mut self, value: bool) {
        self.is_interlaced = Some(value);
    }

    /// Gets the value of IsInterlaced
    pub fn get_is_interlaced(&self) -> Option<&bool> {
        self.is_interlaced.as_ref()
    }

    /// Sets the value of IsSerrationRequired
    pub fn set_is_serration_required(&mut self, value: u8) {
        self.is_serration_required = Some(value);
    }

    /// Gets the value of IsSerrationRequired
    pub fn get_is_serration_required(&self) -> Option<&u8> {
        self.is_serration_required.as_ref()
    }

    /// Sets the value of IsSyncOnRGB
    pub fn set_is_sync_on_rgb(&mut self, value: u8) {
        self.is_sync_on_rgb = Some(value);
    }

    /// Gets the value of IsSyncOnRGB
    pub fn get_is_sync_on_rgb(&self) -> Option<&u8> {
        self.is_sync_on_rgb.as_ref()
    }

    /// Sets the value of Origin
    pub fn set_origin(&mut self, value: u8) {
        self.origin = Some(value);
    }

    /// Gets the value of Origin
    pub fn get_origin(&self) -> Option<&u8> {
        self.origin.as_ref()
    }

    /// Sets the value of PixelClockRate
    pub fn set_pixel_clock_rate(&mut self, value: u32) {
        self.pixel_clock_rate = Some(value);
    }

    /// Gets the value of PixelClockRate
    pub fn get_pixel_clock_rate(&self) -> Option<&u32> {
        self.pixel_clock_rate.as_ref()
    }

    /// Sets the value of StereoModeType
    pub fn set_stereo_mode_type(&mut self, value: u8) {
        self.stereo_mode_type = Some(value);
    }

    /// Gets the value of StereoModeType
    pub fn get_stereo_mode_type(&self) -> Option<&u8> {
        self.stereo_mode_type.as_ref()
    }

    /// Sets the value of SyncSignalType
    pub fn set_sync_signal_type(&mut self, value: u8) {
        self.sync_signal_type = Some(value);
    }

    /// Gets the value of SyncSignalType
    pub fn get_sync_signal_type(&self) -> Option<&u8> {
        self.sync_signal_type.as_ref()
    }

    /// Sets the value of TimingType
    pub fn set_timing_type(&mut self, value: u8) {
        self.timing_type = Some(value);
    }

    /// Gets the value of TimingType
    pub fn get_timing_type(&self) -> Option<&u8> {
        self.timing_type.as_ref()
    }

    /// Sets the value of VerticalActivePixels
    pub fn set_vertical_active_pixels(&mut self, value: u16) {
        self.vertical_active_pixels = Some(value);
    }

    /// Gets the value of VerticalActivePixels
    pub fn get_vertical_active_pixels(&self) -> Option<&u16> {
        self.vertical_active_pixels.as_ref()
    }

    /// Sets the value of VerticalBlankingPixels
    pub fn set_vertical_blanking_pixels(&mut self, value: u16) {
        self.vertical_blanking_pixels = Some(value);
    }

    /// Gets the value of VerticalBlankingPixels
    pub fn get_vertical_blanking_pixels(&self) -> Option<&u16> {
        self.vertical_blanking_pixels.as_ref()
    }

    /// Sets the value of VerticalBorder
    pub fn set_vertical_border(&mut self, value: u16) {
        self.vertical_border = Some(value);
    }

    /// Gets the value of VerticalBorder
    pub fn get_vertical_border(&self) -> Option<&u16> {
        self.vertical_border.as_ref()
    }

    /// Sets the value of VerticalImageSize
    pub fn set_vertical_image_size(&mut self, value: u16) {
        self.vertical_image_size = Some(value);
    }

    /// Gets the value of VerticalImageSize
    pub fn get_vertical_image_size(&self) -> Option<&u16> {
        self.vertical_image_size.as_ref()
    }

    /// Sets the value of VerticalPolarityType
    pub fn set_vertical_polarity_type(&mut self, value: u8) {
        self.vertical_polarity_type = Some(value);
    }

    /// Gets the value of VerticalPolarityType
    pub fn get_vertical_polarity_type(&self) -> Option<&u8> {
        self.vertical_polarity_type.as_ref()
    }

    /// Sets the value of VerticalRefreshRateDenominator
    pub fn set_vertical_refresh_rate_denominator(&mut self, value: u32) {
        self.vertical_refresh_rate_denominator = Some(value);
    }

    /// Gets the value of VerticalRefreshRateDenominator
    pub fn get_vertical_refresh_rate_denominator(&self) -> Option<&u32> {
        self.vertical_refresh_rate_denominator.as_ref()
    }

    /// Sets the value of VerticalRefreshRateNumerator
    pub fn set_vertical_refresh_rate_numerator(&mut self, value: u32) {
        self.vertical_refresh_rate_numerator = Some(value);
    }

    /// Gets the value of VerticalRefreshRateNumerator
    pub fn get_vertical_refresh_rate_numerator(&self) -> Option<&u32> {
        self.vertical_refresh_rate_numerator.as_ref()
    }

    /// Sets the value of VerticalSyncOffset
    pub fn set_vertical_sync_offset(&mut self, value: u16) {
        self.vertical_sync_offset = Some(value);
    }

    /// Gets the value of VerticalSyncOffset
    pub fn get_vertical_sync_offset(&self) -> Option<&u16> {
        self.vertical_sync_offset.as_ref()
    }

    /// Sets the value of VerticalSyncPulseWidth
    pub fn set_vertical_sync_pulse_width(&mut self, value: u16) {
        self.vertical_sync_pulse_width = Some(value);
    }

    /// Gets the value of VerticalSyncPulseWidth
    pub fn get_vertical_sync_pulse_width(&self) -> Option<&u16> {
        self.vertical_sync_pulse_width.as_ref()
    }

    /// Sets the value of VideoStandardType
    pub fn set_video_standard_type(&mut self, value: u8) {
        self.video_standard_type = Some(value);
    }

    /// Gets the value of VideoStandardType
    pub fn get_video_standard_type(&self) -> Option<&u8> {
        self.video_standard_type.as_ref()
    }
}

