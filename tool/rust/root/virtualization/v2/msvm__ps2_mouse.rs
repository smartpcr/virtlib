// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Ps2Mouse struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Ps2Mouse {
    #[serde(flatten)]
    pub base: CIM_PointingDevice,

/// 
    #[serde(rename = "AbsoluteCoordinates")]
    pub absolute_coordinates: Option<bool>,
}

impl Msvm_Ps2Mouse {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PointingDevice::new(),
            absolute_coordinates: None,
        }
    }


    /// Sets the value of AbsoluteCoordinates
    pub fn set_absolute_coordinates(&mut self, value: bool) {
        self.absolute_coordinates = Some(value);
    }

    /// Gets the value of AbsoluteCoordinates
    pub fn get_absolute_coordinates(&self) -> Option<&bool> {
        self.absolute_coordinates.as_ref()
    }

/// 

    /// * `button_index` -  (u32)

    /// * `button_state` -  (bool)
    /// * `return_value` -  (u32)
    pub fn get_button_state(&self, button_index: u32, button_state: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ButtonIndex".to_string(), value: button_index.into() });

        let result = self.invoke_method("GetButtonState", &args)?;
        let button_state = result.get_value("ButtonState")?;
        Ok(result.return_value)

    }


/// 

    /// * `button_index` -  (u32)
    /// * `button_state` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_button_state(&self, button_index: u32, button_state: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ButtonIndex".to_string(), value: button_index.into() });
        args.push(MethodParameter { name: "ButtonState".to_string(), value: button_state.into() });
        self.invoke_method("SetButtonState", &args)

    }


/// 

    /// * `button_index` -  (u32)

    /// * `return_value` -  (u32)
    pub fn click_button(&self, button_index: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ButtonIndex".to_string(), value: button_index.into() });
        self.invoke_method("ClickButton", &args)

    }


/// 

    /// * `horizontal_delta` -  (u8)
    /// * `vertical_delta` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_relative_position(&self, horizontal_delta: u8, vertical_delta: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HorizontalDelta".to_string(), value: horizontal_delta.into() });
        args.push(MethodParameter { name: "VerticalDelta".to_string(), value: vertical_delta.into() });
        self.invoke_method("SetRelativePosition", &args)

    }


/// 

    /// * `scroll_position_delta` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_scroll_position(&self, scroll_position_delta: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScrollPositionDelta".to_string(), value: scroll_position_delta.into() });
        self.invoke_method("SetScrollPosition", &args)

    }

}

