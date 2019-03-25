<template>
	<div>
		<md-empty-state
			md-icon="person"
			md-label="Not Logged In"
			md-description="Login or create an account to get started.">
			<md-button class="md-primary md-raised" :to="{name: 'login'}">Login</md-button>
		</md-empty-state>
	</div>
</template>

<script>
</script>
