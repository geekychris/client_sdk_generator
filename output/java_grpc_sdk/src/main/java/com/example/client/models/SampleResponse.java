package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * Sample gRPC response
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Sampleresponse {
    
    /**
     * Response result
     */
    
    @JsonProperty("result")
    private String result;
    
    
    // Constructors
    public Sampleresponse() {
    }
    
    public Sampleresponse(String result) {
        this.result = result;
    }
    
    // Getters and Setters
    public String getResult() {
        return result;
    }
    
    public void setResult(String result) {
        this.result = result;
    }
    
    public Sampleresponse withResult(String result) {
        this.result = result;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Sampleresponse that = (Sampleresponse) o;
        return Objects.equals(result, that.result);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(result);
    }
    
    @Override
    public String toString() {
        return "Sampleresponse{" +
                "result=" + result +
                '}';
    }
}