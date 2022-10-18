<script>
  import CodeSnippet from "../components/CodeSnippet.svelte";
  import {
    AccessControlLevel,
    selectedAccessControlLevel,
    apiEndpoint,
    apiResponse,
    getPublicResource,
    getProtectedResource,
    getRbacResource,
    checkCorsAllowedMethod,
  } from "../services/external-api-service";
</script>

<div class="content-layout">
  <h1 class="content__title">External API</h1>
  <div class="content__body">
    <p>
      Use the buttons below to request resources from an API server.
      <br />
      Each API endpoint has a different access control level.
      <br />
      <strong>Only authenticated users can access this page.</strong>
    </p>

    <div class="messages-grid">
      <div class="messages-grid__header">
        API Endpoint by Access Control Level
      </div>
      <div class="messages-grid__options">
        <div
          on:click={getPublicResource}
          class={`messages-grid__option ${
            $selectedAccessControlLevel === AccessControlLevel.PUBLIC &&
            "messages-grid__option--active"
          }`}
        >
          Public
        </div>
        <div
          on:click={getProtectedResource}
          class={`messages-grid__option ${
            $selectedAccessControlLevel === AccessControlLevel.PROTECTED &&
            "messages-grid__option--active"
          }`}
        >
          Protected
        </div>
        <div
          on:click={getRbacResource}
          class={`messages-grid__option ${
            $selectedAccessControlLevel === AccessControlLevel.RBAC &&
            "messages-grid__option--active"
          }`}
        >
          RBAC
        </div>
        <div
          on:click={checkCorsAllowedMethod}
          class={`messages-grid__option ${
            $selectedAccessControlLevel === AccessControlLevel.CORS &&
            "messages-grid__option--active"
          }`}
        >
          CORS
        </div>
      </div>
      <CodeSnippet title={$apiEndpoint} code={$apiResponse} />
    </div>
  </div>
</div>
