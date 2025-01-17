<template>
  <div class="h-200 rounded-xl bg-slate-400">
   <!-- <h2 class="centered">Hello chat</h2> -->
    <div class="content-top h-100 flex justify-center rounded-xl">
     <input class="h-100 w-2/3 bg-slate-400" type="text" placeholder="Search" ref="inputField">
  </div>
    </div>
 </template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue'
const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
const inputField = ref(null)

const focusInput = () => {
  inputField.value?.focus()
}
// Set up event listener
listen('focus-input', () => {
  focusInput()
})

</script>