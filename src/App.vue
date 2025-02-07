<template>
  <div @keydown.esc="closeWin" class="min-h-screen bg-gray-900 text-white p-4 mx-auto">
    <!-- Fixed search container -->
    <div class="fixed top-0 left-0 right-0 bg-gray-900 shadow-md p-4 z-10">
      <div class="max-w-7xl mx-auto w-full">
        <input 
        class="bg-gray-900 focus:outline-none rounded-md w-full"
        type="text"
        placeholder="Search"
        ref="inputField"
        @keydown="handleKeyDown"
        v-model="query"
        />
      </div>
    </div>
    <hr class="my-10 h-0.5 border-t-0 bg-gray-600 dark:bg-white/10" />
    <!-- Response container with padding to account for fixed search box -->
    <div class="pt-2">
      <div class="mx-auto space-y-1 ">
        <div 
          v-for="(response, index) in responses" 
          :key="'response-'+index"
          class="bg-gray-800 p-6 rounded-lg shadow-sm border border-gray-400"
        >
        <div 
            class="prose max-w-none text-white" 
            v-html="parseMarkdown(response)"
          >
        </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke, Channel } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ref, watch } from 'vue'
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window'
import { marked } from 'marked';
import DOMPurify from 'dompurify';

const initialHeight = 150 // Height for just the input
const defaultWidth = 1600

const query = ref('')
const responses = ref<string[]>([])
let currentHeight = initialHeight
const lastResponse = ref('')

watch(lastResponse, async (Response: string) => {
  let responseSize  = 0
  console.log("Current Height: "+currentHeight)
  if (Response!=''){
    responseSize = Response.length
  }
  const someResponse = Response!='' || responses.value.length!=0
  console.log('Some resp: '+someResponse)
  const maxSize = 1000
  const additionnalHeight = Math.ceil(responseSize/86)*50+100
  const newHeight = someResponse ? Math.min(currentHeight+additionnalHeight, maxSize) : initialHeight
  // const newHeight = Responses.length!=0 ? Math.min(initialHeight+100+additionnalHeight, maxSize) : initialHeight
  console.log("New height: "+newHeight)
  await getCurrentWindow().setSize(new PhysicalSize(defaultWidth, newHeight))
  currentHeight = newHeight

})

const closeWin = async () =>{
  responses.value = []
  await getCurrentWindow().close();
}

const handleSubmit = (e: Event) => {
  e.preventDefault()
  if (!query.value.trim()) return
  invoke<string>('ask_stream', { input: query.value.trim(), sender:onEvent}).then((message) =>{
    console.log(message);
  });
  // Clear the input
  query.value = ''
  lastResponse.value = ""
  responses.value = responses.value.concat([""])
}
const parseMarkdown = (content:string) => {
  const rawHtml =  marked(content) as string;
  return DOMPurify.sanitize(rawHtml);
};
const onEvent = new Channel<string>();
onEvent.onmessage = (message) => {
  console.log("Received: "+message)
  // if(message == "<answer>"){
  //   responses.value = responses.value.concat([""])
  // }
  // else{
  //   responses.value[responses.value.length-1] = responses.value[responses.value.length-1].concat(message)
  // }
  responses.value[responses.value.length-1] = responses.value[responses.value.length-1].concat(message)
  lastResponse.value = lastResponse.value.concat(message)
};

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Enter') {
    handleSubmit(e)
  }
}

const inputField = ref()

const focusInput = () => {
  inputField.value?.focus()
}
// Set up event listener
listen('focus-input', async () => {
  responses.value = []
  await getCurrentWindow().setSize(new PhysicalSize(defaultWidth, 200))
  // await appWindow.setSize(new PhysicalSize(600, initialHeight))
  focusInput()
})

</script>