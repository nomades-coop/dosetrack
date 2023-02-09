import API_URL from "../settings";

export const getFilmsByCompany = async (company_id) => {

  let cache_data = localStorage.getItem('dosetrack.filmslist'); 
  
  if (cache_data) {

    try {
      cache_data = JSON.parse(cache_data);
      if (Math.floor((Date.now() - cache_data['timestamp']) /1000) < 30) {
        return cache_data['data']
      }
    } catch (e) {
      cache_data = null;
    }

  }
  
  let resFilms = await fetch(`${API_URL}/film/by_company/${company_id}`);
    
  if (resFilms.ok) {
    let films = await resFilms.json();
    let data = {'timestamp': Date.now(), 'data': films}
    localStorage.setItem('dosetrack.filmslist', JSON.stringify(data));
    return await films;
  } else {
    throw new Error("No se pudo obtener la lista de films");
  }
  
}

  