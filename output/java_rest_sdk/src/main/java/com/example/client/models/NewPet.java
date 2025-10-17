package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * 
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Newpet {
    
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
     * URLs of pet photos
     */
    
    @JsonProperty("photoUrls")
    private String photourls;
    
    
    // Constructors
    public Newpet() {
    }
    
    public Newpet(String name, String tag, String photourls) {
        this.name = name;
        this.tag = tag;
        this.photourls = photourls;
    }
    
    // Getters and Setters
    public String getName() {
        return name;
    }
    
    public void setName(String name) {
        this.name = name;
    }
    
    public Newpet withName(String name) {
        this.name = name;
        return this;
    }
    
    public String getTag() {
        return tag;
    }
    
    public void setTag(String tag) {
        this.tag = tag;
    }
    
    public Newpet withTag(String tag) {
        this.tag = tag;
        return this;
    }
    
    public String getPhotourls() {
        return photourls;
    }
    
    public void setPhotourls(String photourls) {
        this.photourls = photourls;
    }
    
    public Newpet withPhotourls(String photourls) {
        this.photourls = photourls;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Newpet that = (Newpet) o;
        return Objects.equals(name, that.name) &&
               Objects.equals(tag, that.tag) &&
               Objects.equals(photourls, that.photourls);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(name, tag, photourls);
    }
    
    @Override
    public String toString() {
        return "Newpet{" +
                "name=" + name +
                ", " +
                "tag=" + tag +
                ", " +
                "photourls=" + photourls +
                '}';
    }
}