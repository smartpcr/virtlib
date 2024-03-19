// -----------------------------------------------------------------------
// <copyright file="JobOutputHelper.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Management;
using System.Threading;
using Definitions;

public static class JobOutputHelper
{
    public static void ValidateOutput(ManagementBaseObject outputParameters)
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
                {
                    errorMessage = (string)job["ErrorDescription"];
                }

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
        return returnValue switch
        {
            0 => "Completed with No Error.",
            1 => "Not Supported.",
            2 => "Failed.",
            3 => "Timeout.",
            4 => "Invalid Parameter.",
            5 => "Invalid State.",
            6 => "Invalid Type.",
            4096 => "Method Parameters Checked - Job Started.",
            32768 => "Failed.",
            32769 => "Access Denied.",
            32770 => "Not Supported.",
            32771 => "Status is Unknown.",
            32772 => "Timeout.",
            32773 => "Invalid Parameter.",
            32774 => "System is In Use.",
            32775 => "Invalid State for this Operation.",
            32776 => "Incorrect Data Type.",
            32777 => "System is Not Available.",
            32778 => "Out of Memory.",
            _ => "The Method Failed. The Reason is Unknown."
        };
    }
}