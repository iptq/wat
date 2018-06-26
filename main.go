package main

import (
	"log"
	"net/http"

	"github.com/gorilla/mux"
	"github.com/iptq/wat/lib"
)

func main() {
	router := mux.NewRouter()

	protected := router.PathPrefix("").Subrouter()
	protected.Use(lib.AuthMiddleware)

	log.Fatal(http.ListenAndServe(":8000", router))
}
