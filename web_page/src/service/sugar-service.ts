import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import {DictService} from "@/service/dict-service";
import {CONST_VAR} from "@/views/const";

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
