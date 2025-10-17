package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * GraphQL request object
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Graphqlrequest {
    
    /**
     * GraphQL query string
     */
    
    @JsonProperty("query")
    private String query;
    
    
    // Constructors
    public Graphqlrequest() {
    }
    
    public Graphqlrequest(String query) {
        this.query = query;
    }
    
    // Getters and Setters
    public String getQuery() {
        return query;
    }
    
    public void setQuery(String query) {
        this.query = query;
    }
    
    public Graphqlrequest withQuery(String query) {
        this.query = query;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Graphqlrequest that = (Graphqlrequest) o;
        return Objects.equals(query, that.query);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(query);
    }
    
    @Override
    public String toString() {
        return "Graphqlrequest{" +
                "query=" + query +
                '}';
    }
}