<script>
  import UserSession from "./UserSession.svelte";
  export let loggedIn = false;
</script>

<header class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
  <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="?">DOSETRACK</a>
  <button
    class="navbar-toggler position-relative d-md-none collapsed"
    type="button"
    data-bs-toggle="collapse"
    data-bs-target="#sidebarMenu"
    aria-controls="sidebarMenu"
    aria-expanded="false"
    aria-label="Toggle navigation"
  >
    <span class="navbar-toggler-icon" />
  </button>

  {#if loggedIn}
    <UserSession />
  {:else}
    <div class="navbar-nav">
      <div class="nav-item text-nowrap">
        <a class="nav-link px-3" href="?">Sign out</a>
      </div>
    </div>
  {/if}

  <!-- <input
    class="form-control form-control-dark w-100"
    type="text"
    placeholder="Search"
    aria-label="Search"
  /> -->
</header>
