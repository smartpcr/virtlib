// -----------------------------------------------------------------------
// <copyright file="EnvironmentHook.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Tests.Hooks;

using System;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Reqnroll;

/// <summary>
/// Make sure IConfiguration and ILoggerFactory are registered in ScenarioContext
/// </summary>
[Binding]
public class EnvironmentHook
{
    private readonly ScenarioContext context;
    private readonly IReqnrollOutputHelper outputHelper;

    public EnvironmentHook(ScenarioContext scenarioContext, IReqnrollOutputHelper outputHelper)
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
        var configuration = new ConfigurationBuilder()
            .SetBasePath(AppContext.BaseDirectory)
            .AddJsonFile("appsettings.json", optional: true)
            .AddEnvironmentVariables()
            .Build();
        services.AddSingleton<IConfiguration>(configuration);
        services.AddLogging(builder => builder.AddConsole());
        var serviceProvider = services.BuildServiceProvider();
        this.context.Set<IServiceProvider>(serviceProvider);
        this.context.Set<IConfiguration>(configuration);
    }
}