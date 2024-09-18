const useGlobalStore = defineStore({
  id: "global",
  state: (): any => ({
    user: null,
    language: "zh-cn",
  }),
  actions: {
    setUser(user: any) {
      this.user = user
    },
    getUser() {
      return this.user
    },
    setLanguage(language: string) {
      this.language = language;
    },
  }
})

export default useGlobalStore;
