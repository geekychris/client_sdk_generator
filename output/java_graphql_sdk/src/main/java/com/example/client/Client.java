package com.example.client;

import dev.failsafe.Failsafe;
import dev.failsafe.RetryPolicy;
import io.micrometer.core.instrument.Counter;
import io.micrometer.core.instrument.MeterRegistry;
import io.micrometer.core.instrument.Timer;
import io.micrometer.prometheus.PrometheusConfig;
import io.micrometer.prometheus.PrometheusMeterRegistry;
import java.time.Duration;

import com.apollographql.apollo3.ApolloClient;
import com.apollographql.apollo3.api.ApolloResponse;
import com.apollographql.apollo3.api.Query;
import com.apollographql.apollo3.api.Mutation;
import com.apollographql.apollo3.api.Subscription;
import com.apollographql.apollo3.exception.ApolloException;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.concurrent.CompletableFuture;
import java.util.Map;
import java.util.HashMap;
import java.time.Duration;

/**
 * Generated from GraphQL schema
 * 
 * Generated SDK for GraphQL API v1.0.0 - GraphQL Client using Apollo
 */
public class GraphqlApiClient {
    
    private final ApolloClient apolloClient;
    private final ObjectMapper objectMapper;
    
    private final TelemetryHandler telemetryHandler;
    
    
    public GraphqlApiClient(ClientConfig config) {
        ApolloClient.Builder builder = new ApolloClient.Builder()
            .serverUrl(config.getBaseUrl())
            .httpConnectTimeout(Duration.ofSeconds(config.getConnectTimeoutSeconds()))
            .httpReadTimeout(Duration.ofSeconds(config.getRequestTimeoutSeconds()));
            
        // Add authentication if configured
        if (config.getApiKey() != null) {
            builder.addHttpHeader("Authorization", "Bearer " + config.getApiKey());
        }
        
        this.apolloClient = builder.build();
        this.objectMapper = new ObjectMapper();
            
        this.telemetryHandler = new TelemetryHandler();
        
    }
    
    /**
     * Execute a GraphQL query
     * @param query The GraphQL query string
     * @param variables Variables for the query
     * @param responseType The expected response type class
     * @return Query response
     */
    public <T> T executeQuery(String query, Map<String, Object> variables, Class<T> responseType) throws ApolloException {
        Map<String, Object> payload = new HashMap<>();
        payload.put("query", query);
        if (variables != null) {
            payload.put("variables", variables);
        }
        
        try {
            // Note: In a real implementation, you'd use Apollo's generated query classes
            ApolloResponse<Object> response = apolloClient.query(new GenericQuery(query, variables)).execute();
            
            if (response.hasErrors()) {
                throw new ApolloException("GraphQL errors: " + response.errors);
            }
            
            Object data = response.data;
            return objectMapper.convertValue(data, responseType);
        } catch (Exception e) {
            throw new ApolloException("Failed to execute GraphQL query", e);
        }
    }
    
    /**
     * Execute a GraphQL mutation
     * @param mutation The GraphQL mutation string
     * @param variables Variables for the mutation
     * @param responseType The expected response type class
     * @return Mutation response
     */
    public <T> T executeMutation(String mutation, Map<String, Object> variables, Class<T> responseType) throws ApolloException {
        try {
            ApolloResponse<Object> response = apolloClient.mutation(new GenericMutation(mutation, variables)).execute();
            
            if (response.hasErrors()) {
                throw new ApolloException("GraphQL errors: " + response.errors);
            }
            
            Object data = response.data;
            return objectMapper.convertValue(data, responseType);
        } catch (Exception e) {
            throw new ApolloException("Failed to execute GraphQL mutation", e);
        }
    }
    
    /**
     * Close the GraphQL client and release resources
     */
    public void close() {
        if (apolloClient != null) {
            apolloClient.close();
        }
    }
    
    // Helper classes for generic GraphQL operations
    private static class GenericQuery implements Query<Object> {
        private final String queryString;
        private final Map<String, Object> variables;
        
        public GenericQuery(String queryString, Map<String, Object> variables) {
            this.queryString = queryString;
            this.variables = variables;
        }
        
        @Override
        public String document() {
            return queryString;
        }
        
        public Map<String, Object> variables() {
            return variables;
        }
    }
    
    private static class GenericMutation implements Mutation<Object> {
        private final String mutationString;
        private final Map<String, Object> variables;
        
        public GenericMutation(String mutationString, Map<String, Object> variables) {
            this.mutationString = mutationString;
            this.variables = variables;
        }
        
        @Override
        public String document() {
            return mutationString;
        }
        
        public Map<String, Object> variables() {
            return variables;
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