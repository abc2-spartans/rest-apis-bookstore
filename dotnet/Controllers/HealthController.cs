using Microsoft.AspNetCore.Mvc;

[ApiController]
[Route("/"),Route("api/v1/"),Route("api/v1/[controller]")]
public class HealthController : ControllerBase
{
    [HttpGet]
    public ActionResult GetHealth()
    {
        return Ok(new { 
            status = "healthy", 
            timestamp = DateTime.UtcNow,
            service = "Bookstore API",
            apiURL = "http://localhost:5000/api/v1/books"
        });
    }

    [HttpGet("status")]
    public ActionResult GetDetailedStatus()
    {
        return Ok(new { 
            status = "healthy",
            timestamp = DateTime.UtcNow,
            service = "Bookstore API",
            version = "1.0.0",
            uptime = Environment.TickCount64
        });
    }
}
