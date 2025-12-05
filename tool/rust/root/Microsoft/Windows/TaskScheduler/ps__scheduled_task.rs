// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_ScheduledTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_ScheduledTask {
}

impl PS_ScheduledTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 22

    /// * `force` - 23 (bool)
    /// * `input_object` - 24 (MSFT_ScheduledTask)
    /// * `password` - 25 (String)
    /// * `task_name` - 27 (String)
    /// * `task_path` - 28 (String)
    /// * `user` - 26 (String)

    /// * `cmdlet_output` - 29 (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_object(&self, force: bool, input_object: MSFT_ScheduledTask, password: &String, user: &String, task_name: &String, task_path: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });

        let result = self.invoke_method("RegisterByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 22

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `description` - 32 (String)
    /// * `force` - 23 (bool)
    /// * `principal` - 30 (MSFT_TaskPrincipal)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `task_name` - 27 (String)
    /// * `task_path` - 28 (String)
    /// * `trigger` - 34 (MSFT_TaskTrigger[])

    /// * `cmdlet_output` - 29 (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_principal(&self, force: bool, principal: MSFT_TaskPrincipal, action: &Vec<MSFT_TaskAction>, description: &String, task_path: &String, settings: MSFT_TaskSettings, trigger: &Vec<MSFT_TaskTrigger>, task_name: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Principal".to_string(), value: principal.into() });
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });

        let result = self.invoke_method("RegisterByPrincipal", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 22

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `description` - 32 (String)
    /// * `force` - 23 (bool)
    /// * `password` - 25 (String)
    /// * `run_level` -  (i32)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `task_name` - 27 (String)
    /// * `task_path` - 28 (String)
    /// * `trigger` - 34 (MSFT_TaskTrigger[])
    /// * `user` - 26 (String)

    /// * `cmdlet_output` - 29 (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_user(&self, force: bool, task_path: &String, trigger: &Vec<MSFT_TaskTrigger>, settings: MSFT_TaskSettings, description: &String, user: &String, password: &String, action: &Vec<MSFT_TaskAction>, run_level: i32, task_name: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "RunLevel".to_string(), value: run_level.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });

        let result = self.invoke_method("RegisterByUser", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 22

    /// * `force` - 23 (bool)
    /// * `password` - 25 (String)
    /// * `task_name` - 27 (String)
    /// * `task_path` - 28 (String)
    /// * `user` - 26 (String)
    /// * `xml` - 35 (String)

    /// * `cmdlet_output` - 29 (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn register_by_xml(&self, force: bool, xml: &String, password: &String, user: &String, task_path: &String, task_name: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Xml".to_string(), value: xml.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });

        let result = self.invoke_method("RegisterByXml", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 36

    /// * `argument` - 39 (String)
    /// * `execute` - 38 (String)
    /// * `id` - 37 (String)
    /// * `working_directory` - 40 (String)

    /// * `cmdlet_output` -  (MSFT_TaskAction)
    /// * `return_value` -  (u32)
    pub fn new_action_by_exec(&self, id: &String, execute: &String, argument: &String, working_directory: &String, cmdlet_output: &mut MSFT_TaskAction) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "Execute".to_string(), value: execute.into() });
        args.push(MethodParameter { name: "Argument".to_string(), value: argument.into() });
        args.push(MethodParameter { name: "WorkingDirectory".to_string(), value: working_directory.into() });

        let result = self.invoke_method("NewActionByExec", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 41

    /// * `group_id` - 42 (String)
    /// * `id` - 43 (String)
    /// * `process_token_sid_type` - 45 (i32)
    /// * `required_privilege` - 46 (String[])
    /// * `run_level` - 44 (i32)

    /// * `cmdlet_output` -  (MSFT_TaskPrincipal)
    /// * `return_value` -  (u32)
    pub fn new_principal_by_group(&self, group_id: &String, id: &String, run_level: i32, process_token_sid_type: i32, required_privilege: &Vec<String>, cmdlet_output: &mut MSFT_TaskPrincipal) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GroupId".to_string(), value: group_id.into() });
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "RunLevel".to_string(), value: run_level.into() });
        args.push(MethodParameter { name: "ProcessTokenSidType".to_string(), value: process_token_sid_type.into() });
        args.push(MethodParameter { name: "RequiredPrivilege".to_string(), value: required_privilege.into() });

        let result = self.invoke_method("NewPrincipalByGroup", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 41

    /// * `id` - 43 (String)
    /// * `logon_type` -  (i32)
    /// * `process_token_sid_type` - 45 (i32)
    /// * `required_privilege` - 46 (String[])
    /// * `run_level` - 44 (i32)
    /// * `user_id` - 47 (String)

    /// * `cmdlet_output` -  (MSFT_TaskPrincipal)
    /// * `return_value` -  (u32)
    pub fn new_principal_by_user(&self, user_id: &String, logon_type: i32, id: &String, process_token_sid_type: i32, required_privilege: &Vec<String>, run_level: i32, cmdlet_output: &mut MSFT_TaskPrincipal) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UserId".to_string(), value: user_id.into() });
        args.push(MethodParameter { name: "LogonType".to_string(), value: logon_type.into() });
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "ProcessTokenSidType".to_string(), value: process_token_sid_type.into() });
        args.push(MethodParameter { name: "RequiredPrivilege".to_string(), value: required_privilege.into() });
        args.push(MethodParameter { name: "RunLevel".to_string(), value: run_level.into() });

        let result = self.invoke_method("NewPrincipalByUser", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 48

    /// * `allow_start_if_on_batteries` - 53 (bool)
    /// * `compatibility` - 51 (i32)
    /// * `delete_expired_task_after` - 52 (String)
    /// * `disable` - 54 (bool)
    /// * `disallow_demand_start` - 49 (bool)
    /// * `disallow_hard_terminate` - 50 (bool)
    /// * `disallow_start_on_remote_app_session` - 61 (bool)
    /// * `dont_stop_if_going_on_batteries` - 65 (bool)
    /// * `dont_stop_on_idle_end` - 69 (bool)
    /// * `execution_time_limit` - 70 (String)
    /// * `hidden` - 56 (bool)
    /// * `idle_duration` - 67 (String)
    /// * `idle_wait_timeout` - 58 (String)
    /// * `maintenance_deadline` - 63 (String)
    /// * `maintenance_exclusive` - 55 (bool)
    /// * `maintenance_period` - 62 (String)
    /// * `multiple_instances` - 71 (i32)
    /// * `network_id` - 59 (String)
    /// * `network_name` - 60 (String)
    /// * `priority` - 72 (i32)
    /// * `restart_count` - 73 (i32)
    /// * `restart_interval` - 74 (String)
    /// * `restart_on_idle` - 68 (bool)
    /// * `run_only_if_idle` - 57 (bool)
    /// * `run_only_if_network_available` - 75 (bool)
    /// * `start_when_available` - 64 (bool)
    /// * `wake_to_run` - 66 (bool)

    /// * `cmdlet_output` -  (MSFT_TaskSettings)
    /// * `return_value` -  (u32)
    pub fn new_settings(&self, disallow_demand_start: bool, disallow_hard_terminate: bool, compatibility: i32, delete_expired_task_after: &String, allow_start_if_on_batteries: bool, disable: bool, maintenance_exclusive: bool, hidden: bool, run_only_if_idle: bool, idle_wait_timeout: &String, network_id: &String, network_name: &String, disallow_start_on_remote_app_session: bool, maintenance_period: &String, maintenance_deadline: &String, start_when_available: bool, dont_stop_if_going_on_batteries: bool, wake_to_run: bool, idle_duration: &String, restart_on_idle: bool, dont_stop_on_idle_end: bool, execution_time_limit: &String, multiple_instances: i32, priority: i32, restart_count: i32, restart_interval: &String, run_only_if_network_available: bool, cmdlet_output: &mut MSFT_TaskSettings) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisallowDemandStart".to_string(), value: disallow_demand_start.into() });
        args.push(MethodParameter { name: "DisallowHardTerminate".to_string(), value: disallow_hard_terminate.into() });
        args.push(MethodParameter { name: "Compatibility".to_string(), value: compatibility.into() });
        args.push(MethodParameter { name: "DeleteExpiredTaskAfter".to_string(), value: delete_expired_task_after.into() });
        args.push(MethodParameter { name: "AllowStartIfOnBatteries".to_string(), value: allow_start_if_on_batteries.into() });
        args.push(MethodParameter { name: "Disable".to_string(), value: disable.into() });
        args.push(MethodParameter { name: "MaintenanceExclusive".to_string(), value: maintenance_exclusive.into() });
        args.push(MethodParameter { name: "Hidden".to_string(), value: hidden.into() });
        args.push(MethodParameter { name: "RunOnlyIfIdle".to_string(), value: run_only_if_idle.into() });
        args.push(MethodParameter { name: "IdleWaitTimeout".to_string(), value: idle_wait_timeout.into() });
        args.push(MethodParameter { name: "NetworkId".to_string(), value: network_id.into() });
        args.push(MethodParameter { name: "NetworkName".to_string(), value: network_name.into() });
        args.push(MethodParameter { name: "DisallowStartOnRemoteAppSession".to_string(), value: disallow_start_on_remote_app_session.into() });
        args.push(MethodParameter { name: "MaintenancePeriod".to_string(), value: maintenance_period.into() });
        args.push(MethodParameter { name: "MaintenanceDeadline".to_string(), value: maintenance_deadline.into() });
        args.push(MethodParameter { name: "StartWhenAvailable".to_string(), value: start_when_available.into() });
        args.push(MethodParameter { name: "DontStopIfGoingOnBatteries".to_string(), value: dont_stop_if_going_on_batteries.into() });
        args.push(MethodParameter { name: "WakeToRun".to_string(), value: wake_to_run.into() });
        args.push(MethodParameter { name: "IdleDuration".to_string(), value: idle_duration.into() });
        args.push(MethodParameter { name: "RestartOnIdle".to_string(), value: restart_on_idle.into() });
        args.push(MethodParameter { name: "DontStopOnIdleEnd".to_string(), value: dont_stop_on_idle_end.into() });
        args.push(MethodParameter { name: "ExecutionTimeLimit".to_string(), value: execution_time_limit.into() });
        args.push(MethodParameter { name: "MultipleInstances".to_string(), value: multiple_instances.into() });
        args.push(MethodParameter { name: "Priority".to_string(), value: priority.into() });
        args.push(MethodParameter { name: "RestartCount".to_string(), value: restart_count.into() });
        args.push(MethodParameter { name: "RestartInterval".to_string(), value: restart_interval.into() });
        args.push(MethodParameter { name: "RunOnlyIfNetworkAvailable".to_string(), value: run_only_if_network_available.into() });

        let result = self.invoke_method("NewSettings", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 76

    /// * `input_object` - 77 (MSFT_ScheduledTask)

    /// * `return_value` -  (u32)
    pub fn start_by_object(&self, input_object: MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("StartByObject", &args)

    }


/// 76

    /// * `task_name` - 79 (String)
    /// * `task_path` - 78 (String)

    /// * `return_value` -  (u32)
    pub fn start_by_path(&self, task_path: &String, task_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        self.invoke_method("StartByPath", &args)

    }


/// 80

    /// * `input_object` - 81 (MSFT_ScheduledTask)

    /// * `return_value` -  (u32)
    pub fn stop_by_object(&self, input_object: MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("StopByObject", &args)

    }


/// 80

    /// * `task_name` - 83 (String)
    /// * `task_path` - 82 (String)

    /// * `return_value` -  (u32)
    pub fn stop_by_path(&self, task_path: &String, task_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        self.invoke_method("StopByPath", &args)

    }


/// 84

    /// * `input_object` - 24 (MSFT_ScheduledTask)
    /// * `password` - 25 (String)
    /// * `user` - 26 (String)

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn set_by_object(&self, input_object: MSFT_ScheduledTask, password: &String, user: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });

        let result = self.invoke_method("SetByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 84

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `principal` - 85 (MSFT_TaskPrincipal)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `task_name` - 27 (String)
    /// * `task_path` - 28 (String)
    /// * `trigger` - 86 (MSFT_TaskTrigger[])

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn set_by_principal(&self, principal: MSFT_TaskPrincipal, action: &Vec<MSFT_TaskAction>, task_path: &String, settings: MSFT_TaskSettings, trigger: &Vec<MSFT_TaskTrigger>, task_name: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Principal".to_string(), value: principal.into() });
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });

        let result = self.invoke_method("SetByPrincipal", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 84

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `password` - 25 (String)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `task_name` - 27 (String)
    /// * `task_path` - 28 (String)
    /// * `trigger` - 86 (MSFT_TaskTrigger[])
    /// * `user` - 26 (String)

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn set_by_user(&self, action: &Vec<MSFT_TaskAction>, task_path: &String, settings: MSFT_TaskSettings, trigger: &Vec<MSFT_TaskTrigger>, password: &String, user: &String, task_name: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });

        let result = self.invoke_method("SetByUser", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 87

    /// * `task_name` - 88 (String)
    /// * `task_path` - 89 (String)

    /// * `cmdlet_output` -  (MSFT_TaskDynamicInfo)
    /// * `return_value` -  (u32)
    pub fn get_info_by_name(&self, task_name: &String, task_path: &String, cmdlet_output: &mut MSFT_TaskDynamicInfo) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });

        let result = self.invoke_method("GetInfoByName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 87

    /// * `input_object` - 24 (MSFT_ScheduledTask)

    /// * `cmdlet_output` -  (MSFT_TaskDynamicInfo)
    /// * `return_value` -  (u32)
    pub fn get_info_by_object(&self, input_object: MSFT_ScheduledTask, cmdlet_output: &mut MSFT_TaskDynamicInfo) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("GetInfoByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 90

    /// * `action` - 31 (MSFT_TaskAction[])
    /// * `description` - 32 (String)
    /// * `principal` - 85 (MSFT_TaskPrincipal)
    /// * `settings` - 33 (MSFT_TaskSettings)
    /// * `trigger` - 86 (MSFT_TaskTrigger[])

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn new(&self, action: &Vec<MSFT_TaskAction>, description: &String, principal: MSFT_TaskPrincipal, settings: MSFT_TaskSettings, trigger: &Vec<MSFT_TaskTrigger>, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Principal".to_string(), value: principal.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "Trigger".to_string(), value: trigger.into() });

        let result = self.invoke_method("New", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 91

    /// * `task_name` - 92 (String)
    /// * `task_path` - 93 (String)

    /// * `cmdlet_output` -  (String)
    /// * `return_value` -  (u32)
    pub fn export_by_name(&self, task_name: &String, task_path: &String, cmdlet_output: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });

        let result = self.invoke_method("ExportByName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 91

    /// * `input_object` - 94 (MSFT_ScheduledTask)

    /// * `cmdlet_output` -  (String)
    /// * `return_value` -  (u32)
    pub fn export_by_object(&self, input_object: MSFT_ScheduledTask, cmdlet_output: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("ExportByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 95

    /// * `task_name` - 92 (String)
    /// * `task_path` - 89 (String)

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn enable_by_name(&self, task_name: &String, task_path: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });

        let result = self.invoke_method("EnableByName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 95

    /// * `input_object` - 24 (MSFT_ScheduledTask)

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn enable_by_object(&self, input_object: MSFT_ScheduledTask, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("EnableByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 96

    /// * `task_name` - 92 (String)
    /// * `task_path` - 89 (String)

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn disable_by_name(&self, task_name: &String, task_path: &String, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskName".to_string(), value: task_name.into() });
        args.push(MethodParameter { name: "TaskPath".to_string(), value: task_path.into() });

        let result = self.invoke_method("DisableByName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 96

    /// * `input_object` - 24 (MSFT_ScheduledTask)

    /// * `cmdlet_output` -  (MSFT_ScheduledTask)
    /// * `return_value` -  (u32)
    pub fn disable_by_object(&self, input_object: MSFT_ScheduledTask, cmdlet_output: &mut MSFT_ScheduledTask) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("DisableByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 99

    /// * `at` - 101 (String)
    /// * `daily` - 99 (bool)
    /// * `days_interval` - 100 (u32)
    /// * `random_delay` - 98 (String)

    /// * `cmdlet_output` -  (MSFT_TaskTrigger)
    /// * `return_value` -  (u32)
    pub fn new_trigger_by_daily(&self, daily: bool, days_interval: u32, random_delay: &String, at: &String, cmdlet_output: &mut MSFT_TaskTrigger) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Daily".to_string(), value: daily.into() });
        args.push(MethodParameter { name: "DaysInterval".to_string(), value: days_interval.into() });
        args.push(MethodParameter { name: "RandomDelay".to_string(), value: random_delay.into() });
        args.push(MethodParameter { name: "At".to_string(), value: at.into() });

        let result = self.invoke_method("NewTriggerByDaily", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 105

    /// * `at` - 101 (String)
    /// * `once` - 105 (bool)
    /// * `random_delay` - 98 (String)
    /// * `repetition_duration` - 106 (String)
    /// * `repetition_interval` - 107 (String)

    /// * `cmdlet_output` -  (MSFT_TaskTrigger)
    /// * `return_value` -  (u32)
    pub fn new_trigger_by_once(&self, once: bool, random_delay: &String, at: &String, repetition_duration: &String, repetition_interval: &String, cmdlet_output: &mut MSFT_TaskTrigger) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Once".to_string(), value: once.into() });
        args.push(MethodParameter { name: "RandomDelay".to_string(), value: random_delay.into() });
        args.push(MethodParameter { name: "At".to_string(), value: at.into() });
        args.push(MethodParameter { name: "RepetitionDuration".to_string(), value: repetition_duration.into() });
        args.push(MethodParameter { name: "RepetitionInterval".to_string(), value: repetition_interval.into() });

        let result = self.invoke_method("NewTriggerByOnce", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 103

    /// * `at_log_on` - 103 (bool)
    /// * `random_delay` - 98 (String)
    /// * `user` - 104 (String)

    /// * `cmdlet_output` -  (MSFT_TaskTrigger)
    /// * `return_value` -  (u32)
    pub fn new_trigger_by_logon(&self, random_delay: &String, at_log_on: bool, user: &String, cmdlet_output: &mut MSFT_TaskTrigger) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RandomDelay".to_string(), value: random_delay.into() });
        args.push(MethodParameter { name: "AtLogOn".to_string(), value: at_log_on.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });

        let result = self.invoke_method("NewTriggerByLogon", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 108

    /// * `at_startup` - 108 (bool)
    /// * `random_delay` - 98 (String)
    /// * `user` - 104 (String)

    /// * `cmdlet_output` -  (MSFT_TaskTrigger)
    /// * `return_value` -  (u32)
    pub fn new_trigger_by_startup(&self, random_delay: &String, at_startup: bool, user: &String, cmdlet_output: &mut MSFT_TaskTrigger) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RandomDelay".to_string(), value: random_delay.into() });
        args.push(MethodParameter { name: "AtStartup".to_string(), value: at_startup.into() });
        args.push(MethodParameter { name: "User".to_string(), value: user.into() });

        let result = self.invoke_method("NewTriggerByStartup", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 110

    /// * `at` - 101 (String)
    /// * `days_of_week` - 109 (i32[])
    /// * `random_delay` - 98 (String)
    /// * `weekly` - 110 (bool)
    /// * `weeks_interval` - 100 (u32)

    /// * `cmdlet_output` -  (MSFT_TaskTrigger)
    /// * `return_value` -  (u32)
    pub fn new_trigger_by_weekly(&self, random_delay: &String, days_of_week: &Vec<i32>, weekly: bool, weeks_interval: u32, at: &String, cmdlet_output: &mut MSFT_TaskTrigger) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RandomDelay".to_string(), value: random_delay.into() });
        args.push(MethodParameter { name: "DaysOfWeek".to_string(), value: days_of_week.into() });
        args.push(MethodParameter { name: "Weekly".to_string(), value: weekly.into() });
        args.push(MethodParameter { name: "WeeksInterval".to_string(), value: weeks_interval.into() });
        args.push(MethodParameter { name: "At".to_string(), value: at.into() });

        let result = self.invoke_method("NewTriggerByWeekly", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

