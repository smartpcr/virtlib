// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ResourcePoolConfigurationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ResourcePoolConfigurationService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl CIM_ResourcePoolConfigurationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// Starts a job to create a root ResourcePool. The ResourcePool will be scoped to the same System as this Service. If 0 is returned, then the task completed successfully and the use of ConcreteJob was not required. If the task will take some time to complete, a ConcreteJob will be created and its reference returned in the output parameter Job. The resulting pool will be a root pool with no parent pool.

    /// * `element_name` - A end user relevant name for the pool being created. If NULL, then a system supplied default name can be used. The value will be stored in the 'ElementName' property for the created pool. (String)
    /// * `host_resources` - Array of zero or more devices that are used to create the Pool or modify the source extents. All elements in the array must be of the same type. (CIM_LogicalDevice[])
    /// * `resource_type` - The type of resources the created poolwill manage. If HostResources contains elements, this property must mach their type. (String)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `pool` - On success, a reference to the resulting ResourcePool is returned. When a Job is returned, this may be NULL, in which case, the client must use the Job to find the resulting ResourcePool once the Job completes. (CIM_ResourcePool)
    /// * `return_value` -  (u32)
    pub fn create_resource_pool(&self, element_name: &String, host_resources: &Vec<CIM_LogicalDevice>, resource_type: &String, pool: &mut CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ElementName".to_string(), value: element_name.into() });
        args.push(MethodParameter { name: "HostResources".to_string(), value: host_resources.into() });
        args.push(MethodParameter { name: "ResourceType".to_string(), value: resource_type.into() });

        let result = self.invoke_method_with_job("CreateResourcePool", &args)?;
        let job = result.get_value("Job")?;
        let pool = result.get_value("Pool")?;
        Ok(result.return_value)

    }


/// Start a job to create a sub-pool from a parent pool using the specified allocation settings If 0 is returned, the function completed successfully and no ConcreteJob instance was required. If 4096/0x1000 is returned, a ConcreteJob will be started to create the sub-pool. The Job's reference will be returned in the output parameter Job.

    /// * `element_name` - A end user relevant name for the pool being created. If NULL, then a system supplied default name can be used. The value will be stored in the 'ElementName' property for the created element. (String)
    /// * `parent_pool` - The Pool(s) from which to create the new Pool. (CIM_ResourcePool[])
    /// * `settings` - String containing a representation of a CIM_SettingData instance that is used to specify the settings for the child Pool. (String[])

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `pool` - A reference to the resulting pool. (CIM_ResourcePool)
    /// * `return_value` -  (u32)
    pub fn create_child_resource_pool(&self, element_name: &String, settings: &Vec<String>, parent_pool: &Vec<CIM_ResourcePool>, pool: &mut CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ElementName".to_string(), value: element_name.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "ParentPool".to_string(), value: parent_pool.into() });

        let result = self.invoke_method_with_job("CreateChildResourcePool", &args)?;
        let job = result.get_value("Job")?;
        let pool = result.get_value("Pool")?;
        Ok(result.return_value)

    }


/// Start a job to delete a ResourcePool. No allocations may be outstanding or the delete will fail with "In Use." If the resource pool is a root resource pool, any host resources are returned back to the underlying system. If 0 is returned, the function completed successfully, and no ConcreteJob was required. If 4096/0x1000 is returned, a ConcreteJob will be started to delete the ResourcePool. A reference to the Job is returned in the Job parameter.

    /// * `pool` - Reference to the pool to delete. (CIM_ResourcePool)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn delete_resource_pool(&self, pool: CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Pool".to_string(), value: pool.into() });

        let result = self.invoke_method_with_job("DeleteResourcePool", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Starts a job to add resources to a ResourcePool. If 0 is returned, then the task completed successfully and the use of ConcreteJob was not required. If the task will take some time to complete, a ConcreteJob will be created and its reference returned in the output parameter Job. The resulting pool will be a root pool with no parent pool.

    /// * `host_resources` - Array of CIM_LogicalDevice instances to add to the Pool. (CIM_LogicalDevice[])
    /// * `pool` - The pool to add the resources to. (CIM_ResourcePool)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn add_resources_to_resource_pool(&self, host_resources: &Vec<CIM_LogicalDevice>, pool: CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HostResources".to_string(), value: host_resources.into() });
        args.push(MethodParameter { name: "Pool".to_string(), value: pool.into() });

        let result = self.invoke_method_with_job("AddResourcesToResourcePool", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Starts a job to remove resources from a ResourcePool. If 0 is returned, then the task completed successfully and the use of ConcreteJob was not required. If the task will take some time to complete, a ConcreteJob will be created and its reference returned in the output parameter Job. The resulting pool will be a root pool with no parent pool.

    /// * `host_resources` - Array of CIM_LogicalDevice instances to remove from the Pool. (CIM_LogicalDevice[])
    /// * `pool` - The pool to remove the resources from. (CIM_ResourcePool)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_resources_from_resource_pool(&self, host_resources: &Vec<CIM_LogicalDevice>, pool: CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HostResources".to_string(), value: host_resources.into() });
        args.push(MethodParameter { name: "Pool".to_string(), value: pool.into() });

        let result = self.invoke_method_with_job("RemoveResourcesFromResourcePool", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Start a job to change a parent pool using the specified allocation settings If 0 is returned, the function completed successfully and no ConcreteJob instance was required. If 4096/0x1000 is returned, a ConcreteJob will be started to change the parent pool. The Job's reference will be returned in the output parameter Job.

    /// * `child_pool` - Reference to the child pool. (CIM_ResourcePool)
    /// * `parent_pool` - Reference to the parent pool(s). (CIM_ResourcePool[])
    /// * `settings` - Optional string containing a representation of a CIM_SettingData instance that is used to specify the settings for the Parent Pool. (String[])

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn change_parent_resource_pool(&self, child_pool: CIM_ResourcePool, parent_pool: &Vec<CIM_ResourcePool>, settings: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ChildPool".to_string(), value: child_pool.into() });
        args.push(MethodParameter { name: "ParentPool".to_string(), value: parent_pool.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });

        let result = self.invoke_method_with_job("ChangeParentResourcePool", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

