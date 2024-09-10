interface DefaultSettings {
  title: string;
  showSettings: boolean;
  tagsView: boolean;
  helpView: boolean;
  fixedHeader: boolean;
  sidebarLogo: boolean;
  errorLog: string;
  theme: string;
  style: string;
}

const defaultSettings: DefaultSettings = {
  title: "Stock Manage Platform",
  showSettings: false,
  tagsView: true,
  fixedHeader: false,
  theme: "#0085d7",
  helpView: false,
  // 是否显示Logo
  sidebarLogo: true,
  errorLog: "production",
  style: "",
};

export default defaultSettings;
