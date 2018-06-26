package lib

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/iptq/wat/lib/models"
	base58 "github.com/itchyny/base58-go"
	"github.com/thanhpk/randstr"
)

var encoding = base58.FlickrEncoding

func (app *App) HandleUserHeartbeat(w http.ResponseWriter, r *http.Request) {
	payload, _ := json.Marshal("shiet")
	w.Write(payload)
}

func (app *App) HandleUserRegister(w http.ResponseWriter, r *http.Request) {
	var err error

	// for now, just generate a user and return the API key
	key := randstr.Hex(64)
	log.Println(key)

	user := models.User{
		APIKey: key,
	}
	_, err = app.engine.Insert(&user)

	if err != nil {
		log.Println("failed to create user")
		return
	}
	payload, _ := json.Marshal(struct {
		Key string
	}{
		Key: key,
	})
	w.Write(payload)
}
