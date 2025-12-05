// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SyntheticMouse struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SyntheticMouse {
    #[serde(flatten)]
    pub base: CIM_PointingDevice,

/// 
    #[serde(rename = "AbsoluteCoordinates")]
    pub absolute_coordinates: Option<bool>,

/// 
    #[serde(rename = "HorizontalPosition")]
    pub horizontal_position: Option<i32>,

/// 
    #[serde(rename = "ScrollPosition")]
    pub scroll_position: Option<i32>,

/// 
    #[serde(rename = "VerticalPosition")]
    pub vertical_position: Option<i32>,
}

impl Msvm_SyntheticMouse {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PointingDevice::new(),
            absolute_coordinates: None,
            horizontal_position: None,
            scroll_position: None,
            vertical_position: None,
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

    /// Sets the value of HorizontalPosition
    pub fn set_horizontal_position(&mut self, value: i32) {
        self.horizontal_position = Some(value);
    }

    /// Gets the value of HorizontalPosition
    pub fn get_horizontal_position(&self) -> Option<&i32> {
        self.horizontal_position.as_ref()
    }

    /// Sets the value of ScrollPosition
    pub fn set_scroll_position(&mut self, value: i32) {
        self.scroll_position = Some(value);
    }

    /// Gets the value of ScrollPosition
    pub fn get_scroll_position(&self) -> Option<&i32> {
        self.scroll_position.as_ref()
    }

    /// Sets the value of VerticalPosition
    pub fn set_vertical_position(&mut self, value: i32) {
        self.vertical_position = Some(value);
    }

    /// Gets the value of VerticalPosition
    pub fn get_vertical_position(&self) -> Option<&i32> {
        self.vertical_position.as_ref()
    }

/// 

    /// * `button_index` -  (u32)

    /// * `is_down` -  (bool)
    /// * `return_value` -  (u32)
    pub fn get_button_state(&self, button_index: u32, is_down: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ButtonIndex".to_string(), value: button_index.into() });

        let result = self.invoke_method("GetButtonState", &args)?;
        let is_down = result.get_value("IsDown")?;
        Ok(result.return_value)

    }


/// 

    /// * `button_index` -  (u32)
    /// * `is_down` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_button_state(&self, button_index: u32, is_down: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ButtonIndex".to_string(), value: button_index.into() });
        args.push(MethodParameter { name: "IsDown".to_string(), value: is_down.into() });
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

    /// * `horizontal_position` -  (i32)
    /// * `vertical_position` -  (i32)

    /// * `return_value` -  (u32)
    pub fn set_absolute_position(&self, horizontal_position: i32, vertical_position: i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HorizontalPosition".to_string(), value: horizontal_position.into() });
        args.push(MethodParameter { name: "VerticalPosition".to_string(), value: vertical_position.into() });
        self.invoke_method("SetAbsolutePosition", &args)

    }


/// 

    /// * `scroll_position_delta` -  (i32)

    /// * `return_value` -  (u32)
    pub fn set_scroll_position(&self, scroll_position_delta: i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScrollPositionDelta".to_string(), value: scroll_position_delta.into() });
        self.invoke_method("SetScrollPosition", &args)

    }

}

