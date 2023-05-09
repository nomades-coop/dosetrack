<script>
  import { onMount } from "svelte";

  import { UserStore } from "../store";
  import FilmsPeriods from "../components/FilmsPeriods.svelte";

  import * as OperatorsService from "../services/operators";

  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};

  UserStore.subscribe((data) => {
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
  });

  let company_id = dosetrack_user.company_id.$oid;

  // get the list of operators operators_by_company asyn function
  let periods_data = [];
  let periods = [];

  onMount(async () => {
    periods_data = async () => {
      return await OperatorsService.getPeriodsData(company_id, null);
    };

    periods = await periods_data();
    console.log(periods);
  });
</script>

{#await periods}
  <div class="spinner-border" role="status">
    <span class="visually-hidden">Loading...</span>
  </div>
{:then periods}
  {#each periods as period}
    <FilmsPeriods {company_id} {period} />
  {/each}
{/await}
