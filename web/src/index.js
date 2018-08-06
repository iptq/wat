// heavily based on https://github.com/prograhammer/vue-pizza

import '@vuikit/theme';

import VuikitIcons from '@vuikit/icons';
import Vue from 'vue';
import VueRouter from 'vue-router';
import Vuikit from 'vuikit';

import App from './App';
import {http, router} from './http';

Vue.use(http);

Vue.use(Vuikit);
Vue.use(VuikitIcons);

Vue.prototype.$extras = {
    apiBase: process.env.API_BASE || 'http://localhost:6800',
};

new Vue({
    el: '#app',
    router,
    render: h => h(App),
});
