<template>
  <div @keydown.esc="closeWin" class="min-h-screen bg-gray-900 text-white p-4">
    <div class="max-w-2xl mx-auto rounded-xl">
        <input class="fixed h-100 w-full bg-slate-900 focus:outline-none"
        type="text" placeholder="Search" ref="inputField"
        @keydown="handleKeyDown" v-model="query"/>
      <!-- <Input /> -->
      <!-- Responses -->
      
        <div class="my-6 fixed">
          <!-- <div v-if="responses.length!=0"> -->
            <div v-for="(response, index) in responses"
            :id="'response-'+index" class="bg-gray-800 rounded-lg p-4 flex flex-col overflow-y-auto transform transition-all duration-200 hover:bg-gray-700 max-h-80">
              {{ response }}
            
            </div>
          <!-- </div> -->
          
          <!-- <div
            v-if="response!=''"
            :id="response"
            class="bg-gray-800 rounded-lg p-4 flex flex-col overflow-y-auto transform transition-all duration-200 hover:bg-gray-700 max-h-80"
          >
          {{ response }}
          </div> -->

        </div>
      
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke, Channel } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ref, watch } from 'vue'
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window'

const initialHeight = 150 // Height for just the input
const defaultWidth = 1600

const query = ref('')
const responses = ref<string[]>([])
let currentHeight = initialHeight
const lastResponse = ref('')
// const lastResponse = ref<string>('')

// watch(responses, async (Responses: string[]) => {
//   let responseSize  = 0
//   console.log("Watch responses: "+responses.value)
//   if (Responses.length!=0){
//     responseSize = Responses[Responses.length - 1].length
//   }
//   const maxSize = 1000
//   const additionnalHeight = Math.ceil(responseSize/86)*50+100
//   const newHeight = Responses.length!=0 ? Math.min(currentHeight+additionnalHeight, maxSize) : initialHeight
//   // const newHeight = Responses.length!=0 ? Math.min(initialHeight+100+additionnalHeight, maxSize) : initialHeight
//   await getCurrentWindow().setSize(new PhysicalSize(defaultWidth, newHeight))
//   currentHeight = newHeight
//   console.log(currentHeight)

// })
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

const inputField = ref(null)

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