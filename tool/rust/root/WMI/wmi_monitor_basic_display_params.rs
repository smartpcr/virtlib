// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorBasicDisplayParams struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorBasicDisplayParams {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DisplayTransferCharacteristic")]
    pub display_transfer_characteristic: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MaxHorizontalImageSize")]
    pub max_horizontal_image_size: Option<u8>,

/// 
    #[serde(rename = "MaxVerticalImageSize")]
    pub max_vertical_image_size: Option<u8>,

/// 
    #[serde(rename = "SupportedDisplayFeatures")]
    pub supported_display_features: Option<WmiMonitorSupportedDisplayFeatures>,

/// 
    #[serde(rename = "VideoInputType")]
    pub video_input_type: Option<u8>,
}

impl WmiMonitorBasicDisplayParams {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            display_transfer_characteristic: None,
            instance_name: None,
            max_horizontal_image_size: None,
            max_vertical_image_size: None,
            supported_display_features: None,
            video_input_type: None,
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

    /// Sets the value of DisplayTransferCharacteristic
    pub fn set_display_transfer_characteristic(&mut self, value: u8) {
        self.display_transfer_characteristic = Some(value);
    }

    /// Gets the value of DisplayTransferCharacteristic
    pub fn get_display_transfer_characteristic(&self) -> Option<&u8> {
        self.display_transfer_characteristic.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MaxHorizontalImageSize
    pub fn set_max_horizontal_image_size(&mut self, value: u8) {
        self.max_horizontal_image_size = Some(value);
    }

    /// Gets the value of MaxHorizontalImageSize
    pub fn get_max_horizontal_image_size(&self) -> Option<&u8> {
        self.max_horizontal_image_size.as_ref()
    }

    /// Sets the value of MaxVerticalImageSize
    pub fn set_max_vertical_image_size(&mut self, value: u8) {
        self.max_vertical_image_size = Some(value);
    }

    /// Gets the value of MaxVerticalImageSize
    pub fn get_max_vertical_image_size(&self) -> Option<&u8> {
        self.max_vertical_image_size.as_ref()
    }

    /// Sets the value of SupportedDisplayFeatures
    pub fn set_supported_display_features(&mut self, value: WmiMonitorSupportedDisplayFeatures) {
        self.supported_display_features = Some(value);
    }

    /// Gets the value of SupportedDisplayFeatures
    pub fn get_supported_display_features(&self) -> Option<&WmiMonitorSupportedDisplayFeatures> {
        self.supported_display_features.as_ref()
    }

    /// Sets the value of VideoInputType
    pub fn set_video_input_type(&mut self, value: u8) {
        self.video_input_type = Some(value);
    }

    /// Gets the value of VideoInputType
    pub fn get_video_input_type(&self) -> Option<&u8> {
        self.video_input_type.as_ref()
    }
}

