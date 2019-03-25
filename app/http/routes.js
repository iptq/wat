import PageIndex from "../pages/Index";
import PageLogin from "../pages/Login";

export default [
	{
		path: "/",
		name: "index",
		component: PageIndex,
		title: "Home",
	},
	{
		path: "/login",
		name: "login",
		component: PageLogin,
		title: "Login",
	},
	{
		path: "/register",
		name: "register",
		component: PageIndex,
		title: "Register",
	},
];