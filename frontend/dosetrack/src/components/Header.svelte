<script>
  import { useAuth0 } from "../services/auth0";
  import Icon from "svelte-icons-pack/Icon.svelte";
  import RiSystemMenuFill from "svelte-icons-pack/ri/RiSystemMenuFill";
  const { user, isAuthenticated } = useAuth0;
  
</script>

<header
  class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow-sm"
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

  {#if $isAuthenticated}
  <h4 class="me-3 mt-1 d-none d-sm-block" style="color: rgb(94 94 94);">
    {$user.given_name} {$user.family_name}
  </h4>
  {/if}

  <!-- <input
    class="form-control form-control-dark w-100"
    type="text"
    placeholder="Search"
    aria-label="Search"
  /> -->
</header>
