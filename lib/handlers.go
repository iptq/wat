package lib

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/iptq/wat/lib/models"
	"github.com/thanhpk/randstr"
)

func (app *App) HandleUserHeartbeat(w http.ResponseWriter, r *http.Request) {
	payload, _ := json.Marshal("shiet")
	w.Write(payload)
}

func (app *App) HandleUserRegister(w http.ResponseWriter, r *http.Request) {
	var err error

	// for now, just generate a user and return the API key
	// note: what the fuck
	p1 := randstr.Hex(2)
	p2 := randstr.Hex(2)
	key := randstr.Hex(4) + "-" + randstr.Hex(2) + "-4" + p1[1:] + "-8" + p2[1:] + "-" + randstr.Hex(6)

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
