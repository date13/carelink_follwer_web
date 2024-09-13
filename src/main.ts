import {createApp} from 'vue'
import installDirective from './directive'
import installFilter from './filter/index'
import {installRouter} from './router'
import {createPinia} from "pinia";
import App from './App.vue'
// 自定义样式
import 'uno.css'
// import 'element-plus/theme-chalk/src/index.scss'
import '@/styles/index.scss'

const app = createApp(App)
createPinia().install(app)
installRouter(app)
installDirective(app)
installFilter(app)
// installElementPlus(app)
app.mount('#app')
