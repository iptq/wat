import Vue from "vue";
import VueRouter from "vue-router";

import { http, router } from "./http";
import App from "./App";

import VueMaterial from "vue-material";
import "vue-material/dist/vue-material.min.css";
import "vue-material/dist/theme/default.css";

Vue.use(http);
Vue.use(VueMaterial);

Vue.prototype.$extras = {
	apiBase: process.env.API_BASE || "http://localhost:6800",
};

new Vue({
	el: "#app",
	router,
	render: h => h(App),
});
