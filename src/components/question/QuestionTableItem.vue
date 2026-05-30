<template>
  <tr @click="questionDetail">
    <td class="tag-column"><Tag :title="answerTypeText" height="10px" /></td>
    <td class="date-column">{{ props.dateUpdated }}</td>
    <td class="content-column">{{ props.content }}</td>
    <td class="delete-column"> 
        <font-awesome-icon
          @click.stop="deleteQuestion"
          icon="fa-solid fa-x"
          style="color: #f97316"
        />
    </td>
  </tr>
</template>

<script setup>
import { ref } from 'vue';
import Tag from '../shared/Tag.vue';
import { useRouter } from 'vue-router';
import { onMounted } from 'vue';
import { answerTypeStore, questionStore } from '../../api';

const props = defineProps({
  id: {
    type: Number,
    required: true,
  },
  index: {
    type: Number,
    required: true,
  },
  answerTypeId: {
    type: String,
    required: true,
  },
  dateUpdated: {
    type: String,
    required: true,
  },
  content: {
    type: String,
    required: true,
  },
});

const router = useRouter(); 
const answerTypeText = ref('');
const emit = defineEmits(["update:delete"]); 

function questionDetail() {
  questionStore.currQuestion = props.index;
  router.push({ path: `/question/view/${props.id}` });
}

async function deleteQuestion() {
  emit("update:delete", props.id); 
}

onMounted(async () => {
  const answerType = await answerTypeStore.viewAnswerType(`${props.answerTypeId}`);
  answerTypeText.value = answerType.name; 
  console.log(answerTypeText.value);  
})

</script>

<style scoped>

tbody tr {
  border-bottom: 2px solid rgb(41 37 36); 
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
  width: 15%; 
}

.content-column {
  color: #b5b3b3;
  font-size: 10px; 
  text-align: left;
  width: 65%; 
}


.delete-column {
  color: #b5b3b3;
  font-size: 10px; 
  text-align: center;
  width: 10%; 
}

</style>

