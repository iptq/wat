package models

import "time"

// Heartbeat describes a single capture from an application.
type Heartbeat struct {
	ID       int64
	Entity   string
	Type     string
	Category string
	Time     time.Time

	Project  string
	Branch   string
	Language string

	Lines     int
	LineNo    int
	CursorPos int
	IsWrite   bool
}
