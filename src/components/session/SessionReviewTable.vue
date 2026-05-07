<template>
  <div class="flex centered-all w-100">
    <table>
      <tbody>
        <tr>
          <th>Date Started</th>
          <th>Date Completed</th>
          <th>Questions Reviewed</th>
          <th>Score</th>
        </tr>
        <tr @click="questionDetail">
          <td class="date-column">04/22/2026</td>
          <td class="date-column">04/22/2026</td>
          <td class="date-column">63</td>
          <td class="tag-column"><Tag title="78%" height="10px" /></td>
        </tr>
        <tr>
          <td class="date-column">04/22/2026</td>
          <td class="date-column">04/22/2026</td>
          <td class="date-column">63</td>
          <td class="tag-column"><Tag title="78%" height="10px" /></td>
        </tr>   
        <tr>
          <td class="date-column">04/22/2026</td>
          <td class="date-column">04/22/2026</td>
          <td class="date-column">63</td>
          <td class="tag-column"><Tag title="78%" height="10px" /></td>
        </tr>          
      </tbody>
    </table>
  </div>
</template>

<script>
import Tag from '../shared/Tag.vue';

export default {
  name: "SessionReviewTable",
  components: {Tag},
  methods: {
    questionDetail() {
      this.$router.push({ path: "/question/view" });
    }
  }
};
</script>

<style scoped>

table {
  width: 90%;
  background-color: rgb(28 25 23);
  border-radius: 10px;
  border-collapse: collapse; /* 👈 add this */
}

tbody tr {
  border-bottom: 2px solid rgb(41 37 36); 
}

th {
  color: white;
  padding: 10px;
  font-weight: normal; 
}

td {
  padding: 10px;
}

.tag-column {
  width: 10%; 
}

.date-column {
  color: #F97316;
  font-size: 10px; 
  text-align: center;
  width: 25%; 
}

.content-column {
  color: #b5b3b3;
  font-size: 10px; 
  text-align: left;
  width: 65%; 
}

</style>
