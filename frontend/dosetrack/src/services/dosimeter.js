import API_URL from "../settings";
import { getCachedData } from "./cache";


export const getDosimetersByCompany = async (company_id) => {

  let dosimeters = await getCachedData('dosetrack.dosimeters_by_company', `${API_URL}/operators/${company_id}`);

  return dosimeters;

  let cache_data = getCachedData('dosetrack.dosimeters_by_company', `${API_URL}/dosimeters/${company_id}`); 
  
  if (cache_data) {

  }

  const res = await fetch(`${API_URL}/dosimeters/${company_id}`);
      
  if (res.ok) {
    let list = await res.json();
    return list;
  } else {
    throw new Error("No se pudo obtener la lista de dosimetros");
  }
}


