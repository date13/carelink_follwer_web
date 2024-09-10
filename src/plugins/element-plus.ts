import {App} from '@vue/runtime-core';
import {ElAside, ElContainer, ElDivider, ElFooter, ElHeader, ElMain, ElPopconfirm} from 'element-plus'


export default (app: App) => {
  app.use(ElAside)
  app.use(ElMain)
  app.use(ElAside)
  app.use(ElHeader)
  app.use(ElFooter)
  app.use(ElContainer)
  app.use(ElDivider)
  app.use(ElPopconfirm)
}
