package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// Pet Store API Go SDK Client
//
// A sample API that uses a pet store as an example
type PetStoreApiclient struct {
	httpClient *http.Client
	baseURL    string
}

// ClientConfig holds client configuration
type ClientConfig struct {
	BaseURL    string
	HTTPClient *http.Client
	Timeout    time.Duration
}

// NewPetStoreApiclient creates a new instance of PetStoreApiclient
func NewPetStoreApiclient(config ClientConfig) *PetStoreApiclient {
	if config.HTTPClient == nil {
		config.HTTPClient = &http.Client{
			Timeout: config.Timeout,
		}
		if config.Timeout == 0 {
			config.HTTPClient.Timeout = 30 * time.Second
		}
	}

	return &PetStoreApiclient{
		httpClient: config.HTTPClient,
		baseURL:    strings.TrimRight(config.BaseURL, "/"),
	}
}

// makeRequest performs an HTTP request
func (c *PetStoreApiclient) makeRequest(ctx context.Context, method, path string, body interface{}, params map[string]string) (*http.Response, error) {
	reqURL := fmt.Sprintf("%s%s", c.baseURL, path)
	
	// Add query parameters
	if len(params) > 0 {
		u, err := url.Parse(reqURL)
		if err != nil {
			return nil, fmt.Errorf("invalid URL: %w", err)
		}
		
		q := u.Query()
		for key, value := range params {
			q.Add(key, value)
		}
		u.RawQuery = q.Encode()
		reqURL = u.String()
	}

	var reqBody io.Reader
	if body != nil {
		jsonData, err := json.Marshal(body)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request body: %w", err)
		}
		reqBody = bytes.NewBuffer(jsonData)
	}

	req, err := http.NewRequestWithContext(ctx, method, reqURL, reqBody)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	// Set headers
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Accept", "application/json")
	req.Header.Set("User-Agent", "client/1.0.0 (Go)")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("request failed: %w", err)
	}

	return resp, nil
}