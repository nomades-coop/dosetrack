import API_URL from "../settings";

export const periods_by_company = async (company_id) =>
{
  const res = await fetch(`${API_URL}/period/by_company/${company_id}`);
    
  if (res.ok) {
    let periods = await res.json();
    return periods;
  } else {
    throw new Error("No se pudo obtener la lista de perídos");
  }
};


