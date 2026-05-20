<template>
  <div class="flex centered-all w-100">
    <table>
      <tbody>
        <tr>
          <th>Type</th>
          <th>Date Updated</th>
          <th>Question</th>
        </tr>

        <QuestionTableItem
          v-for="(item, index) in questionStore.questions"
          :key="index"
          :id="item.id"
          :answerTypeId="`${item.answer_type_id}`"
          :dateUpdated="item.updated"
          :content="item.raw"
        />
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { questionStore } from '../../api/Question';
import Tag from '../shared/Tag.vue';
import QuestionTableItem from './QuestionTableItem.vue';

const props = defineProps({
  deckId: {
    type: String,
    required: true,
  }
})

onMounted(async () => {
  await questionStore.loadQuestions(`${props.deckId}`);
})


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
