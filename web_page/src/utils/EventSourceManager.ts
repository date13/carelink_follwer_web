import {fetchEventSource} from "@microsoft/fetch-event-source";

export class EventSourceManager {
  url
  options
  controller
  isConnected
  onMessage
  onError
  onOpen

  constructor(url, options = {}) {
    this.url = url;
    this.options = options;
    this.controller = null;
    this.isConnected = false;
    this.onMessage = null;
    this.onError = null;
    this.onOpen = null;
  }

  async start() {
    // 先停止现有的连接
    this.stop();

    this.controller = new AbortController();
    this.isConnected = true;

    try {
      await fetchEventSource(this.url, {
        ...this.options,
        signal: this.controller.signal,
        onopen: async (response) => {
          console.log('EventSource 连接已建立');
          this.isConnected = true;
          if (this.onOpen) this.onOpen(response);
        },
        onmessage: (event) => {
          if (this.onMessage) this.onMessage(event);
        },
        onerror: (error) => {
          console.error('EventSource 错误:', error);
          this.isConnected = false;
          if (this.onError) this.onError(error);

          // 不抛出错误，避免自动重连
          return;
        },
        onclose: () => {
          console.log('EventSource 连接已关闭');
          this.isConnected = false;
        }
      });
    } catch (error: any) {
      if (error.name !== 'AbortError') {
        console.error('EventSource 启动失败:', error);
        this.isConnected = false;
      }
    }
  }

  stop() {
    if (this.controller) {
      this.controller.abort();
      this.controller = null;
      this.isConnected = false;
      console.log('EventSource 已停止');
    }
  }

  restart() {
    console.log('重启 EventSource...');
    this.start();
  }

  // 设置回调函数
  setCallbacks({onMessage, onError, onOpen}) {
    this.onMessage = onMessage;
    this.onError = onError;
    this.onOpen = onOpen;
  }
}
