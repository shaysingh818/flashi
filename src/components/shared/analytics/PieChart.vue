<template>
  <div
    class="flex centered-all rounded-2 p-6"
    style="background-color: rgb(28 25 23)"
  >
    <Pie :data="chartData" :options="chartOptions" />
  </div>
</template>

<script>
import { Pie } from "vue-chartjs";
import { Chart as ChartJS, ArcElement, Tooltip, Legend, Title } from "chart.js";

ChartJS.register(ArcElement, Tooltip, Legend, Title);

export default {
  name: "PieChart",
  components: { Pie },
  props: {
    title: {
      type: String,
      default: () => "Chart title",
    },
    labels: {
      type: Array,
      default: () => ["Open Ended", "Multiple Choice", "Ordered Sequence"],
    },
  },
  computed: {
    chartData() {
      return {
        labels: this.labels,
        datasets: [
          {
            label: "Question Types",
            data: [40, 20, 35],
            backgroundColor: ["#F97316", "#FB923C", "#FDBA74"],
            border: 'none'
          },
        ],
      };
    },
    chartOptions() {
      return {
        responsive: true,
        plugins: {
          title: {
            display: true,
            text: this.title,
            color: "#ffffff",
            font: { size: 16, weight: "bold" },
            padding: { bottom: 10 }, 
          },
          legend: {
            position: "bottom",
            labels: { color: "#ffffff" },
          },
        },
      };
    },
  },
};
</script>
