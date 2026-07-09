import { ref } from "vue";

export default function (chart) {
  // const chart = ref<any>()
  const _chart = chart;
  const sidebarElm = ref<Element>();
  const appMainElm = ref<Element>();
  let resizeTimer: any = null;
  const chartResizeHandler = () => {
    if (_chart) {
      if (resizeTimer) {
        clearTimeout(resizeTimer);
      }
      resizeTimer = setTimeout(() => {
        _chart.resize();
      }, 100);
    }
  };

  const sidebarResizeHandler = (e: TransitionEvent) => {
    if (e.propertyName === "width") {
      chartResizeHandler();
    }
  };

  const appMainResizeHandler = (e: TransitionEvent) => {
    if (e.propertyName === "width") {
      chartResizeHandler();
    }
  };

  const initResizeEvent = () => {
    window.addEventListener("resize", chartResizeHandler);
  };

  const destroyResizeEvent = () => {
    window.removeEventListener("resize", chartResizeHandler);
    if (resizeTimer) {
      clearTimeout(resizeTimer);
      resizeTimer = null;
    }
  };

  const initSidebarResizeEvent = () => {
    sidebarElm.value = document.getElementsByClassName("sidebar-container")[0];
    if (sidebarElm.value) {
      sidebarElm.value.addEventListener(
        "transitionend",
        sidebarResizeHandler as EventListener,
      );
    }
  };

  const initAppMainResizeEvent = () => {
    appMainElm.value = document.getElementsByClassName("app-main-wrapper")[0];
    if (appMainElm.value) {
      appMainElm.value.addEventListener(
        "transitionend",
        appMainResizeHandler as EventListener,
      );
    }
  };
  const destroySidebarResizeEvent = () => {
    if (sidebarElm.value) {
      sidebarElm.value.removeEventListener(
        "transitionend",
        sidebarResizeHandler as EventListener,
      );
    }
  };

  const destroyAppMainResizeEvent = () => {
    if (appMainElm.value) {
      appMainElm.value.removeEventListener(
        "transitionend",
        appMainResizeHandler as EventListener,
      );
    }
  };

  const mounted = () => {
    initResizeEvent();
    initSidebarResizeEvent();
    initAppMainResizeEvent();
  };

  const beforeDestroy = () => {
    destroyResizeEvent();
    destroySidebarResizeEvent();
    destroyAppMainResizeEvent();
  };

  const activated = () => {
    initResizeEvent();
    initSidebarResizeEvent();
    initAppMainResizeEvent();
  };

  const deactivated = () => {
    destroyResizeEvent();
    destroySidebarResizeEvent();
    destroyAppMainResizeEvent();
  };

  return {
    chart: _chart,
    mounted,
    beforeDestroy,
    activated,
    deactivated,
  };
}
