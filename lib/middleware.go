package lib

import (
	"encoding/base64"
	"log"
	"net/http"
	"strings"

	"git.mzhang.me/wat/wat/lib/models"
)

func (app *App) authMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var user models.User
		var err error

		auth := r.Header.Get("Authorization")
		parts := strings.Split(auth, " ")
		if len(parts) < 2 {
			log.Println("parts:", parts)
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(""))
			return
		}
		decode, _ := base64.StdEncoding.DecodeString(parts[1])
		api_key := string(decode)
		has, err := app.engine.Where("api_key = ?", api_key).Get(&user)
		if err != nil {
			log.Println("error:", err)
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(""))
			return
		}
		if !has {
			log.Println("api key not found:", api_key)
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(""))
			return
		}
		log.Println("auth ok")
		next.ServeHTTP(w, r)
	})
}

func (app *App) jsonMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		next.ServeHTTP(w, r)
	})
}

func (app *App) logMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Println(r)
		next.ServeHTTP(w, r)
	})
}
