package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * 
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Pet {
    
    /**
     * Unique identifier for the pet
     */
    
    @JsonProperty("id")
    private String id;
    
    /**
     * Name of the pet
     */
    
    @JsonProperty("name")
    private String name;
    
    /**
     * Tag associated with the pet
     */
    
    @JsonProperty("tag")
    private String tag;
    
    /**
     * Pet status in the store
     */
    
    @JsonProperty("status")
    private String status;
    
    /**
     * URLs of pet photos
     */
    
    @JsonProperty("photoUrls")
    private String photourls;
    
    
    // Constructors
    public Pet() {
    }
    
    public Pet(String id, String name, String tag, String status, String photourls) {
        this.id = id;
        this.name = name;
        this.tag = tag;
        this.status = status;
        this.photourls = photourls;
    }
    
    // Getters and Setters
    public String getId() {
        return id;
    }
    
    public void setId(String id) {
        this.id = id;
    }
    
    public Pet withId(String id) {
        this.id = id;
        return this;
    }
    
    public String getName() {
        return name;
    }
    
    public void setName(String name) {
        this.name = name;
    }
    
    public Pet withName(String name) {
        this.name = name;
        return this;
    }
    
    public String getTag() {
        return tag;
    }
    
    public void setTag(String tag) {
        this.tag = tag;
    }
    
    public Pet withTag(String tag) {
        this.tag = tag;
        return this;
    }
    
    public String getStatus() {
        return status;
    }
    
    public void setStatus(String status) {
        this.status = status;
    }
    
    public Pet withStatus(String status) {
        this.status = status;
        return this;
    }
    
    public String getPhotourls() {
        return photourls;
    }
    
    public void setPhotourls(String photourls) {
        this.photourls = photourls;
    }
    
    public Pet withPhotourls(String photourls) {
        this.photourls = photourls;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Pet that = (Pet) o;
        return Objects.equals(id, that.id) &&
               Objects.equals(name, that.name) &&
               Objects.equals(tag, that.tag) &&
               Objects.equals(status, that.status) &&
               Objects.equals(photourls, that.photourls);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(id, name, tag, status, photourls);
    }
    
    @Override
    public String toString() {
        return "Pet{" +
                "id=" + id +
                ", " +
                "name=" + name +
                ", " +
                "tag=" + tag +
                ", " +
                "status=" + status +
                ", " +
                "photourls=" + photourls +
                '}';
    }
}