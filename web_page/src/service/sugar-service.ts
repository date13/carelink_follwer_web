import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import {DictService} from "@/service/dict-service";
import {CONST_VAR} from "@/views/const";

export class SugarService extends BaseService {
  constructor() {
    super('/public/', '');
  }

  async loadData(mask) {
    let resultData: any = null
    const dictService = new DictService()
    if (!CONST_VAR.isDemo) {
      const result: any = await HttpClient.put(`${this.apiContext}sugar`, {
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
}
