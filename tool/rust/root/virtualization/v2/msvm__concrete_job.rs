// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ConcreteJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ConcreteJob {
    #[serde(flatten)]
    pub base: CIM_ConcreteJob,

/// 
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,

/// 
    #[serde(rename = "ErrorSummaryDescription")]
    pub error_summary_description: Option<String>,

/// 
    #[serde(rename = "JobType")]
    pub job_type: Option<ConcreteJob_JobType>,
}

impl Msvm_ConcreteJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ConcreteJob::new(),
            cancellable: None,
            error_summary_description: None,
            job_type: None,
        }
    }


    /// Sets the value of Cancellable
    pub fn set_cancellable(&mut self, value: bool) {
        self.cancellable = Some(value);
    }

    /// Gets the value of Cancellable
    pub fn get_cancellable(&self) -> Option<&bool> {
        self.cancellable.as_ref()
    }

    /// Sets the value of ErrorSummaryDescription
    pub fn set_error_summary_description(&mut self, value: String) {
        self.error_summary_description = Some(value);
    }

    /// Gets the value of ErrorSummaryDescription
    pub fn get_error_summary_description(&self) -> Option<&String> {
        self.error_summary_description.as_ref()
    }

    /// Sets the value of JobType
    pub fn set_job_type(&mut self, value: ConcreteJob_JobType) {
        self.job_type = Some(value);
    }

    /// Gets the value of JobType
    pub fn get_job_type(&self) -> Option<&ConcreteJob_JobType> {
        self.job_type.as_ref()
    }

/// 

    /// * `errors` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_error_ex(&self, errors: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetErrorEx", &[])?;
        let errors = result.get_value("Errors")?;
        Ok(result.return_value)

    }

}

