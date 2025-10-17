package 

import (
	"encoding/json"
	"time"
)

// Category 
type Category struct {
	// Category identifier
Id *String `json:"id" omitempty`
	// Category name
Name *String `json:"name" omitempty`
}

// NewCategory creates a new Category with required fields
func NewCategory() *Category {
	return &Category{
	}
}

// SetId sets the id field
func (m *Category) SetId(id String) *Category {
	m.Id = &id
	return m
}

// GetId returns the id field value if set, nil otherwise
func (m *Category) GetId() *String {
	return m.Id
}

// HasId returns true if the id field is set
func (m *Category) HasId() bool {
	return m.Id != nil
}

// SetName sets the name field
func (m *Category) SetName(name String) *Category {
	m.Name = &name
	return m
}

// GetName returns the name field value if set, nil otherwise
func (m *Category) GetName() *String {
	return m.Name
}

// HasName returns true if the name field is set
func (m *Category) HasName() bool {
	return m.Name != nil
}



// Validate validates the Category instance
func (m *Category) Validate() error {
	// Add validation logic here if needed
	return nil
}

// String returns a string representation of the Category
func (m *Category) String() string {
	data, _ := json.MarshalIndent(m, "", "  ")
	return string(data)
}