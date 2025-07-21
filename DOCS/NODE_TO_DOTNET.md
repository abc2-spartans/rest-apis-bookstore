# Node.js/npm to .NET (with dotnet CLI) Equivalents

This guide maps common Node.js/npm commands and concepts to their .NET equivalents, using the dotnet CLI as the primary tool for project management and development.

---

## Equivalent of `npm init` in .NET

```sh
dotnet new webapi -n MyApp
```

- Creates a new ASP.NET Core Web API project.
- Other templates:

```sh
dotnet new console -n MyApp    # Console application
dotnet new mvc -n MyApp        # MVC web application
dotnet new react -n MyApp      # React SPA template
```

---

## Equivalent of `npm run start` in .NET

```sh
dotnet run
```

- Compiles and runs your .NET application.
- For development with auto-reload:

```sh
dotnet watch run
```

- Or run the built application:

```sh
dotnet build
dotnet ./bin/Debug/net8.0/MyApp.dll
```

---

## Equivalent of `npm install <package_name>` in .NET

```sh
dotnet add package <PackageName>
```

- Adds a NuGet package to your project.
- Example:

```sh
dotnet add package Newtonsoft.Json
dotnet add package Microsoft.EntityFrameworkCore
```

---

## Equivalent of `package.json` in .NET

```xml
<!-- MyApp.csproj -->
<Project Sdk="Microsoft.NET.Sdk.Web">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
    <PackageReference Include="Microsoft.EntityFrameworkCore" Version="8.0.0" />
  </ItemGroup>
</Project>
```

- `.csproj` file contains project metadata, dependencies, and build configuration.

---

## Equivalent of `npm install` in .NET

```sh
dotnet restore
```

- Downloads all NuGet packages listed in `.csproj`.
- Creates `obj/` folder with package cache and build artifacts.
- Usually runs automatically with `dotnet build` or `dotnet run`.

---

## Equivalent of `npm uninstall <package_name>` in .NET

```sh
dotnet remove package <PackageName>
```

- Removes the NuGet package from your project.
- Example:

```sh
dotnet remove package Newtonsoft.Json
```

---

## Equivalent of `npm run <script>` in .NET

- .NET doesn't have built-in scripts like `package.json`.
- Use dotnet CLI commands directly:

```sh
dotnet build
dotnet test
dotnet publish
dotnet watch run
```

- Or create custom MSBuild targets in `.csproj`.

---

## Equivalent of `npx <tool>` in .NET

```sh
dotnet tool install -g <ToolName>
<ToolName>
```

- Or run locally:

```sh
dotnet tool run <ToolName>
```

- Example:

```sh
dotnet tool install -g dotnet-ef
dotnet ef migrations add InitialCreate
```

---

## Equivalent of `JSON.stringify()` in .NET

```csharp
using System.Text.Json;

var data = new {
    name = "John Doe",
    age = 30
};
string jsonString = JsonSerializer.Serialize(data);
```

**Or with Newtonsoft.Json:**

```csharp
using Newtonsoft.Json;

var data = new { name = "John Doe", age = 30 };
string jsonString = JsonConvert.SerializeObject(data);
```

---

## Equivalent of dotenv (`process.env`) in .NET

**Using appsettings.json (recommended):**

```json
{
  "MySecret": "supersecret",
  "ConnectionStrings": {
    "DefaultConnection": "Server=localhost;Database=MyDb;"
  }
}
```

```csharp
// In controller or service
public class MyController : ControllerBase
{
    private readonly IConfiguration _config;

    public MyController(IConfiguration config)
    {
        _config = config;
    }

    public IActionResult Get()
    {
        var secret = _config["MySecret"];
        return Ok(secret);
    }
}
```

**Using environment variables:**

```csharp
string secret = Environment.GetEnvironmentVariable("MY_SECRET");
```

---

## Starting a Typical Server

**ASP.NET Core Web API (Program.cs):**

```csharp
var builder = WebApplication.CreateBuilder(args);

// Add services
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Configure pipeline
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();
app.UseAuthorization();
app.MapControllers();

app.Run();
```

**Minimal API (Program.cs):**

```csharp
var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/", () => new { message = "Hello World" });

app.MapGet("/books", () => new[] { "Book 1", "Book 2" });

app.MapPost("/books", (Book book) =>
{
    return Results.Created($"/books/{book.Id}", book);
});

app.Run();

record Book(int Id, string Title, string Author);
```

---

## Middleware Example (ASP.NET Core)

**Custom Middleware:**

```csharp
public class LoggingMiddleware
{
    private readonly RequestDelegate _next;

    public LoggingMiddleware(RequestDelegate next)
    {
        _next = next;
    }

    public async Task InvokeAsync(HttpContext context)
    {
        Console.WriteLine($"Request: {context.Request.Method} {context.Request.Path}");
        await _next(context);
    }
}

// In Program.cs
app.UseMiddleware<LoggingMiddleware>();
```

**Inline Middleware:**

```csharp
app.Use(async (context, next) =>
{
    Console.WriteLine($"Request: {context.Request.Method} {context.Request.Path}");
    await next();
});
```

---

## Routing Example (ASP.NET Core)

**Controller-based routing:**

```csharp
[ApiController]
[Route("api/v1/[controller]")]
public class BooksController : ControllerBase
{
    [HttpGet]
    public IActionResult GetBooks()
    {
        return Ok(new[] { "Book 1", "Book 2" });
    }

    [HttpGet("{id}")]
    public IActionResult GetBook(int id)
    {
        return Ok(new { id, title = $"Book {id}" });
    }

    [HttpPost]
    public IActionResult CreateBook([FromBody] CreateBookRequest request)
    {
        var book = new { id = 1, title = request.Title, status = "created" };
        return CreatedAtAction(nameof(GetBook), new { id = book.id }, book);
    }
}

public record CreateBookRequest(string Title, string Author);
```

**Minimal API routing:**

```csharp
app.MapGet("/api/v1/books", () => new[] { "Book 1", "Book 2" });

app.MapGet("/api/v1/books/{id}", (int id) =>
    new { id, title = $"Book {id}" });

app.MapPost("/api/v1/books", (CreateBookRequest request) =>
{
    var book = new { id = 1, title = request.Title, status = "created" };
    return Results.Created($"/api/v1/books/{book.id}", book);
});
```

---

## Notes

- dotnet CLI is the primary tool for .NET development and package management.
- .NET is compiled, so `dotnet build` compiles your application.
- ASP.NET Core is the modern web framework for .NET.
- Use `dotnet watch run` for development with auto-reload.
- NuGet packages are cached in global packages folder.
- .NET has strong typing, excellent tooling, and cross-platform support.
- Minimal APIs provide a lightweight alternative to controllers.
