package lib

type Config struct {
	BindAddress string `description:"Address to bind to (ex. ':6800')"`
	Database    string `description:"Location for the database file (ex. 'wat.db')"`
}

var DefaultCfg = Config{
	BindAddress: ":6800",
	Database:    "wat.db",
}
