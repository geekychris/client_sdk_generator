package 

import (
	"encoding/json"
	"time"
)

// Error 
type Error struct {
	// Error code
Code String `json:"code"`
	// Error message
Message String `json:"message"`
	// Additional error details
Details *String `json:"details" omitempty`
}

// NewError creates a new Error with required fields
func NewError(code String, message String, ) *Error {
	return &Error{
		Code: code,
		Message: message,
	}
}

// SetDetails sets the details field
func (m *Error) SetDetails(details String) *Error {
	m.Details = &details
	return m
}

// GetDetails returns the details field value if set, nil otherwise
func (m *Error) GetDetails() *String {
	return m.Details
}

// HasDetails returns true if the details field is set
func (m *Error) HasDetails() bool {
	return m.Details != nil
}



// Validate validates the Error instance
func (m *Error) Validate() error {
	// Add validation logic here if needed
	// Code is required
	// Message is required
	return nil
}

// String returns a string representation of the Error
func (m *Error) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}