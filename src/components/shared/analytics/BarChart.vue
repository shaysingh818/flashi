<template>
  <div
    class="flex centered-all rounded-2 p-6"
    style="background-color: rgb(28 25 23)"
  >
    <Bar :data="chartData" :options="chartOptions" />
  </div>
</template>

<script>
import { Bar } from "vue-chartjs";
import { Chart as ChartJS, BarElement, CategoryScale, LinearScale, Tooltip, Legend, Title } from "chart.js";

ChartJS.register(BarElement, CategoryScale, LinearScale, Tooltip, Legend, Title);

export default {
  name: "BarChart",
  components: { Bar },
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
            data: [60, 100, 35],
            backgroundColor: ["#F97316", "#FB923C", "#FDBA74"],
            borderRadius: 4,
            borderSkipped: false,
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
        scales: {
          x: {
            ticks: { color: "#ffffff" },
            grid: { color: "rgba(255,255,255,0.1)" },
          },
          y: {
            ticks: { color: "#ffffff" },
            grid: { display: false },
          },
        },
      };
    },
  },
};
</script>