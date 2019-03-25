import Vue from "vue";
import Router from "vue-router";

import routes from "./routes";

Vue.use(Router);

export default new Router({
	mode: "history",
	routes: routes.map(route => ({
		name: route.name,
		path: route.path,
		component: route.component,
		beforeEnter: (to, from, next) => {
			document.title = route.title;
			next();
		},
	})),
});
