<template>
  <div 
    class="flex centered-all p-6 rounded-2"
    style="background-color: rgb(28 25 23);">
    <Doughnut :data="chartData" :options="chartOptions" />
  </div>
</template>

<script>
import { Doughnut } from 'vue-chartjs';
import { Chart as ChartJS, ArcElement, Tooltip, Legend, Title } from 'chart.js';

ChartJS.register(ArcElement, Tooltip, Legend, Title);

export default {
  name: "DoughnutChart",
  components: { Doughnut },
  props: {
    title: {
      type: String,
      default: () => "Chart title",
    },
    labels: {
      type: Array,
      default: () => ['Correct', 'Incorrect'],
    },
    values: {
      type: Array,
      default: () => [45, 55],
    },
  },
  computed: {
    chartData() {
      return {
        label: "Chart title",
        labels: this.labels,
        datasets: [
          {
            label: "Accuracy of questions",
            data: this.values,
            backgroundColor: ['#F97316', '#3D3D3D'],
            borderWidth: 0,
          },
        ],
      };
    },
    chartOptions() {
      return {
        reponsive: true,
        plugins: {
          title: {
            display: true,
            text: this.title,
            color: '#ffffff',
            font: {
              size: 16,
              weight: 'bold',
            },
            padding: {
              bottom: 10,
            },
          },
          legend: {
            position: 'bottom',
          },
        },
      };
    },
  }
};

</script>