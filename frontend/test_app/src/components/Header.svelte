<script>
  import UserSession from "./UserSession.svelte";
  import Icon from "svelte-icons-pack/Icon.svelte";
  import RiSystemMenuFill from "svelte-icons-pack/ri/RiSystemMenuFill";
  export let loggedIn = false;
</script>

<header
  class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow"
  style="background-color: #eeeeee !important"
>
  <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#/dashboard"
    ><img src="./dosetrack-01.svg" alt="" style="height: 2.6em;" /></a
  >
  <button
    class="navbar-toggler position-relative d-md-none collapsed"
    type="button"
    data-bs-toggle="collapse"
    data-bs-target="#sidebarMenu"
    aria-controls="sidebarMenu"
    aria-expanded="false"
    aria-label="Toggle navigation"
  >
    <!-- <span class="navbar-toggler-icon" /> -->
    <span><Icon src={RiSystemMenuFill} size="1.5em" /></span>
  </button>

  <h4 class="me-3 mt-1 d-none d-sm-block" style="color: rgb(94 94 94);;">
    ACME Gammagrafía SA
  </h4>

  {#if loggedIn}
    <!-- <UserSession /> -->
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
