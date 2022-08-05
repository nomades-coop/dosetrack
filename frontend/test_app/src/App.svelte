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
  import {getUser, newUser} from  "./services/users";
	import { onMount } from "svelte";
  import Loader from "./components/Loader.svelte";
  import ExternalApi from "./routes/ExternalApi.svelte";
  import { useAuth0 } from "./services/auth0";
  import {auth0_user} from "./store";


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
      dosetrack_user = await getUser($user['email'])
      //dosetrack_user = await getUser('apcenturion@nuclearis.com')
      console.log('Auth0 user', $user)
      auth0_user.update(n => $user);
      if (dosetrack_user) {
        console.log('Dosetrack User', dosetrack_user)
      } else {
        console.log('dosetrack_user:', dosetrack_user)
      }
    } 

  });


	let routes = {
		"/": Dashboard,
		"/book/:id": Book,
		"/faq": Faq,
		"/operators/": Operators,
		"/dosimeters/": Dosimeters,
		"/operator/:company_id/:operator_id": Operator,
		"/dose/registration": DoseRegistration,
		"*": NotFound,
	};


</script>

{#if $isLoading}
	<div class="page-layout">
		<Loader />
	</div>
{/if}

{#if !$isLoading}
<Header/>
<div class="container-fluid">
	<div class="row">
		<NavBar />

		<main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
      {#if $isAuthenticated && dosetrack_user}
			<Router {routes} />
      {:else}
      <AuthenticationButton/>
        {#if $isAuthenticated}
        <Section title="Nueva Empresa">
          <NewCompany/>
        </Section>
        {/if}
      {/if}
		</main>

	</div>
</div>
{/if}