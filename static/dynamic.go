// +build !bindata

package static

import "net/http"

func StaticFS() http.FileSystem {
	return http.Dir("./web/dist")
}
