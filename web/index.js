// heavily based on https://github.com/prograhammer/vue-pizza

import Vue from "vue";
import VueRouter from "vue-router";
import Vuikit from "vuikit";

import "@vuikit/theme";
import VuikitIcons from "@vuikit/icons";

import { http, router } from "./http";
import App from "./App";

Vue.use(http);

Vue.use(Vuikit);
Vue.use(VuikitIcons);

Vue.prototype.$extras = {
	apiBase: process.env.API_BASE || "http://localhost:6800",
};

new Vue({
	el: "#app",
	router,
	render: h => h(App),
});
