<template>
  <div class="flex justify-center question-box p-2 w-full">
    <p class="flex flex-col items-left p-4" v-html="renderedText"> </p>
    
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
console.log("RENDERED TEXT: ", renderedText.value); 
</script>

<style scoped>

.question-box {
  background-color: var(--app-color-surface);
  margin: 20px;
  border-radius: 10px;
}

.question-box :deep(h1) {
  font-size: 1.6rem;
  font-weight: 700;
  margin-bottom: 0.5rem;
}

.question-box :deep(h2) {
  font-size: 1.3rem;
  font-weight: 700;
  margin-bottom: 0.5rem;
}

.question-box :deep(h3) {
  font-size: 1.1rem;
  font-weight: 700;
  margin-bottom: 0.5rem;
}

.question-box :deep(p) {
  font-weight: normal;
  font-size: 13px;
  margin-bottom: 1rem;
}

.question-box :deep(li) {
  font-weight: normal;
  font-size: 13px;
  margin-left: 1.25rem;
}

.question-box :deep(ul) {
  list-style: disc;
  margin-bottom: 1rem;
}

.question-box :deep(ol) {
  list-style: decimal;
  margin-bottom: 1rem;
}

.question-box :deep(code) {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 0.85em;
}

.question-box :deep(pre) {
  overflow-x: auto;
  padding: 0.75rem;
  border-radius: 0.5rem;
  margin-bottom: 1rem;
}

.question-box :deep(blockquote) {
  border-left: 3px solid var(--app-color-accent);
  padding-left: 0.75rem;
  margin: 0 0 1rem 0;
  color: var(--app-color-text-muted);
}

p {
  color: #b5b3b3; 
  font-size: 15px; 
}

</style>
