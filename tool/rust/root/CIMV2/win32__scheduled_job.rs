// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ScheduledJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ScheduledJob {
    #[serde(flatten)]
    pub base: CIM_Job,

/// 
    #[serde(rename = "Command")]
    pub command: Option<String>,

/// 
    #[serde(rename = "DaysOfMonth")]
    pub days_of_month: Option<u32>,

/// 
    #[serde(rename = "DaysOfWeek")]
    pub days_of_week: Option<u32>,

/// 
    #[serde(rename = "InteractWithDesktop")]
    pub interact_with_desktop: Option<bool>,

/// 
    #[serde(rename = "JobId")]
    pub job_id: Option<u32>,

/// 
    #[serde(rename = "RunRepeatedly")]
    pub run_repeatedly: Option<bool>,
}

impl Win32_ScheduledJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Job::new(),
            command: None,
            days_of_month: None,
            days_of_week: None,
            interact_with_desktop: None,
            job_id: None,
            run_repeatedly: None,
        }
    }


    /// Sets the value of Command
    pub fn set_command(&mut self, value: String) {
        self.command = Some(value);
    }

    /// Gets the value of Command
    pub fn get_command(&self) -> Option<&String> {
        self.command.as_ref()
    }

    /// Sets the value of DaysOfMonth
    pub fn set_days_of_month(&mut self, value: u32) {
        self.days_of_month = Some(value);
    }

    /// Gets the value of DaysOfMonth
    pub fn get_days_of_month(&self) -> Option<&u32> {
        self.days_of_month.as_ref()
    }

    /// Sets the value of DaysOfWeek
    pub fn set_days_of_week(&mut self, value: u32) {
        self.days_of_week = Some(value);
    }

    /// Gets the value of DaysOfWeek
    pub fn get_days_of_week(&self) -> Option<&u32> {
        self.days_of_week.as_ref()
    }

    /// Sets the value of InteractWithDesktop
    pub fn set_interact_with_desktop(&mut self, value: bool) {
        self.interact_with_desktop = Some(value);
    }

    /// Gets the value of InteractWithDesktop
    pub fn get_interact_with_desktop(&self) -> Option<&bool> {
        self.interact_with_desktop.as_ref()
    }

    /// Sets the value of JobId
    pub fn set_job_id(&mut self, value: u32) {
        self.job_id = Some(value);
    }

    /// Gets the value of JobId
    pub fn get_job_id(&self) -> Option<&u32> {
        self.job_id.as_ref()
    }

    /// Sets the value of RunRepeatedly
    pub fn set_run_repeatedly(&mut self, value: bool) {
        self.run_repeatedly = Some(value);
    }

    /// Gets the value of RunRepeatedly
    pub fn get_run_repeatedly(&self) -> Option<&bool> {
        self.run_repeatedly.as_ref()
    }

/// 

    /// * `command` -  (String)
    /// * `days_of_month` -  (u32)
    /// * `days_of_week` -  (u32)
    /// * `interact_with_desktop` -  (bool)
    /// * `run_repeatedly` -  (bool)
    /// * `start_time` -  (String)

    /// * `job_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn create(&self, command: &String, start_time: &String, run_repeatedly: Option<bool>, days_of_week: Option<u32>, days_of_month: Option<u32>, interact_with_desktop: Option<bool>, job_id: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Command".to_string(), value: command.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });
        if let Some(val) = run_repeatedly {
            args.push(MethodParameter { name: "RunRepeatedly".to_string(), value: val.into() });
        }
        if let Some(val) = days_of_week {
            args.push(MethodParameter { name: "DaysOfWeek".to_string(), value: val.into() });
        }
        if let Some(val) = days_of_month {
            args.push(MethodParameter { name: "DaysOfMonth".to_string(), value: val.into() });
        }
        if let Some(val) = interact_with_desktop {
            args.push(MethodParameter { name: "InteractWithDesktop".to_string(), value: val.into() });
        }

        let result = self.invoke_method("Create", &args)?;
        let job_id = result.get_value("JobId")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn delete(&self) -> Result<(), WmiError> {
        self.invoke_method("Delete", &[])

    }

}

