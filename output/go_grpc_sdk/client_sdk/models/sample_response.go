package 

import (
	"encoding/json"
	"time"
)

// Sampleresponse Sample gRPC response
type Sampleresponse struct {
	// Response result
Result String `json:"result"`
}

// NewSampleresponse creates a new Sampleresponse with required fields
func NewSampleresponse(result String) *Sampleresponse {
	return &Sampleresponse{
		Result: result,
	}
}



// Validate validates the Sampleresponse instance
func (m *Sampleresponse) Validate() error {
	// Add validation logic here if needed
	// Result is required
	return nil
}

// String returns a string representation of the Sampleresponse
func (m *Sampleresponse) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}