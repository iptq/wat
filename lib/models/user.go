package models

import "time"

type User struct {
	Id          int64
	DisplayName string
	FullName    string
	Email       string
	APIKey      string

	EmailPublic    bool
	EmailConfirmed bool
	Timezone       string
	LastHeartbeat  time.Time
	LastPlugin     string
	LastPluginName string
	LastProject    string

	Username string
	Website  string
	Location string

	LoggedTimePublic    bool
	LanguagesUsedPublic bool

	RegisteredAt time.Time
	ModifiedAt   time.Time
}
