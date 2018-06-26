package lib

// Config describes the application configuration.
type Config struct {
	BindAddress      string `description:"Address to bind to (ex. ':6800')"`
	DatabaseProvider string `description:"Backend to use for the database { mysql | postgres | sqlite3 }"`
	Database         string `description:"Location for the database file (ex. 'wat.db')"`

	RedisAddress string `description:"The address of the redis server."`
	RedisDb      int    `description:"The number of the redis db."`
}

// DefaultCfg lists some default configuration options.
// Using these options it should be possible to get the application running without any problems.
var DefaultCfg = Config{
	BindAddress:      ":6800",
	DatabaseProvider: "sqlite3",
	Database:         "wat.db",

	RedisAddress: "localhost:6379",
	RedisDb:      0,
}
