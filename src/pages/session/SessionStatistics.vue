<template>
  <div class="flex col-center w-100">
    <div class="grid cols-3 w-95 gap-6 place-center p-6">
      <MetricCard metric="Average Accuracy" numeric="20" />
      <MetricCard metric="Total Questions Reviewed" numeric="20" />
      <MetricCard metric="Total Review Time" numeric="20" />
    </div>

    <div class="grid cols-2 cols-responsive w-95 gap-4">
      <DoughnutChart title="Question accuracy ratio" />
      <PieChart title="Question type breakdown" />
    </div>

    <br />
    <div class="grid cols-1 cols-responsive w-95 gap-4">
      <LineChart title="Historical review metrics" />
    </div>

    <br />

    <div class="grid cols-2 cols-responsive w-95 gap-4">
      <DataView title="Most incorrect questions" />
      <DataView title="Most correct question" />
    </div>

    <br />
    <br />
  </div>
</template>

<script>
import MetricCard from "../../components/shared/analytics/MetricCard.vue";
import DoughnutChart from "../../components/shared/analytics/DoughnutChart.vue";
import PieChart from "../../components/shared/analytics/PieChart.vue";
import BarChart from "../../components/shared/analytics/BarChart.vue";
import LineChart from "../../components/shared/analytics/LineChart.vue";
import DataView from "../../components/shared/analytics/DataView.vue";

export default {
  name: "SessionStatistics",
  components: {
    MetricCard,
    DoughnutChart,
    PieChart,
    BarChart,
    LineChart,
    DataView,
  },
  methods: {
    test() {
      console.log("test");
    },
  },
};
</script>
