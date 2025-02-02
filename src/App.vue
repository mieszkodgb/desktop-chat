<template>
  <div @keydown.esc="closeWin" class="min-h-screen bg-gray-900 text-white p-4">
    <div class="max-w-2xl mx-auto rounded-xl">
      <input class="h-100 w-full bg-slate-900 focus:outline-none"
      type="text" placeholder="Search" ref="inputField"
      @keydown="handleKeyDown" v-model="query"/>
      <!-- <Input /> -->
      <!-- Responses -->
      <div class="my-6">
        <div
          v-if="response!=''"
          :id="response"
          class="bg-gray-800 rounded-lg p-4 transform transition-all duration-200 hover:bg-gray-700"
        >
          {{ response }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ref, watch } from 'vue'
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window'

const initialHeight = 150 // Height for just the input
const defaultWidth = 1600

const query = ref('')
const response = ref<string>('')

// Update window size based on responses
watch(response, async (newResponse) => {
  let responseSize  = 0
  if (newResponse){
    responseSize = newResponse.length
  }
  const maxSize = 1000
  const newHeight = newResponse ? Math.min(initialHeight+100+Math.ceil(responseSize/86)*50, maxSize) : initialHeight
  await getCurrentWindow().setSize(new PhysicalSize(defaultWidth, newHeight))

})

const closeWin = async () =>{
  response.value = ''
  await getCurrentWindow().close();
}

const handleSubmit = (e: Event) => {
  e.preventDefault()
  if (!query.value.trim()) return
  
  invoke<string>('ask', { query: query.value.trim() }).then((message) =>{
    console.log(message);
    response.value = message
  });
  // Clear the input
  query.value = ''
}


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
  response.value = ''
  await getCurrentWindow().setSize(new PhysicalSize(defaultWidth, 200))
  // await appWindow.setSize(new PhysicalSize(600, initialHeight))
  focusInput()
})

</script>