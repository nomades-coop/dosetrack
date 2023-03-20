
export const getCachedData = async (key, fetchEndpoint, reload = false) => {
  let cache_data = localStorage.getItem(key);

  if (cache_data && !reload) {

    try {
      cache_data = JSON.parse(cache_data);
      if (Math.floor((Date.now() - cache_data['timestamp']) / 1000) < 30) {
        return cache_data['data']
      }
    } catch (e) {
      return null;
    }

  }

  const res = await fetch(fetchEndpoint);

  if (res.ok) {
    let list = await res.json();
    let data = { 'timestamp': Date.now(), 'data': list }
    localStorage.setItem(key, JSON.stringify(data));
    return list;
  } else {
    throw new Error("No se pudo obtener los resultados del fetch.");
  }

}