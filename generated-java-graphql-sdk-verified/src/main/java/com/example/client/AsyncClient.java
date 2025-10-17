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
 * Generated from GraphQL schema
 * 
 * Async Generated SDK for GraphQL API v1.0.0
 */
public class GraphqlApiAsyncClient {
    
    private final HttpClient httpClient;
    private final ObjectMapper objectMapper;
    private final String baseUrl;
    private final ClientConfig config;
    
    private final TelemetryHandler telemetryHandler;
    
    
    public GraphqlApiAsyncClient(ClientConfig config) {
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
     * Sample GraphQL query (Async)
     * @param query GraphQL query string
     * @param request The request body
     * @return CompletableFuture containing GraphQL response
     */
    
    public CompletableFuture<String> sampleAsync(
        String query
        ,
        String request
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/graphql", query);
                
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
                if (query != null) {
                    url += (url.contains("?") ? "&" : "?") + "query=" + query;
                }
                
                // Add headers
                if (query != null) {
                    requestBuilder.header("query", String.valueOf(query));
                }
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new GraphqlApiException("Failed to execute sample: " + e.getMessage(), e);
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
            throw new GraphqlApiException("HTTP " + response.statusCode() + ": " + response.body());
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

    
    public static class GraphqlApiException extends RuntimeException {
        public GraphqlApiException(String message) {
            super(message);
        }
        
        public GraphqlApiException(String message, Throwable cause) {
            super(message, cause);
        }
    }
}