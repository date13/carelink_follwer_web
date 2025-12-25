import {BaseService} from './base-service'
import {API_URL, HttpClient} from "@/utils/http-client";
import {EventStreamContentType} from '@microsoft/fetch-event-source';
import {Msg, Tools} from "@/utils/tools";
import router from "@/router";
import {EventSourceManager} from "@/utils/EventSourceManager";

export class SugarService extends BaseService {
  eventSourceManager: EventSourceManager

  constructor() {
    super('/sugar/', '');

    // 使用示例
    const user = Tools.getUser();
    this.eventSourceManager = new EventSourceManager(`${API_URL}sugar/sse`, {
      method: 'put',
      headers: {
        Authorization: `${user ? `Bearer ${user.token}` : ''}`,
        'Content-Type': 'application/json',
      },
      openWhenHidden: false,
    });
  }

  async loadData() {
    const result = await HttpClient.put(`${this.apiContext}index`, {
      mask: true
    })
    if (result) {
      return {
        ...result.data,
        myData: result.myData
      }
    }
  }

  async initSugarSSE(callback = (res) => {
  }) {
    this.eventSourceManager.setCallbacks({
      onMessage: (event) => {
        try {
          if (event.data) {
            const data = JSON.parse(event.data)
            callback({
              ...data.data,
              myData: data.myData
            })
          }
        } catch (e) {
          console.log(`sse数据解析错误:`, e)
        }
        // 处理消息
      },
      onError: (error) => {
        console.error(`sse err:${error}`);
      },
      onOpen: (response) => {
        if (response.ok && response.headers.get('content-type') === EventStreamContentType) {
          return; // everything's good
        } else if (response.status >= 400 && response.status < 500 && response.status !== 429) {
          // client-side errors are usually non-retriable:
          console.error(`sse status err:${response.status}`);
          if (response.status === 401) {
            Msg.alert(`用户登陆 token 已失效,请重新登录`, () => {
              const fullpath = location.href.substring(location.href.indexOf(location.pathname))
              router.push(`/login?redirect?=${fullpath}`)
            })
          }
        } else {
          console.log(`sse连接打开`, response)
        }
      }
    });
    // 启动连接
    this.eventSourceManager.start();
  }

  SSERestart() {
    this.eventSourceManager.restart();
  }

  refreshCarelinkToken() {
    return HttpClient.put(`${this.apiContext}refreshCarelinkToken`, {
      mask: true
    })
  }

  refreshCarelinkData() {
    return HttpClient.put(`${this.apiContext}refreshCarelinkData`, {
      mask: true
    })
  }

  loadFood() {
    return HttpClient.get(`${this.apiContext}foods`, {})
  }

  updateFood(params) {
    return HttpClient.putBody(`${this.apiContext}food`, {
      body: params
    })
  }

  delFood(key) {
    return HttpClient.destroy(`${this.apiContext}food/${key}`,)
  }

  refreshDexcomToken() {
    return HttpClient.get(`${this.apiContext}dexcomAuthRefresh/${Tools.getUser().name}`, {})
  }

  refreshDexcomData() {
    return HttpClient.get(`${this.apiContext}dexcomData/${Tools.getUser().name}`, {})
  }
}
