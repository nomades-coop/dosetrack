<script>
  import { pop, push, replace } from "svelte-spa-router";
  export let params = {};

  const go_back = () => {
    pop();
  };
</script>

<h1>Hello book {params.id}</h1>

<button>Back</button>
<button on:click={go_back}>Home</button>
