// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.private.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Linq;
using System.Management;
using System.Security.Principal;
using System.Threading;
using Definitions;

public partial class HostComputeSystem
{
    private ManagementObject GetVirtualMachineManagementService()
    {
        using var managementClass = new ManagementClass(MsvmVirtualSystemManagementService);
        managementClass.Scope = this.virtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        return managementObjects.OfType<ManagementObject>().First();
    }

    private ManagementObject GetSecurityService()
    {
        using var managementClass = new ManagementClass(MsvmSecurityService);
        managementClass.Scope = this.virtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        return managementObjects.OfType<ManagementObject>().First();
    }

    private ManagementObject GetImageManagementService()
    {
        using var managementClass = new ManagementClass(MsvmImageManagementService);
        managementClass.Scope = virtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        return managementObjects.OfType<ManagementObject>().First();
    }

    private static void ValidateOutput(ManagementBaseObject outputParameters)
    {
        uint errorCode = (uint)outputParameters["ReturnValue"];
        if (errorCode == 4096)
        {
            using ManagementObject job = new ManagementObject((string)outputParameters["Job"]);
            while (!IsJobComplete(job["JobState"]))
            {
                Thread.Sleep(500);
                job.Get();
            }

            if (!IsJobSuccessful(job["JobState"]))
            {
                string errorMessage = "The method failed.";
                if (!string.IsNullOrWhiteSpace((string)job["ErrorDescription"]))
                    errorMessage = (string)job["ErrorDescription"];

                throw new ManagementException(errorMessage);
            }

        }
        else if (errorCode != 0)
        {
            throw new ManagementException(ErrorCodeMeaning(errorCode));
        }
    }

    private static bool IsJobComplete(object jobStateObj)
    {
        JobState jobState = (JobState)(ushort)jobStateObj;
        return Enum.IsDefined(typeof(JobState), jobState) && jobState.IsJobComplete();
    }

    private static bool IsJobSuccessful(object jobStateObj)
    {
        JobState jobState = (JobState)(ushort)jobStateObj;

        return
            (jobState == JobState.Completed) ||
            (jobState == JobState.CompletedWithWarnings);
    }

    private static string ErrorCodeMeaning(uint returnValue)
    {
        switch (returnValue)
        {
            case 0: return "Completed with No Error.";
            case 1: return "Not Supported.";
            case 2: return "Failed.";
            case 3: return "Timeout.";
            case 4: return "Invalid Parameter.";
            case 5: return "Invalid State.";
            case 6: return "Invalid Type.";
            case 4096: return "Method Parameters Checked - Job Started.";
            case 32768: return "Failed.";
            case 32769: return "Access Denied.";
            case 32770: return "Not Supported.";
            case 32771: return "Status is Unknown.";
            case 32772: return "Timeout.";
            case 32773: return "Invalid Parameter.";
            case 32774: return "System is In Use.";
            case 32775: return "Invalid State for this Operation.";
            case 32776: return "Incorrect Data Type.";
            case 32777: return "System is Not Available.";
            case 32778: return "Out of Memory.";
            default: return "The Method Failed. The Reason is Unknown.";
        }
    }

    private bool IsElevated()
    {
        var wi = WindowsIdentity.GetCurrent();
        var wp = new WindowsPrincipal(wi);
        return wp.IsInRole(WindowsBuiltInRole.Administrator);
    }
}