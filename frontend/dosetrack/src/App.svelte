<script>
  import Router from "svelte-spa-router";
  import Book from "./routes/Book.svelte";
  import NotFound from "./routes/NotFound.svelte";

  import Header from "./components/Header.svelte";
  import NavBar from "./components/NavBar.svelte";
  import Dashboard from "./routes/Dashboard.svelte";
  import Faq from "./routes/Faq.svelte";
  import Operator from "./routes/Operator.svelte";
  import Operators from "./routes/Operators.svelte";
  import Dosimeters from "./routes/Dosimeters.svelte";
  import DoseRegistration from "./routes/DoseRegistration.svelte";
  import AuthenticationButton from "./components/auth/authentication-button.svelte";
  import Section from "./components/Section.svelte";
  import NewCompany from "./components/NewCompany.svelte";
  import { getUser, newUser } from "./services/users";
  import { onMount } from "svelte";
  import Loader from "./components/Loader.svelte";
  import { useAuth0 } from "./services/auth0";
  import { UserStore } from "./store";
  import Users from "./routes/Users.svelte";
  import API_URL from "./settings";
  import FormError from "./components/FormError.svelte";
  import Films from "./routes/Films.svelte";

  import { SvelteToast } from "@zerodevx/svelte-toast";
  const toast_options = {};

  let {
    auth0Client,
    isLoading,
    isAuthenticated,
    user,
    login,
    initializeAuth0,
    createAuth0Client,
  } = useAuth0;

  let dosetrack_user = null;
  let dosetrack_operator = null;

  const authenticationGuard = (ctx, next) => {
    if ($isAuthenticated) {
      next();
    } else {
      login({ appState: { targetUrl: ctx.pathname } });
    }
  };

  const onRedirectCallback = (appState) => {
    window.history.replaceState(
      {},
      document.title,
      appState && appState.targetUrl
        ? appState.targetUrl
        : window.location.pathname
    );
  };

  onMount(async () => {
    await initializeAuth0({ onRedirectCallback });

    if ($isAuthenticated) {
      dosetrack_user = await getUser($user["email"]);
      dosetrack_operator = window.localStorage.getItem("operator");

      let u_data = {
        Auth0: $user,
        Dosetrack: dosetrack_user,
      };
      UserStore.update((n) => u_data);

      if (dosetrack_user || dosetrack_operator) {
        // console.log('Dosetrack User', dosetrack_user)
      } else {
        let registration_form = document.getElementById("registration_form");
        registration_form.classList.remove("visually-hidden");
        console.log(
          `Dosetrack database does not have a user with this email ${$user["email"]}`
        );
      }
    }

    registration_form.classList.remove("visually-hidden");
  });

  let routes = {
    "/": Dashboard,
    "/book/:id": Book,
    "/faq": Faq,
    "/operators/": Operators,
    "/dosimeters/": Dosimeters,
    "/films/": Films,
    "/operator/:company_id/:operator_id": Operator,
    "/dose/registration": DoseRegistration,
    "/users": Users,
    "*": NotFound,
  };

  let errors = {};

  const setOperator = async () => {
    let operator_id = document.getElementById("token").value;
    console.log("setOperator", operator_id);

    const res = await fetch(`${API_URL}/operator/${operator_id}`);

    if (res.ok) {
      let operator = await res.json();
      window.localStorage.setItem("operator", JSON.stringify(operator));
    } else {
      errors["token"] = { error: [] };
      errors["token"]["error"].push(
        "El token es inválido, por favor verifíquelo."
      );
      console.log(errors);
    }
  };
</script>

{#if $isLoading}
  <div class="page-layout">
    <Loader />
  </div>
{/if}

{#if !$isLoading}
  <SvelteToast options={toast_options} />
  <Header />
  <div class="container-fluid">
    <div class="row">
      <NavBar />

      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        {#if $isAuthenticated && (dosetrack_user || dosetrack_operator)}
          <Router {routes} />
        {:else}
          <div id="registration_form" class="visually-hidden">
            {#if $isAuthenticated}
              <Section
                title="Bienvenido {$user.name}"
                subtitle="Registrarme como operador"
              >
                <div class="mb-3">
                  <label for="name" class="form-label"
                    >Por favor ingrese su token de indentificación</label
                  >
                  <input
                    type="text"
                    class="form-control"
                    name="token"
                    id="token"
                    aria-describedby="nameHelp"
                  />
                  <div id="nameHelp" class="form-text">
                    El token se lo debe proporcionar su empleador.
                  </div>
                  <FormError err={errors.token} />
                </div>

                <div class="text-center">
                  <button on:click={setOperator} class="btn btn-primary btn-lg"
                    >Ingresar como operador</button
                  >
                </div>
              </Section>
              <Section
                title="Nueva Empresa"
                subtitle="Registrar mi empresa y mi usuario como administrador"
              >
                <NewCompany />
              </Section>
            {/if}
            <div class="row p-5">
              <AuthenticationButton />
            </div>
          </div>
        {/if}
      </main>
    </div>
  </div>
{/if}

<style>
  :root {
    --toastBorderRadius: 0.5rem;
    --toastBorder: 1px solid #000;
    --toastColor: #000;
    --toastBackground: #f4f1f1;
    --toastContainerRight: 1rem;
  }
</style>
