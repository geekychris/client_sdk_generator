package 

import (
	"encoding/json"
	"time"
)

// Graphqlrequest GraphQL request object
type Graphqlrequest struct {
	// GraphQL query string
Query String `json:"query"`
}

// NewGraphqlrequest creates a new Graphqlrequest with required fields
func NewGraphqlrequest(query String) *Graphqlrequest {
	return &Graphqlrequest{
		Query: query,
	}
}



// Validate validates the Graphqlrequest instance
func (m *Graphqlrequest) Validate() error {
	// Add validation logic here if needed
	// Query is required
	return nil
}

// String returns a string representation of the Graphqlrequest
func (m *Graphqlrequest) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}