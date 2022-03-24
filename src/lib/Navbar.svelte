<script lang="ts">
  import Icon from 'svelte-awesome'
  import { faGithub } from '@fortawesome/free-brands-svg-icons'
</script>

<nav
  class="relative w-full flex flex-wrap items-center justify-between py-3 bg-gray-900 text-gray-200 shadow-lg navbar navbar-expand-lg navbar-light"
>
  <div class="container-fluid w-full flex flex-wrap items-center justify-between px-6">
    <a class="text-xl text-white pr-2 font-semibold" href="#">Inertia Monitoring</a>
    <!-- Left links -->
    <ul class="navbar-nav flex flex-col pl-0 list-style-none mr-auto">
      <li class="nav-item p-2">
        <a class="nav-link text-white" href="#">Dashboard</a>
      </li>
    </ul>
    <!-- Left links -->

    <!-- Right elements -->
    <div class="flex items-center relative">
      <!-- Icon -->
      <a
        class="text-white opacity-60 hover:opacity-80 focus:opacity-80 mr-4"
        href="https://github.com/G-PST/inertia-monitoring-prototype"
      >
        <Icon data={faGithub} />
      </a>
    </div>
    <!-- Right elements -->
  </div>
</nav>
