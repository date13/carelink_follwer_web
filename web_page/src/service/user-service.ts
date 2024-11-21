import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";

export class UserService extends BaseService {
  constructor() {
    super('/user/', '');
  }

  login(params: any) {
    return HttpClient.postBody(`${this.apiContext}login`, {
      body: params
    })
  }
}
