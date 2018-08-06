package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/iptq/wat/lib"

	// include database backend drivers
	_ "github.com/go-sql-driver/mysql"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
)

func main() {
	config := lib.ReadConfig()
	app := lib.NewApp(config)
	go app.Start()

	// wait for signals
	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

	log.Println("shutting down")
	app.Close()
}
