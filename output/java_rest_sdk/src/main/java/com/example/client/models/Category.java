package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * 
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Category {
    
    /**
     * Category identifier
     */
    
    @JsonProperty("id")
    private String id;
    
    /**
     * Category name
     */
    
    @JsonProperty("name")
    private String name;
    
    
    // Constructors
    public Category() {
    }
    
    public Category(String id, String name) {
        this.id = id;
        this.name = name;
    }
    
    // Getters and Setters
    public String getId() {
        return id;
    }
    
    public void setId(String id) {
        this.id = id;
    }
    
    public Category withId(String id) {
        this.id = id;
        return this;
    }
    
    public String getName() {
        return name;
    }
    
    public void setName(String name) {
        this.name = name;
    }
    
    public Category withName(String name) {
        this.name = name;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Category that = (Category) o;
        return Objects.equals(id, that.id) &&
               Objects.equals(name, that.name);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(id, name);
    }
    
    @Override
    public String toString() {
        return "Category{" +
                "id=" + id +
                ", " +
                "name=" + name +
                '}';
    }
}