import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import {DictService} from "@/service/dict-service";
import {CARELINK_DICT_KEY, CONST_VAR} from "@/views/const";

export class SugarService extends BaseService {
  constructor() {
    super('/public/', '');
  }

  async loadData(mask) {
    let resultData: any = null
    const dictService = new DictService()
    if (!CONST_VAR.isDemo) {
      const result: any = await dictService.getDict(CARELINK_DICT_KEY.carelinkData, mask)
      if (result) {
        const data = JSON.parse(result)
        resultData = {
          data: JSON.parse(data.data),
          forecast: data.forecast,
          status: data.status
        }
        console.log(resultData);
      }
    } else {
      const result = await dictService.getDemo()
      if (result) {
        resultData = {
          ...result,
          status: 200
        }
      }
    }
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
