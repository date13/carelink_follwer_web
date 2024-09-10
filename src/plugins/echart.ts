import * as echarts from "echarts/core";

/** 引入柱状图and折线图图表，图表后缀都为 Chart  */
import {BarChart, LineChart, PieChart, ScatterChart} from "echarts/charts";
// 引入提示框，标题，直角坐标系，数据集，内置数据转换器组件，组件后缀都为 Component
import {
  DatasetComponent,
  DataZoomComponent,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
  GridComponent,
  LegendComponent,
  MarkLineComponent,
  TitleComponent,
  ToolboxComponent,
  TooltipComponent,
  TransformComponent,
} from "echarts/components";
// 标签自动布局，全局过渡动画等特性
import {LabelLayout, UniversalTransition} from "echarts/features";

// 引入 Canvas 渲染器，注意引入 CanvasRenderer 或者 SVGRenderer 是必须的一步
import {CanvasRenderer} from "echarts/renderers";

// 注册必须的组件
echarts.use([
  TitleComponent,
  TooltipComponent,
  ToolboxComponent,
  GridComponent,
  DatasetComponent,
  TransformComponent,
  LabelLayout,
  MarkLineComponent,
  UniversalTransition,
  CanvasRenderer,
  LegendComponent,
  DataZoomComponent,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
  BarChart,
  LineChart,
  PieChart,
  ScatterChart
]);

// 导出
export default echarts;
