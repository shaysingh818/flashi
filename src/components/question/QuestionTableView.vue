<template>
  <div class="container centered">
    <table>
      <tbody>
        <tr>
          <th>Type</th>
          <th>Date Updated</th>
          <th>Question</th>
        </tr>
        <tr>
          <td class="tag-column"><Tag title="multiple choice" height="10px" /></td>
          <td class="date-column">04/22/2026</td>
          <td class="content-column">What are the main compounds that make up your pysio</td>
  
        </tr>
        <tr>
          <td class="tag-column"><Tag title="open ended" height="10px" /></td>
          <td class="date-column">10/22/2026</td>
          <td class="content-column">Explain the order in which ATP starts from the brain to the heart. Add sequence diagrams</td>
        </tr>   
        <tr>
          <td class="tag-column"><Tag title="ordered sequence" height="10px" /></td>
          <td class="date-column">10/22/2026</td>
          <td class="content-column">List the parts of the brain that are responsible for feeling temperature and sending action potential to neurons in the nervous system</td>
        </tr>          
      </tbody>
    </table>
  </div>
</template>

<script>
import Tag from '../shared/Tag.vue';

export default {
  name: "QuestionTableView",
  components: {Tag}
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
