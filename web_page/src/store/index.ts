import useGlobalStore from "./modules/global";

const useStore = () => ({
  global: useGlobalStore(),
})

export default useStore
