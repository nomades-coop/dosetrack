<script>
  export let date = new Date();
  export let dosimeterValue = 0;
  export let filmValue = 0;

  let months = {
    1: "Ene",
    2: "Feb",
    3: "Mar",
    4: "Abr",
    5: "May",
    6: "Jun",
    7: "Jul",
    8: "Ago",
    9: "Sep",
    10: "Oct",
    11: "Nov",
    12: "Dic",
  };

  if (dosimeterValue < 0) dosimeterValue = Math.floor(Math.random() * 201);
  if (filmValue < 0) filmValue = Math.floor(Math.random() * 201);

  let month = date.getMonth() + 1;
  let year = date.getFullYear();

  const monthColor = month % 2 === 0 ? "month-odd" : "month-even";
</script>

<div class="card">
  <table>
    {#if dosimeterValue != 0}
      <tr class="dosimeters"><td>{dosimeterValue}</td></tr>
    {:else}
      <tr><td>&nbsp;</td></tr>
    {/if}

    <tr
      ><td class="card-title {monthColor}"
        ><span class="month">{months[month]}</span><br />{date.getDate()}</td
      ></tr
    >

    {#if filmValue != 0}
      <tr class="films"><td>{filmValue}</td></tr>
    {:else}
      <tr><td>&nbsp;</td></tr>
    {/if}
  </table>
</div>

<style>
  .month {
    font-size: 10pt;
    color: gray;
  }

  .dosimeters {
    background-color: #fff9a8;
  }

  .films {
    background-color: #efe0ff;
  }

  .card .card-title {
    font-family: "Wallpoet", sans-serif;
    font-weight: bold;
    text-align: center;
  }

  .card .card-content {
    text-align: center;
    background-color: #ffc7c7;
  }

  .month-odd {
    background-color: #c7eeff;
  }

  .month-even {
    background-color: #fff;
  }

  .card .card-day {
    max-width: 100%;
    text-align: center;
    /* 			background-color: #c7eeff; */
  }

  table:hover tr td {
    color: var(--dark);
    background: #00ff5a;
  }

  table {
    background-color: #ffffff;
    border: 1px solid white;
    border-spacing: 1px 5px;
    padding: 1px 1px 1px 1px;
  }

  td {
    /* 		padding: 3px; */
    border-radius: 6px;
    text-align: center;
  }

  /* 		@media (min-width: 500px) {
		.card {
			flex-basis: calc(50% - 10px);
		}

		.card:not(:last-child) {
			margin-right: 20px;
		}
	}

	@media (min-width: 700px) {
		.card {
			flex-basis: calc(calc(100% / 3) - 20px);
		}

		.card:not(:last-child) {
			margin-right: 30px;
		}
	}

	@media (min-width: 1100px) {
		.card {
			flex-basis: calc(25% - 30px);
		}

		.card:not(:last-child) {
			margin-right: 40px;
		}
	} */
</style>
