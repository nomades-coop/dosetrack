import API_URL from "../settings";
import { getCachedData } from "./cache";

export const getByCompany = async (company_id, reload = false) => {

  let operators = await getCachedData('dosetrack.operators_by_company', `${API_URL}/operators/${company_id}`, reload);

  return operators;


};

export const getPeriodsData = async (company_id, period_id, reload = false) => {

  let periods = await getCachedData('dosetrack.operators_by_company', `${API_URL}/operators/doses/${company_id}/${period_id}`, reload);

  return await periods;

};

export const getOverdoses = async (company_id, period_id, reload = false) => {
  
    let overdoses = await getCachedData('dosetrack.operators_overdoses', `${API_URL}/operators/overdose/${company_id}/${period_id}`, reload);
  
    return overdoses;
  
  }