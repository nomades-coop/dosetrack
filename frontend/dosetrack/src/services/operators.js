import API_URL from "../settings";
import { getCachedData } from "./cache";

export const operators_by_company = async (company_id, reload = false) => {

  let operators = await getCachedData('dosetrack.operators_by_company', `${API_URL}/operators/${company_id}`, reload);

  return operators;


};


