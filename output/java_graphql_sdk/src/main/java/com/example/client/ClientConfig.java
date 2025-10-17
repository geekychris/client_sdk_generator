package com.example.client;

/**
 * Configuration class for GraphQL API Client SDK
 */
public class ClientConfig {
    
    private String baseUrl;
    private int connectTimeoutSeconds = 30;
    private int requestTimeoutSeconds = 60;
    private String userAgent = "GraphQL API-client/1.0.0";
    private String apiKey;
    private String bearerToken;
    
    private boolean retryEnabled = true;
    private int maxRetries = 3;
    private int retryDelayMs = 1000;
    
    
    private boolean telemetryEnabled = true;
    private String metricsEndpoint;
    
    public ClientConfig(String baseUrl) {
        this.baseUrl = baseUrl;
    }
    
    // Getters and Setters
    
    public String getBaseUrl() {
        return baseUrl;
    }
    
    public ClientConfig setBaseUrl(String baseUrl) {
        this.baseUrl = baseUrl;
        return this;
    }
    
    public int getConnectTimeoutSeconds() {
        return connectTimeoutSeconds;
    }
    
    public ClientConfig setConnectTimeoutSeconds(int connectTimeoutSeconds) {
        this.connectTimeoutSeconds = connectTimeoutSeconds;
        return this;
    }
    
    public int getRequestTimeoutSeconds() {
        return requestTimeoutSeconds;
    }
    
    public ClientConfig setRequestTimeoutSeconds(int requestTimeoutSeconds) {
        this.requestTimeoutSeconds = requestTimeoutSeconds;
        return this;
    }
    
    public String getUserAgent() {
        return userAgent;
    }
    
    public ClientConfig setUserAgent(String userAgent) {
        this.userAgent = userAgent;
        return this;
    }
    
    public String getApiKey() {
        return apiKey;
    }
    
    public ClientConfig setApiKey(String apiKey) {
        this.apiKey = apiKey;
        return this;
    }
    
    public String getBearerToken() {
        return bearerToken;
    }
    
    public ClientConfig setBearerToken(String bearerToken) {
        this.bearerToken = bearerToken;
        return this;
    }
    
    public boolean isRetryEnabled() {
        return retryEnabled;
    }
    
    public ClientConfig setRetryEnabled(boolean retryEnabled) {
        this.retryEnabled = retryEnabled;
        return this;
    }
    
    public int getMaxRetries() {
        return maxRetries;
    }
    
    public ClientConfig setMaxRetries(int maxRetries) {
        this.maxRetries = maxRetries;
        return this;
    }
    
    public int getRetryDelayMs() {
        return retryDelayMs;
    }
    
    public ClientConfig setRetryDelayMs(int retryDelayMs) {
        this.retryDelayMs = retryDelayMs;
        return this;
    }
    
    
    public boolean isTelemetryEnabled() {
        return telemetryEnabled;
    }
    
    public ClientConfig setTelemetryEnabled(boolean telemetryEnabled) {
        this.telemetryEnabled = telemetryEnabled;
        return this;
    }
    
    public String getMetricsEndpoint() {
        return metricsEndpoint;
    }
    
    public ClientConfig setMetricsEndpoint(String metricsEndpoint) {
        this.metricsEndpoint = metricsEndpoint;
        return this;
    }
}