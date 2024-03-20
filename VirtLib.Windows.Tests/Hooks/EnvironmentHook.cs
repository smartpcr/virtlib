// -----------------------------------------------------------------------
// <copyright file="EnvironmentHook.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Tests.Hooks;

using System;
using Common.Config;
using Common.Monitoring;
using Microsoft.Extensions.DependencyInjection;
using TechTalk.SpecFlow;
using TechTalk.SpecFlow.Infrastructure;

/// <summary>
/// Make sure IConfiguration and ILoggerFactory are registered in ScenarioContext
/// </summary>
[Binding]
public class EnvironmentHook
{
    private readonly ScenarioContext context;
    private readonly ISpecFlowOutputHelper outputHelper;

    public EnvironmentHook(ScenarioContext scenarioContext, ISpecFlowOutputHelper outputHelper)
    {
        this.context = scenarioContext;
        this.outputHelper = outputHelper;
    }

    [BeforeScenario]
    public void SetupEnv()
    {
        SetupEnv("Production");
    }

    [BeforeScenario("dev")]
    public void SetupDevEnv()
    {
        SetupEnv("Development");
    }

    [BeforeScenario("prod")]
    public void SetupProdEnv()
    {
        SetupEnv("Production");
    }

    [AfterScenario]
    public void Cleanup()
    {
        foreach (var value in this.context.Values)
        {
            if (value is IDisposable disposable)
            {
                disposable.Dispose();
            }
        }
    }

    private void SetupEnv(string envName)
    {
        outputHelper.WriteLine($"Use {envName} environment");
        Environment.SetEnvironmentVariable("ASPNETCORE_ENVIRONMENT", envName, EnvironmentVariableTarget.Process);
        ConfigureServices();
    }

    private void ConfigureServices()
    {
        var services = this.context.GetServices();
        var configuration = services.AddConfiguration();
        services.AddR9Monitoring(configuration);
        var serviceProvider = services.BuildServiceProvider();
        this.context.Set<IServiceProvider>(serviceProvider);
        this.context.Set(configuration);
    }
}