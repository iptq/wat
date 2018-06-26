package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/iptq/wat/lib"
	"github.com/thehowl/conf"

	_ "github.com/mattn/go-sqlite3"
)

func main() {
	configFile := flag.String("conf", "wat.conf", "config file location")
	flag.Parse()

	config := lib.DefaultCfg
	err := conf.Load(&config, *configFile)
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
