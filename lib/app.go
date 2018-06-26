package lib

import (
	"fmt"
	"log"
	"net/http"

	"github.com/go-xorm/core"
	"github.com/go-xorm/xorm"
	"github.com/gorilla/mux"
	"github.com/iptq/wat/lib/models"
	_ "github.com/mattn/go-sqlite3"
)

type App struct {
	engine *xorm.Engine
	router *mux.Router
}

func NewApp() *App {
	engine, err := xorm.NewEngine("sqlite3", "wat.db")
	if err != nil {
		log.Fatal(err)
	}
	engine.SetMapper(core.GonicMapper{})
	err = engine.Sync(new(models.User), new(models.Heartbeat))
	if err != nil {
		log.Fatal(err)
	}

	app := App{engine: engine, router: nil}
	app.router = app.Router()
	return &app
}

func (app *App) Start() {
	fmt.Println("Running...")
	log.Fatal(http.ListenAndServe(":8000", app.router))
}
