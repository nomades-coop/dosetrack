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

	let routes = {
		"/": Dashboard,
		"/book/:id": Book,
		"/faq": Faq,
		"/operators/": Operators,
		"/dosimeters/": Dosimeters,
		"/operator/:id": Operator,
		"/dose/registration": DoseRegistration,
		"*": NotFound,
	};

	let isloggedIn = true;
</script>

<Header loggedIn={isloggedIn} />

<div class="container-fluid">
	<div class="row">
		<NavBar />

		<main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
			<Router {routes} />
		</main>
	</div>
</div>
