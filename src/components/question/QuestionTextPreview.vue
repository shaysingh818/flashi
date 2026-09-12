<template>
  <div class="flex justify-center question-box p-2 w-full">
    <p class="flex col items-left p-4" v-html="renderedText"> </p>
  </div>

</template>

<script setup>
import { computed } from 'vue';
import { marked } from 'marked';
import markedKatex from 'marked-katex-extension';
import { markedHighlight } from 'marked-highlight';
import hljs from 'highlight.js';
import 'katex/dist/katex.min.css';
import 'highlight.js/styles/github-dark.css'; 

const props = defineProps({  
  text: {
    type: String,
    required: true,
  },
})

marked.use(markedKatex({ throwOnError: false }));
marked.use(markedHighlight({
  langPrefix: 'hljs language-',
  highlight(code, lang) {
    const language = hljs.getLanguage(lang) ? lang : 'plaintext';
    return hljs.highlight(code, { language }).value;
  }
}));


const renderedText = computed(() => marked(props.text))
</script>

<style scoped>

.question-box {
  background-color: var(--app-color-surface);
  margin: 20px;
  border-radius: 10px;
}


.question-box :deep(h1) {
  margin-bottom: 0rem;
}

.question-box :deep(h2) {
  margin-bottom: 0rem;
}

.question-box :deep(p) {
  font-weight: normal;
  font-size: 13px; 
  margin-bottom: 1rem; 
}

.question-box :deep(li) {
  font-weight: normal;
  font-size: 13px; 
}

.question-box :deep(ul) {
  margin-bottom: 1rem; 
}

p {
  color: #b5b3b3; 
  font-size: 15px; 
}

</style>
