package lib

// Config describes the application configuration.
type Config struct {
	BindAddress string `description:"Address to bind to (ex. ':6800')"`
	Database    string `description:"Location for the database file (ex. 'wat.db')"`
}

// DefaultCfg lists some default configuration options.
// Using these options it should be possible to get the application running without any problems.
var DefaultCfg = Config{
	BindAddress: ":6800",
	Database:    "wat.db",
}
