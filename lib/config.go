package lib

import (
	"fmt"

	"github.com/spf13/viper"
)

// Config describes the application configuration.
type Config struct {
	BindAddress      string `description:"Address to bind to (ex. ':6800')"`
	DatabaseProvider string `description:"Backend to use for the database { mysql | postgres | sqlite3 }"`
	Database         string `description:"Access string for the database (ex. 'wat.db' for sqlite3, 'postgres://host:port/dbname' for postgres, etc.)"`
}

func ReadConfig() Config {
	// no global pls
	c := viper.New()

	// default configuration options
	c.SetDefault("bind_address", ":6800")
	c.SetDefault("database_provider", "sqlite3")
	c.SetDefault("database", "wat.db")

	// set up search locations
	c.SetEnvPrefix("wat")
	c.BindEnv("bind_address")
	c.BindEnv("database_provider")
	c.BindEnv("database")

	c.SetConfigName("wat")
	c.AddConfigPath(".")

	// read config
	err := c.ReadInConfig()
	if err != nil {
		panic(err)
	}

	fmt.Printf("%+v\n", c.AllKeys())
	return Config{
		BindAddress:      c.Get("bind_address").(string),
		DatabaseProvider: c.Get("database_provider").(string),
		Database:         c.Get("database").(string),
	}
}
