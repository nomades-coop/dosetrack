<script>
  import { useAuth0 } from "../../services/auth0";

  const { login } = useAuth0;
</script>

<button class="btn btn-success btn-lg" on:click={() => login()}>
  Iniciar sesión
</button>
