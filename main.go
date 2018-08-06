package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/iptq/wat/lib"
	"github.com/spf13/viper"
	"github.com/thehowl/conf"

	// include database backend drivers
	_ "github.com/go-sql-driver/mysql"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
)

func main() {
	viper.SetDefault("BindAddress", ":6800")
	viper.SetConfigName("wat")
	viper.AddConfigPath(".")
	err := viper.ReadInConfig()
	if err != nil {
		panic(err)
	}

	fmt.Printf("%+v", viper.AllKeys())

	configFile := flag.String("conf", "wat.conf", "config file location")
	flag.Parse()

	config := lib.DefaultCfg
	err = conf.Load(&config, *configFile)
	if err == conf.ErrNoFile {
		conf.Export(lib.DefaultCfg, *configFile)
		fmt.Println("Default configuration written to " + *configFile)
		os.Exit(0)
	}

	app := lib.NewApp(config)
	go app.Start()

	// wait for signals
	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

	log.Println("shutting down")
	app.Close()
}
