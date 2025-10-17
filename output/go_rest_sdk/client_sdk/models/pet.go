package 

import (
	"encoding/json"
	"time"
)

// Pet 
type Pet struct {
	// Unique identifier for the pet
Id String `json:"id"`
	// Name of the pet
Name String `json:"name"`
	// Tag associated with the pet
Tag *String `json:"tag" omitempty`
	// Pet status in the store
Status *String `json:"status" omitempty`
	// URLs of pet photos
Photourls *String `json:"photoUrls" omitempty`
}

// NewPet creates a new Pet with required fields
func NewPet(id String, name String, ) *Pet {
	return &Pet{
		Id: id,
		Name: name,
	}
}

// SetTag sets the tag field
func (m *Pet) SetTag(tag String) *Pet {
	m.Tag = &tag
	return m
}

// GetTag returns the tag field value if set, nil otherwise
func (m *Pet) GetTag() *String {
	return m.Tag
}

// HasTag returns true if the tag field is set
func (m *Pet) HasTag() bool {
	return m.Tag != nil
}

// SetStatus sets the status field
func (m *Pet) SetStatus(status String) *Pet {
	m.Status = &status
	return m
}

// GetStatus returns the status field value if set, nil otherwise
func (m *Pet) GetStatus() *String {
	return m.Status
}

// HasStatus returns true if the status field is set
func (m *Pet) HasStatus() bool {
	return m.Status != nil
}

// SetPhotourls sets the photoUrls field
func (m *Pet) SetPhotourls(photourls String) *Pet {
	m.Photourls = &photourls
	return m
}

// GetPhotourls returns the photoUrls field value if set, nil otherwise
func (m *Pet) GetPhotourls() *String {
	return m.Photourls
}

// HasPhotourls returns true if the photoUrls field is set
func (m *Pet) HasPhotourls() bool {
	return m.Photourls != nil
}



// Validate validates the Pet instance
func (m *Pet) Validate() error {
	// Add validation logic here if needed
	// Id is required
	// Name is required
	return nil
}

// String returns a string representation of the Pet
func (m *Pet) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}