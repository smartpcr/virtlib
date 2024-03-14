// -----------------------------------------------------------------------
// <copyright file="MockedLoggerFactory.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Tests.Mocks;

using Microsoft.Extensions.Logging;

internal sealed class MockedLoggerFactory : ILoggerFactory
{
    public void Dispose()
    {
    }

    public ILogger CreateLogger(string categoryName) => new MockedLogger(categoryName);

    public ILogger<T> CreateLogger<T>() => new MockedLogger<T>();

    public void AddProvider(ILoggerProvider provider)
    {
    }
}