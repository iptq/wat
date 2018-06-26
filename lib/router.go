package lib

import (
	"github.com/gorilla/mux"
	"github.com/iptq/wat/lib/handlers"
)

func Router() *mux.Router {
	router := mux.NewRouter()
	router.Use(logMiddleware)

	api := router.PathPrefix("/api/v1").Subrouter()
	api.Use(jsonMiddleware)

	users := api.PathPrefix("/users").Subrouter()
	protected := users.PathPrefix("/current").Subrouter()
	protected.HandleFunc("/heartbeats.bulk", handlers.PostHeartbeat).Methods("POST")
	protected.Use(authMiddleware)

	return router
}
