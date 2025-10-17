package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * Sample gRPC request
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Samplerequest {
    
    /**
     * Request message
     */
    
    @JsonProperty("message")
    private String message;
    
    
    // Constructors
    public Samplerequest() {
    }
    
    public Samplerequest(String message) {
        this.message = message;
    }
    
    // Getters and Setters
    public String getMessage() {
        return message;
    }
    
    public void setMessage(String message) {
        this.message = message;
    }
    
    public Samplerequest withMessage(String message) {
        this.message = message;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Samplerequest that = (Samplerequest) o;
        return Objects.equals(message, that.message);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(message);
    }
    
    @Override
    public String toString() {
        return "Samplerequest{" +
                "message=" + message +
                '}';
    }
}