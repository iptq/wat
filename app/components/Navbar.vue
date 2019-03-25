<template>
	<div>
		<md-toolbar class="md-primary">
			<h3 class="md-title" id="title">WaT</h3>
			<md-button :to="{name: 'index'}">Home</md-button>
			<md-button :to="{name: 'register'}" v-if="registrationEnabled">Register</md-button>
			<md-button :to="{name: 'login'}">Login</md-button>
		</md-toolbar>
	</div>
</template>

<script>
	export default {
		props: ["registrationEnabled"],
	};
</script>

<style lang="scss" scoped>
	#title {
		flex: 1;

		a {
			color: white;
		}
	}
</style>