import Base from "@/model/classes/Base";

export default class User extends Base {
  name?: string
  token?: string

  constructor(obj?: any | undefined) {
    super(obj)
    if (obj) {
      Object.assign(this, obj)
    }
  }
}
