// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AffectedJobElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AffectedJobElement {

/// The ManagedElement affected by the execution of the Job.
    #[serde(rename = "AffectedElement")]
    pub affected_element: Option<CIM_ManagedElement>,

/// The Job that is affecting the ManagedElement.
    #[serde(rename = "AffectingElement")]
    pub affecting_element: Option<CIM_Job>,

/// An enumeration describing the 'effect' on the ManagedElement. This array corresponds to the OtherElementEffectsDescriptions array, where the latter provides details related to the high-level 'effects' enumerated by this property. Additional detail is required if the ElementEffects array contains the value 1, "Other".
    #[serde(rename = "ElementEffects")]
    pub element_effects: Vec<AffectedJobElement_ElementEffects>,

/// Provides details for the 'effect' at the corresponding array position in ElementEffects. This information is required whenever ElementEffects contains the value 1 ("Other").
    #[serde(rename = "OtherElementEffectsDescriptions")]
    pub other_element_effects_descriptions: Vec<String>,
}

impl CIM_AffectedJobElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            affected_element: None,
            affecting_element: None,
            element_effects: Vec::new(),
            other_element_effects_descriptions: Vec::new(),
        }
    }


    /// Sets the value of AffectedElement
    pub fn set_affected_element(&mut self, value: CIM_ManagedElement) {
        self.affected_element = Some(value);
    }

    /// Gets the value of AffectedElement
    pub fn get_affected_element(&self) -> Option<&CIM_ManagedElement> {
        self.affected_element.as_ref()
    }

    /// Sets the value of AffectingElement
    pub fn set_affecting_element(&mut self, value: CIM_Job) {
        self.affecting_element = Some(value);
    }

    /// Gets the value of AffectingElement
    pub fn get_affecting_element(&self) -> Option<&CIM_Job> {
        self.affecting_element.as_ref()
    }

    /// Sets the value of ElementEffects
    pub fn set_element_effects(&mut self, value: Vec<AffectedJobElement_ElementEffects>) {
        self.element_effects = value;
    }

    /// Gets the value of ElementEffects
    pub fn get_element_effects(&self) -> &Vec<AffectedJobElement_ElementEffects> {
        &self.element_effects
    }

    /// Sets the value of OtherElementEffectsDescriptions
    pub fn set_other_element_effects_descriptions(&mut self, value: Vec<String>) {
        self.other_element_effects_descriptions = value;
    }

    /// Gets the value of OtherElementEffectsDescriptions
    pub fn get_other_element_effects_descriptions(&self) -> &Vec<String> {
        &self.other_element_effects_descriptions
    }
}

