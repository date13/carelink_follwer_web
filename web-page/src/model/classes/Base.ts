export default class Base {
  id: any = null
  creationDate = ''
  createdBy = ''
  lastUpdateDate = ''
  lastUpdateBy = ''

  constructor(obj?: any | undefined) {
    if (obj) {
      Object.assign(this, obj)
    }
  }

  convert(...params: any) {
    return this
  }

  parseField() {
  }
}
