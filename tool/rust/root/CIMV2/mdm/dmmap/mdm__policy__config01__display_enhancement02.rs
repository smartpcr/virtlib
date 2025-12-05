// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_DisplayEnhancement02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_DisplayEnhancement02 {

/// 
    #[serde(rename = "AutobrightnessLuxToNitsCurve")]
    pub autobrightness_lux_to_nits_curve: Option<String>,

/// 
    #[serde(rename = "DefaultAdaptiveColorAdaptationStrength")]
    pub default_adaptive_color_adaptation_strength: Option<i32>,

/// 
    #[serde(rename = "DefaultBatterySaverBrightnessMultiplier")]
    pub default_battery_saver_brightness_multiplier: Option<i32>,

/// 
    #[serde(rename = "DefaultBrightnessSliderPercentage")]
    pub default_brightness_slider_percentage: Option<i32>,

/// 
    #[serde(rename = "DefaultDimBrightnessMultiplier")]
    pub default_dim_brightness_multiplier: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsAdaptiveColorOnByDefault")]
    pub is_adaptive_color_on_by_default: Option<i32>,

/// 
    #[serde(rename = "IsAutobrightnessOnByDefault")]
    pub is_autobrightness_on_by_default: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ShouldStopTransitionDuringHandsOnDisplay")]
    pub should_stop_transition_during_hands_on_display: Option<i32>,
}

impl MDM_Policy_Config01_DisplayEnhancement02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            autobrightness_lux_to_nits_curve: None,
            default_adaptive_color_adaptation_strength: None,
            default_battery_saver_brightness_multiplier: None,
            default_brightness_slider_percentage: None,
            default_dim_brightness_multiplier: None,
            instance_id: None,
            is_adaptive_color_on_by_default: None,
            is_autobrightness_on_by_default: None,
            parent_id: None,
            should_stop_transition_during_hands_on_display: None,
        }
    }


    /// Sets the value of AutobrightnessLuxToNitsCurve
    pub fn set_autobrightness_lux_to_nits_curve(&mut self, value: String) {
        self.autobrightness_lux_to_nits_curve = Some(value);
    }

    /// Gets the value of AutobrightnessLuxToNitsCurve
    pub fn get_autobrightness_lux_to_nits_curve(&self) -> Option<&String> {
        self.autobrightness_lux_to_nits_curve.as_ref()
    }

    /// Sets the value of DefaultAdaptiveColorAdaptationStrength
    pub fn set_default_adaptive_color_adaptation_strength(&mut self, value: i32) {
        self.default_adaptive_color_adaptation_strength = Some(value);
    }

    /// Gets the value of DefaultAdaptiveColorAdaptationStrength
    pub fn get_default_adaptive_color_adaptation_strength(&self) -> Option<&i32> {
        self.default_adaptive_color_adaptation_strength.as_ref()
    }

    /// Sets the value of DefaultBatterySaverBrightnessMultiplier
    pub fn set_default_battery_saver_brightness_multiplier(&mut self, value: i32) {
        self.default_battery_saver_brightness_multiplier = Some(value);
    }

    /// Gets the value of DefaultBatterySaverBrightnessMultiplier
    pub fn get_default_battery_saver_brightness_multiplier(&self) -> Option<&i32> {
        self.default_battery_saver_brightness_multiplier.as_ref()
    }

    /// Sets the value of DefaultBrightnessSliderPercentage
    pub fn set_default_brightness_slider_percentage(&mut self, value: i32) {
        self.default_brightness_slider_percentage = Some(value);
    }

    /// Gets the value of DefaultBrightnessSliderPercentage
    pub fn get_default_brightness_slider_percentage(&self) -> Option<&i32> {
        self.default_brightness_slider_percentage.as_ref()
    }

    /// Sets the value of DefaultDimBrightnessMultiplier
    pub fn set_default_dim_brightness_multiplier(&mut self, value: i32) {
        self.default_dim_brightness_multiplier = Some(value);
    }

    /// Gets the value of DefaultDimBrightnessMultiplier
    pub fn get_default_dim_brightness_multiplier(&self) -> Option<&i32> {
        self.default_dim_brightness_multiplier.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsAdaptiveColorOnByDefault
    pub fn set_is_adaptive_color_on_by_default(&mut self, value: i32) {
        self.is_adaptive_color_on_by_default = Some(value);
    }

    /// Gets the value of IsAdaptiveColorOnByDefault
    pub fn get_is_adaptive_color_on_by_default(&self) -> Option<&i32> {
        self.is_adaptive_color_on_by_default.as_ref()
    }

    /// Sets the value of IsAutobrightnessOnByDefault
    pub fn set_is_autobrightness_on_by_default(&mut self, value: i32) {
        self.is_autobrightness_on_by_default = Some(value);
    }

    /// Gets the value of IsAutobrightnessOnByDefault
    pub fn get_is_autobrightness_on_by_default(&self) -> Option<&i32> {
        self.is_autobrightness_on_by_default.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ShouldStopTransitionDuringHandsOnDisplay
    pub fn set_should_stop_transition_during_hands_on_display(&mut self, value: i32) {
        self.should_stop_transition_during_hands_on_display = Some(value);
    }

    /// Gets the value of ShouldStopTransitionDuringHandsOnDisplay
    pub fn get_should_stop_transition_during_hands_on_display(&self) -> Option<&i32> {
        self.should_stop_transition_during_hands_on_display.as_ref()
    }
}

