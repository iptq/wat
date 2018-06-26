package lib

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/go-xorm/core"
	"github.com/go-xorm/xorm"
	"github.com/gorilla/mux"
	"github.com/iptq/wat/lib/models"
	_ "github.com/mattn/go-sqlite3"
)

type App struct {
	config Config
	engine *xorm.Engine
	router *mux.Router
	srv    *http.Server
}

func NewApp(config Config) *App {
	engine, err := xorm.NewEngine("sqlite3", "wat.db")
	if err != nil {
		log.Fatal(err)
	}
	engine.SetMapper(core.GonicMapper{})
	err = engine.Sync(new(models.User), new(models.Heartbeat))
	if err != nil {
		log.Fatal(err)
	}

	app := App{config: config, engine: engine, router: nil, srv: nil}
	app.router = app.Router()
	return &app
}

func (app *App) Start() {
	fmt.Printf("Running on '%s'...\n", app.config.BindAddress)
	app.srv = &http.Server{
		Addr:    app.config.BindAddress,
		Handler: app.router,
	}
	if err := app.srv.ListenAndServe(); err != nil {
		log.Println(err)
	}
}

func (app *App) Close() {
	var wait time.Duration
	ctx, cancel := context.WithTimeout(context.Background(), wait)
	defer cancel()
	app.srv.Shutdown(ctx)
}
