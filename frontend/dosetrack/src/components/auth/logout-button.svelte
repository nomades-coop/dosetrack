<script>
  import { useAuth0 } from "../../services/auth0";

  const { user, logout } = useAuth0;
  let userinfo = $user;
</script>

<center>
  <button
    class="btn btn-danger"
    on:click={() =>
      logout({
        returnTo: window.location.origin,
      })}
  >
    Cerrar sesión
  </button>
</center>
