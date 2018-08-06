<template>
	<div>
		<vk-navbar>
			<vk-navbar-nav>
				<vk-navbar-logo id="logo"><router-link tag="li" to="/">WaT</router-link></vk-navbar-logo>
				<router-link tag="li" :to="{ name: 'login' }"><a>Features</a></router-link>
			</vk-navbar-nav>
			<vk-navbar-nav slot="right">
				<router-link tag="li" :to="{ name: 'login' }"><a>Login</a></router-link>
				<router-link tag="li" :to="{ name: 'register' }" v-if="registrationEnabled"><a>Register</a></router-link>
			</vk-navbar-nav>
		</vk-navbar>
	</div>
</template>

<script>
	export default {
		props: ["registrationEnabled"],
	};
</script>

<style>
	#logo {
		cursor: pointer;
	}
</style>