import {App} from "@vue/runtime-core";
import * as directive from '@/directive'
import {Directive} from "vue";


export default (app: App) => {

  Object.keys(directive).forEach(key => {
    app.directive(key, (directive as { [key: string]: Directive })[key]);
  });
}
