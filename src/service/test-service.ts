import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";

export class TestService extends BaseService {
  constructor() {
    super('/public/', '');
  }

  //手机密码登录
  testHelloWorld() {
    return HttpClient.get(`${this.apiContext}test`, {});
  }

  luck() {
    return HttpClient.get(`${this.apiContext}luck`, {})
  }

  luckPush(params) {
    return HttpClient.postBody(`${this.apiContext}luck_push`, {
      body: params
    })
  }

}
