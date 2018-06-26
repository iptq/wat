package models

import "time"

type Heartbeat struct {
	Id       int64
	Entity   string
	Type     string
	Category string
	Time     time.Time

	Project  string
	Branch   string
	Language string
}
