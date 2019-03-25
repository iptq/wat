<template>
	<div class="centered-container">
		<md-content class="md-elevation-3">
            <div class="form">
                <md-field>
                    <label>Username</label>
                    <md-input v-model="username" autofocus></md-input>
                </md-field>

                <md-field md-has-password>
                    <label>Password</label>
                    <md-input v-model="password" type="password"></md-input>
                </md-field>
            </div>

            <div class="actions md-layout">
                <!-- <a href="/resetpassword">Reset password</a> -->
                <md-button class="md-raised md-primary" @click="login">Log in</md-button>
            </div>

            <div class="loading-overlay" v-if="loading">
                <md-progress-spinner md-mode="indeterminate" :md-stroke="2"></md-progress-spinner>
            </div>
    	</md-content>
	</div>
</template>

<script>
    export default {
        data() {
            return {
                loading: false,
                username: "",
                password: "",
            };
        },
        methods: {
            login() {
                this.loading = true;
            }
        }
    }
</script>

<style lang="scss" scoped>
    .centered-container {
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        margin-top: 30px;


        .form {
            margin-bottom: 60px;
        }

        .md-content {
            z-index: 1;
            padding: 40px;
            width: 100%;
            max-width: 450px;
            position: relative;
        }

        .loading-overlay {
            z-index: 10;
            top: 0;
            left: 0;
            right: 0;
            position: absolute;
            width: 100%;
            height: 100%;
            background: rgba(255, 255, 255, 0.9);
            display: flex;
            align-items: center;
            justify-content: center;
        }
    }
</style>
