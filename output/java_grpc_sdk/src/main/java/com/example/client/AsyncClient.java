package com.example.client;

import dev.failsafe.Failsafe;
import dev.failsafe.RetryPolicy;
import io.micrometer.core.instrument.Counter;
import io.micrometer.core.instrument.MeterRegistry;
import io.micrometer.core.instrument.Timer;
import io.micrometer.prometheus.PrometheusConfig;
import io.micrometer.prometheus.PrometheusMeterRegistry;
import java.time.Duration;

import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.URI;
import java.time.Duration;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.DeserializationFeature;

/**
 * Generated from UserService gRPC proto files
 * 
 * Async Generated SDK for UserService gRPC API v1.0.0
 */
public class UserserviceGrpcApiAsyncClient {
    
    private final HttpClient httpClient;
    private final ObjectMapper objectMapper;
    private final String baseUrl;
    private final ClientConfig config;
    
    private final TelemetryHandler telemetryHandler;
    
    
    public UserserviceGrpcApiAsyncClient(ClientConfig config) {
        this.config = config;
        this.baseUrl = config.getBaseUrl();
        
        this.httpClient = HttpClient.newBuilder()
            .connectTimeout(Duration.ofSeconds(config.getConnectTimeoutSeconds()))
            .build();
            
        this.objectMapper = new ObjectMapper()
            .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
            
        this.telemetryHandler = new TelemetryHandler();
        
    }
    
    /**
     * Register a new user account (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> registeruserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/RegisterUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute RegisterUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Authenticate a user and create session (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> loginuserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/LoginUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute LoginUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Refresh authentication token (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> refreshtokenAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/RefreshToken");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute RefreshToken: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Logout user and invalidate session (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> logoutuserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/LogoutUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute LogoutUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Get user profile by ID (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> getuserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/GetUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute GetUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Get current authenticated user profile (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> getcurrentuserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/GetCurrentUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute GetCurrentUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Update user profile information (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> updateuserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/UpdateUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute UpdateUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Delete user account (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> deleteuserAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/DeleteUser");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute DeleteUser: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * List users with pagination and filtering (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> listusersAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/ListUsers");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute ListUsers: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Change user password (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> changepasswordAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/ChangePassword");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute ChangePassword: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Reset user password (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> resetpasswordAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/ResetPassword");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute ResetPassword: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Send email verification (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> sendverificationemailAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/SendVerificationEmail");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute SendVerificationEmail: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Verify email address (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> verifyemailAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/VerifyEmail");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute VerifyEmail: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * List user active sessions (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> listusersessionsAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/ListUserSessions");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute ListUserSessions: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Revoke a user session (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> revokesessionAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/RevokeSession");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute RevokeSession: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Get user preferences (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> getuserpreferencesAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/GetUserPreferences");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute GetUserPreferences: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Update user preferences (Async)
     * @param request The request body
     * @return CompletableFuture containing gRPC response
     */
    
    public CompletableFuture<String> updateuserpreferencesAsync(
        
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/userservice.v1.UserService/UpdateUserPreferences");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.POST(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                String jsonBody = objectMapper.writeValueAsString(request);
                requestBuilder.PUT(HttpRequest.BodyPublishers.ofString(jsonBody));
                requestBuilder.header("Content-Type", "application/json");
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new UserserviceGrpcApiException("Failed to execute UpdateUserPreferences: " + e.getMessage(), e);
            }
        });
    }
    
    
    private String buildUrl(String path, Object... pathParams) {
        String url = baseUrl + path;
        for (int i = 0; i < pathParams.length; i++) {
            url = url.replaceFirst("\\{[^}]+\\}", String.valueOf(pathParams[i]));
        }
        return url;
    }
    
    private <T> T parseResponse(HttpResponse<String> response, Class<T> responseType) throws Exception {
        if (response.statusCode() >= 200 && response.statusCode() < 300) {
            if (responseType == Void.class) {
                return null;
            }
            return objectMapper.readValue(response.body(), responseType);
        } else {
            throw new UserserviceGrpcApiException("HTTP " + response.statusCode() + ": " + response.body());
        }
    }
    
    
private static final RetryPolicy<Object> RETRY_POLICY = RetryPolicy.builder()
    .withMaxAttempts(3)
    .withBackoff(Duration.ofMillis(100), Duration.ofMillis(5000), 2)
    .handle(ConnectException.class, SocketTimeoutException.class)
    .build();

public <T> T executeWithRetry(Supplier<T> operation) {
    return Failsafe.with(RETRY_POLICY).get(operation);
}



private final MeterRegistry meterRegistry;
private final Timer requestTimer;
private final Counter requestCounter;
private final Counter errorCounter;

public void recordRequest(String method, String path, long durationMs, boolean success) {
    requestTimer.record(Duration.ofMillis(durationMs));
    requestCounter.increment(
        Tags.of(
            Tag.of("method", method),
            Tag.of("path", path)
        )
    );
    
    if (!success) {
        errorCounter.increment(
            Tags.of(
                Tag.of("method", method),
                Tag.of("path", path)
            )
        );
    }
}

    
    public static class UserserviceGrpcApiException extends RuntimeException {
        public UserserviceGrpcApiException(String message) {
            super(message);
        }
        
        public UserserviceGrpcApiException(String message, Throwable cause) {
            super(message, cause);
        }
    }
}