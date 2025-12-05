// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalPackage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalPackage {
    #[serde(flatten)]
    pub base: CIM_PhysicalElement,

/// 
    #[serde(rename = "Depth")]
    pub depth: Option<f32>,

/// 
    #[serde(rename = "Height")]
    pub height: Option<f32>,

/// 
    #[serde(rename = "HotSwappable")]
    pub hot_swappable: Option<bool>,

/// 
    #[serde(rename = "Removable")]
    pub removable: Option<bool>,

/// 
    #[serde(rename = "Replaceable")]
    pub replaceable: Option<bool>,

/// 
    #[serde(rename = "Weight")]
    pub weight: Option<f32>,

/// 
    #[serde(rename = "Width")]
    pub width: Option<f32>,
}

impl CIM_PhysicalPackage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalElement::new(),
            depth: None,
            height: None,
            hot_swappable: None,
            removable: None,
            replaceable: None,
            weight: None,
            width: None,
        }
    }


    /// Sets the value of Depth
    pub fn set_depth(&mut self, value: f32) {
        self.depth = Some(value);
    }

    /// Gets the value of Depth
    pub fn get_depth(&self) -> Option<&f32> {
        self.depth.as_ref()
    }

    /// Sets the value of Height
    pub fn set_height(&mut self, value: f32) {
        self.height = Some(value);
    }

    /// Gets the value of Height
    pub fn get_height(&self) -> Option<&f32> {
        self.height.as_ref()
    }

    /// Sets the value of HotSwappable
    pub fn set_hot_swappable(&mut self, value: bool) {
        self.hot_swappable = Some(value);
    }

    /// Gets the value of HotSwappable
    pub fn get_hot_swappable(&self) -> Option<&bool> {
        self.hot_swappable.as_ref()
    }

    /// Sets the value of Removable
    pub fn set_removable(&mut self, value: bool) {
        self.removable = Some(value);
    }

    /// Gets the value of Removable
    pub fn get_removable(&self) -> Option<&bool> {
        self.removable.as_ref()
    }

    /// Sets the value of Replaceable
    pub fn set_replaceable(&mut self, value: bool) {
        self.replaceable = Some(value);
    }

    /// Gets the value of Replaceable
    pub fn get_replaceable(&self) -> Option<&bool> {
        self.replaceable.as_ref()
    }

    /// Sets the value of Weight
    pub fn set_weight(&mut self, value: f32) {
        self.weight = Some(value);
    }

    /// Gets the value of Weight
    pub fn get_weight(&self) -> Option<&f32> {
        self.weight.as_ref()
    }

    /// Sets the value of Width
    pub fn set_width(&mut self, value: f32) {
        self.width = Some(value);
    }

    /// Gets the value of Width
    pub fn get_width(&self) -> Option<&f32> {
        self.width.as_ref()
    }

/// 

    /// * `element_to_check` -  (CIM_PhysicalElement)

    /// * `return_value` -  (u32)
    pub fn is_compatible(&self, element_to_check: CIM_PhysicalElement) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ElementToCheck".to_string(), value: element_to_check.into() });
        self.invoke_method("IsCompatible", &args)

    }

}

