package lib

import (
	"fmt"

	"github.com/spf13/viper"
)

// Config describes the application configuration.
type Config struct {
	BindAddress      string
	DatabaseProvider string
	Database         string

	RegistrationEnabled bool
}

func ReadConfig() Config {
	// no global pls
	c := viper.New()

	// default configuration options
	c.SetDefault("bind_address", ":6800")
	c.SetDefault("database_provider", "sqlite3")
	c.SetDefault("database", "wat.db")
	c.SetDefault("registration_enabled", false)

	// set up search locations
	c.SetEnvPrefix("wat")
	c.BindEnv("bind_address")
	c.BindEnv("database_provider")
	c.BindEnv("database")
	c.BindEnv("registration_enabled")

	c.SetConfigName("wat")
	c.AddConfigPath(".")

	// read config
	err := c.ReadInConfig()
	if err != nil {
		panic(err)
	}

	// TODO: debug
	fmt.Printf("%+v\n", c.AllKeys())
	return Config{
		BindAddress:      c.GetString("bind_address"),
		DatabaseProvider: c.GetString("database_provider"),
		Database:         c.GetString("database"),

		RegistrationEnabled: c.GetBool("registration_enabled"),
	}
}
