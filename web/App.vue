<template>
	<div>
		<navbar :registrationEnabled="siteConfig.registrationEnabled"></navbar>
		<router-view></router-view>
	</div>
</template>

<script>
	import Navbar from "~/components/Navbar";

	export default {
		components: { Navbar },
		data() {
			return {
				siteConfig: {
					registrationEnabled: false,
				}
			};
		},
		created() {
			this.fetchConfig();
		},
		methods: {
			fetchConfig() {
				this.$http.get(`${this.$extras.apiBase}/config`).then(result => {
					this.siteConfig = result.data;
				});
			}
		}
	};
</script>
