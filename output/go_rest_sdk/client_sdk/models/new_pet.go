package 

import (
	"encoding/json"
	"time"
)

// Newpet 
type Newpet struct {
	// Name of the pet
Name String `json:"name"`
	// Tag associated with the pet
Tag *String `json:"tag" omitempty`
	// URLs of pet photos
Photourls *String `json:"photoUrls" omitempty`
}

// NewNewpet creates a new Newpet with required fields
func NewNewpet(name String, ) *Newpet {
	return &Newpet{
		Name: name,
	}
}

// SetTag sets the tag field
func (m *Newpet) SetTag(tag String) *Newpet {
	m.Tag = &tag
	return m
}

// GetTag returns the tag field value if set, nil otherwise
func (m *Newpet) GetTag() *String {
	return m.Tag
}

// HasTag returns true if the tag field is set
func (m *Newpet) HasTag() bool {
	return m.Tag != nil
}

// SetPhotourls sets the photoUrls field
func (m *Newpet) SetPhotourls(photourls String) *Newpet {
	m.Photourls = &photourls
	return m
}

// GetPhotourls returns the photoUrls field value if set, nil otherwise
func (m *Newpet) GetPhotourls() *String {
	return m.Photourls
}

// HasPhotourls returns true if the photoUrls field is set
func (m *Newpet) HasPhotourls() bool {
	return m.Photourls != nil
}



// Validate validates the Newpet instance
func (m *Newpet) Validate() error {
	// Add validation logic here if needed
	// Name is required
	return nil
}

// String returns a string representation of the Newpet
func (m *Newpet) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}