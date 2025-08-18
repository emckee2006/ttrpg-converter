using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Schemas;
using TTRPGConverter.CLI.Commands;

Console.WriteLine("TTRPGConverter CLI - Model Generation");

var builder = Host.CreateApplicationBuilder(args);
builder.Services.AddLogging(configure => configure.AddConsole());
builder.Services.AddSingleton<SchemaGenerator>();
builder.Services.AddSingleton<GenerateModelsCommand>();

var host = builder.Build();
var generateModelsCommand = host.Services.GetRequiredService<GenerateModelsCommand>();

// Simple hardcoded paths for now
var schemaPath = "schemas";
var outputPath = "src/TTRPGConverter.Core/Models";
var baseNamespace = "TTRPGConverter.Core.Models";

if (args.Length >= 1) schemaPath = args[0];
if (args.Length >= 2) outputPath = args[1];
if (args.Length >= 3) baseNamespace = args[2];

await generateModelsCommand.ExecuteAsync(schemaPath, outputPath, baseNamespace);
return 0;
