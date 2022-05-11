<script>
  import Icon from "svelte-icons-pack/Icon.svelte";
  import FaSolidHome from "svelte-icons-pack/fa/FaSolidHome";
  import FaSolidUsers from "svelte-icons-pack/fa/FaSolidUsers";
  import FaBrandsReact from "svelte-icons-pack/fa/FaBrandsReact";
  import FaSolidBookDead from "svelte-icons-pack/fa/FaSolidBookDead";

  const toggleMenu = () => {
    let menu = document.getElementById("sidebarMenu");
    if (menu.classList.contains("show")) {
      menu.classList.remove("show");
    } else {
      menu.classList.add("show");
    }
  };
</script>

<nav
  id="sidebarMenu"
  class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse"
>
  <div class="position-sticky pt-3">
    <ul class="nav flex-column">
      <li class="nav-item">
        <a
          class="nav-link active"
          aria-current="page"
          on:click={toggleMenu}
          href="#/"
        >
          <Icon src={FaSolidHome} color="gray" className="iconito-menu" />
          Dashboard
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="#/operators" on:click={toggleMenu}>
          <Icon src={FaSolidUsers} color="gray" className="iconito-menu" />
          Operadores
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="#/dosimeters" on:click={toggleMenu}>
          <Icon src={FaBrandsReact} color="gray" className="iconito-menu" />
          Dosímetros
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="#/reports" on:click={toggleMenu}>
          <!-- <span data-feather="bar-chart-2" /> -->
          <Icon src={FaSolidBookDead} color="gray" className="iconito-menu" />
          Informes
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="#/dose/registration" on:click={toggleMenu}>
          <!-- <span data-feather="bar-chart-2" /> -->
          <Icon src={FaSolidBookDead} color="gray" className="iconito-menu" />
          Registro de Dosis
        </a>
      </li>
    </ul>

    <!-- <h6
      class="sidebar-heading d-flex justify-content-between align-items-center px-3 mt-4 mb-1 text-muted"
    >
      <span>Saved reports</span>
      <a class="link-secondary" href="?" aria-label="Add a new report">
        <span data-feather="plus-circle" />
      </a>
    </h6>
    <ul class="nav flex-column mb-2">
      <li class="nav-item">
        <a class="nav-link" href="?">
          <span data-feather="file-text" />
          Current month
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="?">
          <span data-feather="file-text" />
          Last quarter
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="?">
          <span data-feather="file-text" />
          Social engagement
        </a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="?">
          <span data-feather="file-text" />
          Year-end sale
        </a>
      </li>
    </ul> -->
  </div>
</nav>

<style>
</style>
