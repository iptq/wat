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
)

// App describes a WaT application, and holds some of its necessary components.
type App struct {
	config Config
	engine *xorm.Engine
	router *mux.Router
	srv    *http.Server
}

// NewApp constructs a WaT application and returns a pointer to it.
func NewApp(config Config) *App {
	engine, err := xorm.NewEngine("sqlite3", config.Database)
	if err != nil {
		log.Fatal(err)
	}
	engine.SetMapper(core.GonicMapper{})
	err = engine.Sync(new(models.User), new(models.Heartbeat))
	if err != nil {
		log.Fatal(err)
	}

	app := App{config: config, engine: engine, router: nil, srv: nil}
	app.router = app.createRouter()
	return &app
}

// Start runs the given WaT App. This function should be run in a thread if possible.
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

// Close gracefully shuts down the server.
func (app *App) Close() {
	var wait time.Duration
	ctx, cancel := context.WithTimeout(context.Background(), wait)
	defer cancel()
	app.srv.Shutdown(ctx)
}
