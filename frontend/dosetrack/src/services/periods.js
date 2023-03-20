import API_URL from "../settings";

export const periods_by_company = async (company_id) => {
  const res = await fetch(`${API_URL}/period/by_company/${company_id}`);

  if (res.ok) {
    let periods = await res.json();
    return periods;
  } else {
    throw new Error("No se pudo obtener la lista de perídos");
  }
};


export const newPeriod = async (period) => {

  const myHeaders = new Headers({
    "Content-Type": "application/json",
  });

  const fetchConfig = {
    method: "POST",
    headers: myHeaders,
    cache: "no-cache",
  };

  const res = await fetch(
    `${API_URL}/period/`
  );

  const new_period = await res.json();

  if (res.ok) {
    return new_period
  } else {
    return false;
  }
}