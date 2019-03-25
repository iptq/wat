import Vue from "vue";
import VueRouter from "vue-router";

import { http, router } from "./http";
import App from "./App";

import BootstrapVue from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(http);
Vue.use(BootstrapVue);

Vue.prototype.$extras = {
	apiBase: process.env.API_BASE || "http://localhost:6800",
};

new Vue({
	el: "#app",
	router,
	render: h => h(App),
});
