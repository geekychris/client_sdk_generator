package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * GraphQL response object
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Graphqlresponse {
    
    /**
     * Response data
     */
    
    @JsonProperty("data")
    private String data;
    
    
    // Constructors
    public Graphqlresponse() {
    }
    
    public Graphqlresponse(String data) {
        this.data = data;
    }
    
    // Getters and Setters
    public String getData() {
        return data;
    }
    
    public void setData(String data) {
        this.data = data;
    }
    
    public Graphqlresponse withData(String data) {
        this.data = data;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Graphqlresponse that = (Graphqlresponse) o;
        return Objects.equals(data, that.data);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(data);
    }
    
    @Override
    public String toString() {
        return "Graphqlresponse{" +
                "data=" + data +
                '}';
    }
}