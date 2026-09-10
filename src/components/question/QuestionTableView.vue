<template>
  <div class="flex items-center justify-center">
    <table>
      <tbody>
        <tr>
          <th>Type</th>
          <th>Date Updated</th>
          <th>Question</th>
          <th>Delete</th>
        </tr>

        <QuestionTableItem
          v-for="(item, index) in questionStore.questions"
          :key="index"
          :id="item.id"
          :index="index"
          :answerTypeId="`${item.answer_type_id}`"
          :dateUpdated="item.updated"
          :content="item.raw"
          @update:delete="handleDelete($event)"
        />
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { message } from '@tauri-apps/plugin-dialog';
import { questionStore } from '../../api/Question';
import Tag from '../shared/Tag.vue';
import QuestionTableItem from './QuestionTableItem.vue';

const props = defineProps({
  deckId: {
    type: String,
    required: true,
  }
})


async function handleDelete(id) {
  const deleteResult = await questionStore.deleteQuestion(`${id}`);
  if (deleteResult === "Success") {    
    await message(
      'Successfully deleted question from deck',
      { title: 'Recalli', kind: 'info' }
    );
    await questionStore.loadQuestions(`${props.deckId}`);
  } else {    
    await message(
      'Could not delete question from deck. This question is currently being used in a session. You must remove the session that has this question before deleting it.',
      { title: 'Recalli', kind: 'info' }
    );
  }
}

onMounted(async () => {
  const result = await questionStore.loadQuestions(`${props.deckId}`);
  console.log("THIS IS THE RESULT: ", questionStore.questions); 
  questionStore.currQuestion = 0;
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
