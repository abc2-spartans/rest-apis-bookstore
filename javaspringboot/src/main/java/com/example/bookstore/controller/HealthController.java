package com.example.bookstore.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

@RestController
public class HealthController {

    // Shared method to create health response
    private Map<String, Object> createHealthResponse() {
        Map<String, Object> response = new HashMap<>();
        response.put("status", "healthy");
        response.put("timestamp", Instant.now().toString());
        response.put("service", "Bookstore API");
        response.put("apiURL", "http://localhost:5000/api/v1/books");
        return response;
    }

    @GetMapping("/")
    public Map<String, Object> healthRoot() {
        return createHealthResponse();
    }

    @GetMapping("/health")
    public Map<String, Object> healthCheck() {
        return createHealthResponse();
    }

    @GetMapping("/api/v1")
    public Map<String, Object> apiHealth() {
        return createHealthResponse();
    }
}
