<template>
  <div
    class="flex centered-all rounded-2 p-6"
    style="background-color: rgb(28 25 23)"
  >
    <Line :data="chartData" :options="chartOptions" />
  </div>
</template>

<script>
import { Line } from "vue-chartjs";
import { Chart as ChartJS, LineElement, PointElement, CategoryScale, LinearScale, Tooltip, Legend, Title, Filler } from "chart.js";
ChartJS.register(LineElement, PointElement, CategoryScale, LinearScale, Tooltip, Legend, Title, Filler);

export default {
  name: "LineChart",
  components: { Line },
  props: {
    title: {
      type: String,
      default: () => "Chart title",
    },
    labels: {
      type: Array,
      default: () => ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
    },
    datasets: {
      type: Array,
      default: () => [
        { label: "Accuracy",       data: [40, 55, 30, 70, 60, 85], color: "#F97316" },
        { label: "Completion Rate", data: [60, 50, 75, 65, 80, 70] ,color: "#38BDF8" },
        { label: "Avg Score",       data: [30, 45, 55, 40, 65, 60], color: "#34D399" },
      ],
    },
  },
  computed: {
    chartData() {
      return {
        labels: this.labels,
        datasets: this.datasets.map((ds, i) => {
          return {
            label: ds.label,
            data: ds.data,
            borderColor: ds.color,
            pointRadius: 4,
            tension: 0.4,
            fill: true,
          };
        }),
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
            grid: { color: "rgba(255,255,255,0.1)" },
          },
        },
      };
    },
  },
};
</script>