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
 * A sample API that uses a pet store as an example
 * 
 * Async Generated SDK for Pet Store API v1.0.0
 */
public class PetStoreApiAsyncClient {
    
    private final HttpClient httpClient;
    private final ObjectMapper objectMapper;
    private final String baseUrl;
    private final ClientConfig config;
    
    private final TelemetryHandler telemetryHandler;
    
    
    public PetStoreApiAsyncClient(ClientConfig config) {
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
     * Returns a list of all pets in the store (Async)
     * @param limit Maximum number of pets to return
     * @param tag Filter pets by tag
     * @return CompletableFuture containing A list of pets
     */
    
    public CompletableFuture<String> listpetsAsync(
        String limit,
        String tag
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/pets", limit, tag);
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                requestBuilder.POST(HttpRequest.BodyPublishers.noBody());
                requestBuilder.PUT(HttpRequest.BodyPublishers.noBody());
                requestBuilder.DELETE();
                
                // Add query parameters
                if (limit != null) {
                    url += (url.contains("?") ? "&" : "?") + "limit=" + limit;
                }
                if (tag != null) {
                    url += (url.contains("?") ? "&" : "?") + "tag=" + tag;
                }
                
                // Add headers
                if (limit != null) {
                    requestBuilder.header("limit", String.valueOf(limit));
                }
                if (tag != null) {
                    requestBuilder.header("tag", String.valueOf(tag));
                }
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                String result = parseResponse(response, String.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new PetStoreApiException("Failed to execute listPets: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Creates a new pet in the store (Async)
     * @return CompletableFuture containing Pet created successfully
     */
    
    public CompletableFuture<Void> createpetAsync(
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/pets");
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                requestBuilder.POST(HttpRequest.BodyPublishers.noBody());
                requestBuilder.PUT(HttpRequest.BodyPublishers.noBody());
                requestBuilder.DELETE();
                
                // Add query parameters
                
                // Add headers
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                Void result = parseResponse(response, Void.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new PetStoreApiException("Failed to execute createPet: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Returns a single pet by its ID (Async)
     * @param petid ID of the pet to retrieve
     * @return CompletableFuture containing Pet details
     */
    
    public CompletableFuture<Void> getpetAsync(
        String petid
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/pets/{petId}", petid);
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                requestBuilder.POST(HttpRequest.BodyPublishers.noBody());
                requestBuilder.PUT(HttpRequest.BodyPublishers.noBody());
                requestBuilder.DELETE();
                
                // Add query parameters
                if (petid != null) {
                    url += (url.contains("?") ? "&" : "?") + "petId=" + petid;
                }
                
                // Add headers
                if (petid != null) {
                    requestBuilder.header("petId", String.valueOf(petid));
                }
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                Void result = parseResponse(response, Void.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new PetStoreApiException("Failed to execute getPet: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Updates an existing pet (Async)
     * @param petid ID of the pet to update
     * @return CompletableFuture containing Pet updated successfully
     */
    
    public CompletableFuture<Void> updatepetAsync(
        String petid
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/pets/{petId}", petid);
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                requestBuilder.POST(HttpRequest.BodyPublishers.noBody());
                requestBuilder.PUT(HttpRequest.BodyPublishers.noBody());
                requestBuilder.DELETE();
                
                // Add query parameters
                if (petid != null) {
                    url += (url.contains("?") ? "&" : "?") + "petId=" + petid;
                }
                
                // Add headers
                if (petid != null) {
                    requestBuilder.header("petId", String.valueOf(petid));
                }
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                Void result = parseResponse(response, Void.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new PetStoreApiException("Failed to execute updatePet: " + e.getMessage(), e);
            }
        });
    }
    
    /**
     * Deletes a pet from the store (Async)
     * @param petid ID of the pet to delete
     * @return CompletableFuture containing Pet deleted successfully
     */
    
    public CompletableFuture<Void> deletepetAsync(
        String petid
    ) {
        return CompletableFuture.supplyAsync(() -> {
            
            
            try {
                String url = buildUrl("/pets/{petId}", petid);
                
                HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .timeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
                
                requestBuilder.GET();
                requestBuilder.POST(HttpRequest.BodyPublishers.noBody());
                requestBuilder.PUT(HttpRequest.BodyPublishers.noBody());
                requestBuilder.DELETE();
                
                // Add query parameters
                if (petid != null) {
                    url += (url.contains("?") ? "&" : "?") + "petId=" + petid;
                }
                
                // Add headers
                if (petid != null) {
                    requestBuilder.header("petId", String.valueOf(petid));
                }
                
                HttpRequest httpRequest = requestBuilder.build();
                
                HttpResponse<String> response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString());
                Void result = parseResponse(response, Void.class);
                
                
                
                return result;
                
            } catch (Exception e) {
                throw new PetStoreApiException("Failed to execute deletePet: " + e.getMessage(), e);
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
            throw new PetStoreApiException("HTTP " + response.statusCode() + ": " + response.body());
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

    
    public static class PetStoreApiException extends RuntimeException {
        public PetStoreApiException(String message) {
            super(message);
        }
        
        public PetStoreApiException(String message, Throwable cause) {
            super(message, cause);
        }
    }
}