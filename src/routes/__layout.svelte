<script>
  import '../app.css'

  import Navbar from '$lib/Navbar.svelte'
</script>

<div class="h-screen w-screen grid">
  <Navbar />
  <main>
    <slot />
  </main>
</div>

<style>
  div {
    grid-template-rows: auto 1fr;
  }
</style>
