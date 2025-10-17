package 

import (
	"encoding/json"
	"time"
)

// Samplerequest Sample gRPC request
type Samplerequest struct {
	// Request message
Message String `json:"message"`
}

// NewSamplerequest creates a new Samplerequest with required fields
func NewSamplerequest(message String) *Samplerequest {
	return &Samplerequest{
		Message: message,
	}
}



// Validate validates the Samplerequest instance
func (m *Samplerequest) Validate() error {
	// Add validation logic here if needed
	// Message is required
	return nil
}

// String returns a string representation of the Samplerequest
func (m *Samplerequest) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}