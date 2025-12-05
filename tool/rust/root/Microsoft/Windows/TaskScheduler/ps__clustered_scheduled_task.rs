// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_ClusteredScheduledTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_ClusteredScheduledTask {
}

impl PS_ClusteredScheduledTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 99

    /// * `cluster` -  (String)
    /// * `input_object` - 24 (MSFT_ScheduledTask)
    /// * `resource` -  (String)
    /// * `task_name` - 100 (String)
    /// * `task_type` -  (i32)

    /// * `cmdlet_output` - 101 (MSFT_ClusteredScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_object(&self, input_object: MSFT_ScheduledTask, cluster: &String, task_name: &String, task_type: i32, resource: &String, cmdlet_output: &mut MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "TaskType".to_string(), value: task_type.into() });
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });

        let result = self.invoke_method("RegisterByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 99

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `cluster` -  (String)
    /// * `description` - 32 (String)
    /// * `resource` -  (String)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `task_name` - 100 (String)
    /// * `task_type` -  (i32)
    /// * `trigger` - 86 (MSFT_TaskTrigger[])

    /// * `cmdlet_output` - 101 (MSFT_ClusteredScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_params(&self, task_name: &String, settings: MSFT_TaskSettings, description: &String, trigger: &Vec<MSFT_TaskTrigger>, cluster: &String, action: &Vec<MSFT_TaskAction>, task_type: i32, resource: &String, cmdlet_output: &mut MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "TaskType".to_string(), value: task_type.into() });
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });

        let result = self.invoke_method("RegisterByParams", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 99

    /// * `cluster` -  (String)
    /// * `resource` -  (String)
    /// * `task_name` - 100 (String)
    /// * `task_type` -  (i32)
    /// * `xml` - 35 (String)

    /// * `cmdlet_output` - 101 (MSFT_ClusteredScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_xml(&self, xml: &String, task_name: &String, cluster: &String, task_type: i32, resource: &String, cmdlet_output: &mut MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Xml".to_string(), value: xml.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "TaskType".to_string(), value: task_type.into() });
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });

        let result = self.invoke_method("RegisterByXml", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 102

    /// * `cluster` -  (String)
    /// * `task_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn unregister_by_name(&self, cluster: &String, task_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        self.invoke_method("UnregisterByName", &args)

    }


/// 102

    /// * `input_object` -  (MSFT_ClusteredScheduledTask)

    /// * `return_value` -  (u32)
    pub fn unregister_by_object(&self, input_object: MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("UnregisterByObject", &args)

    }


/// 103

    /// * `cluster` - 105 (String)
    /// * `task_name` - 104 (String)
    /// * `task_type` -  (i32)

    /// * `cmdlet_output` - 106 (MSFT_ClusteredScheduledTask[])
    /// * `return_value` -  (u32)
    pub fn get(&self, task_name: &String, cluster: &String, task_type: i32, cmdlet_output: &mut Vec<MSFT_ClusteredScheduledTask>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "TaskType".to_string(), value: task_type.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 107

    /// * `cluster` -  (String)
    /// * `input_object` - 24 (MSFT_ScheduledTask)
    /// * `task_name` -  (String)

    /// * `cmdlet_output` -  (MSFT_ClusteredScheduledTask)
    /// * `return_value` -  (u32)
    pub fn set_by_object(&self, task_name: &String, cluster: &String, input_object: MSFT_ScheduledTask, cmdlet_output: &mut MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("SetByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 107

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `cluster` -  (String)
    /// * `description` - 32 (String)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `task_name` -  (String)
    /// * `trigger` - 86 (MSFT_TaskTrigger[])

    /// * `cmdlet_output` -  (MSFT_ClusteredScheduledTask)
    /// * `return_value` -  (u32)
    pub fn set_by_params(&self, action: &Vec<MSFT_TaskAction>, settings: MSFT_TaskSettings, cluster: &String, task_name: &String, trigger: &Vec<MSFT_TaskTrigger>, description: &String, cmdlet_output: &mut MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("SetByParams", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 107

    /// * `cluster` -  (String)
    /// * `task_name` -  (String)
    /// * `xml` - 35 (String)

    /// * `cmdlet_output` -  (MSFT_ClusteredScheduledTask)
    /// * `return_value` -  (u32)
    pub fn set_by_xml(&self, xml: &String, cluster: &String, task_name: &String, cmdlet_output: &mut MSFT_ClusteredScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Xml".to_string(), value: xml.into() });
        args.push(MethodParameter { name: "Cluster".to_string(), value: cluster.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });

        let result = self.invoke_method("SetByXml", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

