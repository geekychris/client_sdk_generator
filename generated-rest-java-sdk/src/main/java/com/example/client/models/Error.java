package com.example.client.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import java.util.Objects;

/**
 * 
 */

@JsonIgnoreProperties(ignoreUnknown = true)
public class Error {
    
    /**
     * Error code
     */
    
    @JsonProperty("code")
    private String code;
    
    /**
     * Error message
     */
    
    @JsonProperty("message")
    private String message;
    
    /**
     * Additional error details
     */
    
    @JsonProperty("details")
    private String details;
    
    
    // Constructors
    public Error() {
    }
    
    public Error(String code, String message, String details) {
        this.code = code;
        this.message = message;
        this.details = details;
    }
    
    // Getters and Setters
    public String getCode() {
        return code;
    }
    
    public void setCode(String code) {
        this.code = code;
    }
    
    public Error withCode(String code) {
        this.code = code;
        return this;
    }
    
    public String getMessage() {
        return message;
    }
    
    public void setMessage(String message) {
        this.message = message;
    }
    
    public Error withMessage(String message) {
        this.message = message;
        return this;
    }
    
    public String getDetails() {
        return details;
    }
    
    public void setDetails(String details) {
        this.details = details;
    }
    
    public Error withDetails(String details) {
        this.details = details;
        return this;
    }
    
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Error that = (Error) o;
        return Objects.equals(code, that.code) &&
               Objects.equals(message, that.message) &&
               Objects.equals(details, that.details);
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(code, message, details);
    }
    
    @Override
    public String toString() {
        return "Error{" +
                "code=" + code +
                ", " +
                "message=" + message +
                ", " +
                "details=" + details +
                '}';
    }
}