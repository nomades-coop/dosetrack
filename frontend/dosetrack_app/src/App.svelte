<script>
	import Router from "svelte-spa-router";
	import Header from "./components/Header.svelte";
	import NavBar from "./components/NavBar.svelte";
	import Dashboard from "./components/Dashboard.svelte";
	import Home from "./routes/Home.svelte";
	import Faq from "./routes/Faq.svelte";
	import Operator from "./routes/Operator.svelte";
	import NotFound from "./routes/NotFound.svelte";

	let routes = {
		"/": Home,
		"/faq": Faq,
		"/operator/:id": Operator,
		"*": NotFound,
	};

	let person1 = {
		picture: "pict",
		name: "Pablo C",
		dose: 0.54,
	};

	let person2 = {
		picture: "img/algo.jpg",
		name: "Pablo C",
		dose: 0.54,
	};

	let isLoggedIn = true;

	let data = {
		headers: [{ title: "" }, { title: "Name" }, { title: "Dose" }],
		rows: [person1, person2],
	};
</script>

<Router {routes} />
<Header loggedIn={isLoggedIn} />

<div class="container-fluid">
	<div class="row">
		<NavBar />

		<main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
			<Dashboard />
		</main>
	</div>
</div>
