import {BaseService} from './base-service'
import {API_URL, HttpClient} from "@/utils/http-client";
import {DictService} from "@/service/dict-service";
import {CONST_VAR} from "@/views/const";
import {EventStreamContentType, fetchEventSource} from '@microsoft/fetch-event-source';
import {Msg, Tools} from "@/utils/tools";
import router from "@/router";

export class SugarService extends BaseService {
  constructor() {
    super('/sugar/', '');
  }

  async loadData(mask) {
    let resultData: any = null
    const dictService = new DictService()
    if (!CONST_VAR.isDemo) {
      const result: any = await HttpClient.put(`${this.apiContext}`, {
        mask
      })
      if (result) {
        const data = JSON.parse(result.data)
        const myData = JSON.parse(result.myData)
        resultData = {
          ...data,
          myData
        }
      }
    } else {
      const result = await dictService.getDemo()
      if (result) {
        resultData = {
          ...result
        }
      }
    }
    console.log(resultData);
    return resultData
  }

  async initSugarSSE(callback = (res) => {
  }) {
    const user = Tools.getUser();
    await fetchEventSource(`${API_URL}sugar/sse`, {
      method: 'put',
      headers: {
        Authorization: `${user ? user.token : ''}`,
        'Content-Type': 'application/json',
      },
      openWhenHidden: true,
      async onopen(response) {
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
      },
      onmessage(msg) {
        try {
          if (msg.data) {
            const data = JSON.parse(msg.data)
            callback({
              ...data.data,
              myData: data.myData
            })
          }
        } catch (e) {
          console.log(`sse数据解析错误:${e}`)
        }
      },
      onclose() {
        // if the server closes the connection unexpectedly, retry:
        console.log("sse close");
      },
      onerror(err) {
        console.error(`sse err:${err}`);
      }
    });
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
}
