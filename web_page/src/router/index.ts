import {createRouter, createWebHistory, NavigationGuardNext, RouteLocationNormalized, RouteRecordRaw} from "vue-router";
import {App} from "@vue/runtime-core";
import NProgress from "nprogress";
import "nprogress/nprogress.css";

NProgress.configure({showSpinner: true}); // 进度环显示/隐藏

export const constantRoutes: Array<RouteRecordRaw> = [
  {
    path: "/",
    component: () => import("@/views/index.vue"),
  },
  {
    path: "/info",
    component: () => import("@/views/info.vue"),
  },
  {
    path: "/dict",
    component: () => import("@/views/system/dict.vue"),
  },
  {
    path: "/test",
    component: () => import("@/views/test/test.vue"),
  },
  {
    path: "/404",
    component: () => import("@/views/error-page/404.vue"),
    meta: {hidden: true},
  },
  {
    path: "/401",
    component: () => import("@/views/error-page/401.vue"),
    meta: {hidden: true},
  }
];

// 创建路由
const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_APP_DOMAIN_CONTEXT),
  routes: constantRoutes as RouteRecordRaw[],
  // 刷新时，滚动条位置还原
  scrollBehavior: () => ({left: 0, top: 0}),
});


router.beforeEach(async (to: RouteLocationNormalized, form: RouteLocationNormalized, next: NavigationGuardNext) => {
  NProgress.start();
  next();
});


router.afterEach(() => {
  // const {tagsView} = useStore();
  // console.log(tagsView.cachedViews);
  NProgress.done();
});

export default router;
export const installRouter = (app: App) => {
  app.use(router);
};
