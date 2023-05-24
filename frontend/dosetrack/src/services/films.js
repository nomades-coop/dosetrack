import API_URL from "../settings";

export const getFilmsByCompany = async (company_id, period = null) => {

  let cache_data = localStorage.getItem('dosetrack.filmslist');

  if (cache_data) {

    try {
      cache_data = JSON.parse(cache_data);
      if (Math.floor((Date.now() - cache_data['timestamp']) / 1000) < 5) {
        return cache_data['data']
      }
    } catch (e) {
      cache_data = null;
    }

  }

  let period_query = period ? `?period=${period}` : '';
  let resFilms = await fetch(`${API_URL}/film/by_company/${company_id}${period_query}`);

  if (resFilms.ok) {
    let films = await resFilms.json();
    let data = { 'timestamp': Date.now(), 'data': films }
    localStorage.setItem('dosetrack.filmslist', JSON.stringify(data));
    return await films;
  } else {
    throw new Error("No se pudo obtener la lista de films");
  }

}

export const saveFilmDose = async (film_dose = null) => {

  let response

  try {

    if (film_dose._id == "") {
      delete film_dose._id
    }

    response = await fetch(`${API_URL}/film_dose`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(film_dose),
    });

  } catch { return { 'error': 'No se pudo guardar la dosis', 'operator_id': film_dose.operator_id } }

  if (response.ok) {
    return await response.json();
  } else {
    return { 'error': 'No se pudo guardar la dosis', 'operator_id': film_dose.operator_id }
  }

}

