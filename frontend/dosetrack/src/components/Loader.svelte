<script>
  const loadingImg = "https://cdn.auth0.com/blog/hello-auth0/loader.svg";
</script>

<div class="loader">
  <img class="filter-svg" src={loadingImg} alt="Loading..." />
</div>

<style>
  .filter-svg {
    filter: invert(0%) sepia(0%) saturate(20%) hue-rotate(298deg) brightness(83%) contrast(101%);
  }

.loader {
  
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  min-height: 100vh;
	height: 5rem;
	width: 5rem;
  margin: auto;
	animation: spin 2s infinite linear;
}

@keyframes spin {
	from {
		transform: rotate(0deg);
	}

	to {
		transform: rotate(360deg);
	}
}  
</style>