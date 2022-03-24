<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { open, save } from '@tauri-apps/api/dialog'
  import { onMount } from 'svelte'
  import { browser } from '$app/env'

  import { FileDrop } from 'svelte-tauri-filedrop'

  let files = {
    accepted: [],
    rejected: [],
  }
  async function handleClick(e) {
    let filepath = await open()
    console.log(filepath)
    invoke('update_system_data', { data: filepath })
      .then((m) => console.log(m))
      .catch((error) => console.error(error))
  }

  function openHandler(paths: string[]) {
    console.log(paths)
  }
</script>

<div class="grid mx-20 mt-6">
  <h1 class="text-2xl font-medium leading-tight mt-0 mb-2">Dashboard</h1>
</div>

<FileDrop extensions={['json']} handleFiles={openHandler} let:files>
  <div class="dropzone" class:droppable={files.length > 0}>
    <h2>Drop JSON files</h2>
  </div>
</FileDrop>

<div class="flex space-x-2 justify-center">
  <button
    on:click={handleClick}
    type="button"
    class="inline-block px-6 py-2.5 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
    >Button</button
  >
</div>
