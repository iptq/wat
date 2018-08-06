package lib

import (
	"github.com/gorilla/mux"
	"github.com/iptq/wat/static"
)

func (app *App) createRouter() *mux.Router {
	router := mux.NewRouter()
	router.Use(app.logMiddleware)

	api := router.PathPrefix("/api").Subrouter()
	api.Use(app.jsonMiddleware)

	api.HandleFunc("/config", app.publicConfig).Methods("GET")

	users := api.PathPrefix("/users").Subrouter()
	if app.config.RegistrationEnabled {
		users.HandleFunc("/register", app.handleUserRegister).Methods("POST")
	} else {
		users.HandleFunc("/register", app.error403).Methods("POST")
	}

	protected := users.PathPrefix("/current").Subrouter()
	protected.HandleFunc("/heartbeats.bulk", app.handleUserHeartbeat).Methods("POST")
	protected.Use(app.authMiddleware)

	router.PathPrefix("/").Handler(static.StaticFS())

	return router
}
