package 

import (
	"encoding/json"
	"time"
)

// Graphqlresponse GraphQL response object
type Graphqlresponse struct {
	// Response data
Data *String `json:"data" omitempty`
	
	// AdditionalProperties holds additional properties not defined in the schema
	AdditionalProperties map[string]interface{} `json:"-"`
}

// NewGraphqlresponse creates a new Graphqlresponse with required fields
func NewGraphqlresponse() *Graphqlresponse {
	return &Graphqlresponse{
		AdditionalProperties: make(map[string]interface{}),
	}
}

// SetData sets the data field
func (m *Graphqlresponse) SetData(data String) *Graphqlresponse {
	m.Data = &data
	return m
}

// GetData returns the data field value if set, nil otherwise
func (m *Graphqlresponse) GetData() *String {
	return m.Data
}

// HasData returns true if the data field is set
func (m *Graphqlresponse) HasData() bool {
	return m.Data != nil
}


// SetAdditionalProperty sets an additional property
func (m *Graphqlresponse) SetAdditionalProperty(key string, value interface{}) *Graphqlresponse {
	if m.AdditionalProperties == nil {
		m.AdditionalProperties = make(map[string]interface{})
	}
	m.AdditionalProperties[key] = value
	return m
}

// GetAdditionalProperty gets an additional property
func (m *Graphqlresponse) GetAdditionalProperty(key string) (interface{}, bool) {
	if m.AdditionalProperties == nil {
		return nil, false
	}
	value, exists := m.AdditionalProperties[key]
	return value, exists
}

// MarshalJSON implements json.Marshaler to handle additional properties
func (m *Graphqlresponse) MarshalJSON() ([]byte, error) {
	type Alias Graphqlresponse
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(m),
	}
	
	result, err := json.Marshal(aux)
	if err != nil {
		return nil, err
	}
	
	if m.AdditionalProperties != nil && len(m.AdditionalProperties) > 0 {
		var base map[string]interface{}
		if err := json.Unmarshal(result, &base); err != nil {
			return nil, err
		}
		
		for k, v := range m.AdditionalProperties {
			base[k] = v
		}
		
		return json.Marshal(base)
	}
	
	return result, nil
}

// UnmarshalJSON implements json.Unmarshaler to handle additional properties
func (m *Graphqlresponse) UnmarshalJSON(data []byte) error {
	type Alias Graphqlresponse
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(m),
	}
	
	if err := json.Unmarshal(data, aux); err != nil {
		return err
	}
	
	var raw map[string]interface{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	
	// Remove known fields from raw map
	delete(raw, "data")
	
	// Store remaining fields as additional properties
	if len(raw) > 0 {
		m.AdditionalProperties = raw
	}
	
	return nil
}

// Validate validates the Graphqlresponse instance
func (m *Graphqlresponse) Validate() error {
	// Add validation logic here if needed
	return nil
}

// String returns a string representation of the Graphqlresponse
func (m *Graphqlresponse) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}